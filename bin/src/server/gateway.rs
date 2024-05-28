use std::{sync::Arc, time::Duration};

use atm0s_sdn::{features::FeaturesEvent, secure::StaticKeyAuthorization, services::visualization, SdnBuilder, SdnControllerUtils, SdnExtOut, SdnOwner};
use clap::Parser;
use media_server_gateway::{store_service::GatewayStoreServiceBuilder, STORE_SERVICE_ID};
use media_server_protocol::{
    gateway::{generate_gateway_zone_tag, GATEWAY_RPC_PORT},
    protobuf::cluster_gateway::{MediaEdgeServiceClient, MediaEdgeServiceServer},
    rpc::quinn::{QuinnClient, QuinnServer},
};
use media_server_secure::jwt::{MediaEdgeSecureJwt, MediaGatewaySecureJwt};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};

use crate::{
    http::run_gateway_http_server,
    quinn::{make_quinn_client, make_quinn_server, VirtualNetwork},
    NodeConfig,
};
use sans_io_runtime::{backend::PollingBackend, ErrorDebugger2};

use self::{dest_selector::build_dest_selector, ip_location::Ip2Location, local_rpc_handler::MediaLocalRpcHandler};

mod dest_selector;
mod ip_location;
mod local_rpc_handler;
mod remote_rpc_handler;

#[derive(Clone, Debug, convert_enum::From, convert_enum::TryInto)]
enum SC {
    Visual(visualization::Control),
    Gateway(media_server_gateway::store_service::Control),
}

#[derive(Clone, Debug, convert_enum::From, convert_enum::TryInto)]
enum SE {
    Visual(visualization::Event),
    Gateway(media_server_gateway::store_service::Event),
}
type TC = ();
type TW = ();

#[derive(Debug, Parser)]
pub struct Args {
    /// Location latude
    #[arg(env, long, default_value_t = 0.0)]
    lat: f32,

    /// Location longtude
    #[arg(env, long, default_value_t = 0.0)]
    lon: f32,

    /// GeoIp database
    #[arg(env, long, default_value = "./maxminddb-data/GeoLite2-City.mmdb")]
    geo_db: String,
}

pub async fn run_media_gateway(workers: usize, http_port: Option<u16>, node: NodeConfig, args: Args) {
    rustls::crypto::ring::default_provider().install_default().expect("should install ring as default");

    let default_cluster_cert_buf = include_bytes!("../../certs/cluster.cert");
    let default_cluster_key_buf = include_bytes!("../../certs/cluster.key");
    let default_cluster_cert = CertificateDer::from(default_cluster_cert_buf.to_vec());
    let default_cluster_key = PrivatePkcs8KeyDer::from(default_cluster_key_buf.to_vec());

    let edge_secure = Arc::new(MediaEdgeSecureJwt::from(node.secret.as_bytes()));
    let gateway_secure = Arc::new(MediaGatewaySecureJwt::from(node.secret.as_bytes()));
    let (req_tx, mut req_rx) = tokio::sync::mpsc::channel(1024);
    if let Some(http_port) = http_port {
        tokio::spawn(async move {
            if let Err(e) = run_gateway_http_server(http_port, req_tx, edge_secure, gateway_secure).await {
                log::error!("HTTP Error: {}", e);
            }
        });
    }

    let node_id = node.node_id;

    let mut builder = SdnBuilder::<(), SC, SE, TC, TW>::new(node_id, node.udp_port, node.custom_addrs);

    builder.set_authorization(StaticKeyAuthorization::new(&node.secret));
    builder.set_manual_discovery(vec!["gateway".to_string(), generate_gateway_zone_tag(node.zone)], vec!["gateway".to_string()]);
    builder.add_service(Arc::new(GatewayStoreServiceBuilder::new(node.zone, args.lat, args.lon)));

    for seed in node.seeds {
        builder.add_seed(seed);
    }

    let mut controller = builder.build::<PollingBackend<SdnOwner, 128, 128>>(workers);
    let (selector, mut requester) = build_dest_selector();

    // Ip location for routing client to closest gateway
    let ip2location = Arc::new(Ip2Location::new(&args.geo_db));

    //
    // Vnet is a virtual udp layer for creating RPC handlers, we separate media server to 2 layer
    // - async for business logic like proxy, logging handling
    // - sync with sans-io style for media data
    //
    let (mut vnet, vnet_tx, mut vnet_rx) = VirtualNetwork::new(node.node_id);

    let media_rpc_socket = vnet.udp_socket(0).await.expect("Should open virtual port for gateway rpc");
    let media_rpc_client = MediaEdgeServiceClient::new(QuinnClient::new(make_quinn_client(media_rpc_socket, &[]).expect("Should create endpoint for media rpc client")));

    let media_rpc_socket = vnet.udp_socket(GATEWAY_RPC_PORT).await.expect("Should open virtual port for gateway rpc");
    let mut media_rpc_server = MediaEdgeServiceServer::new(
        QuinnServer::new(make_quinn_server(media_rpc_socket, default_cluster_key, default_cluster_cert.clone()).expect("Should create endpoint for media rpc server")),
        remote_rpc_handler::Ctx {
            selector: selector.clone(),
            client: media_rpc_client.clone(),
            ip2location: ip2location.clone(),
        },
        remote_rpc_handler::MediaRemoteRpcHandlerImpl::default(),
    );

    let local_rpc_processor = Arc::new(MediaLocalRpcHandler::new(selector, media_rpc_client, ip2location));

    tokio::task::spawn_local(async move {
        media_rpc_server.run().await;
    });

    tokio::task::spawn_local(async move { while vnet.recv().await.is_some() {} });

    loop {
        if controller.process().is_none() {
            break;
        }
        while let Ok(control) = vnet_rx.try_recv() {
            controller.feature_control((), control.into());
        }
        while let Some(out) = requester.recv() {
            controller.service_control(STORE_SERVICE_ID.into(), (), out.into());
        }
        while let Ok(req) = req_rx.try_recv() {
            let res_tx = req.answer_tx;
            let param = req.req;
            let conn_part = param.get_conn_part();
            let local_rpc_processor = local_rpc_processor.clone();

            tokio::spawn(async move {
                let res = local_rpc_processor.process_req(conn_part, param).await;
                res_tx.send(res).print_err2("answer http request error");
            });
        }

        while let Some(out) = controller.pop_event() {
            match out {
                SdnExtOut::ServicesEvent(_, _, SE::Gateway(event)) => {
                    requester.on_event(event);
                }
                SdnExtOut::FeaturesEvent(_, FeaturesEvent::Socket(event)) => {
                    if let Err(e) = vnet_tx.try_send(event) {
                        log::error!("[MediaEdge] forward Sdn SocketEvent error {:?}", e);
                    }
                }
                _ => {}
            }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
