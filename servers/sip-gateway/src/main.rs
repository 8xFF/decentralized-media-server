use clap::Parser;
use cluster_local::ServerLocal;
use cluster_sdn::{NodeAddr, NodeId, ServerAtm0s, ServerAtm0sConfig};
use std::net::SocketAddr;

mod server;
mod sip_in_session;
mod sip_out_session;

/// Media Server node
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Current Node ID
    #[arg(env, long)]
    node_id: Option<NodeId>,

    /// Neighbors
    #[arg(env, long)]
    neighbours: Vec<NodeAddr>,

    /// Sip listen socket
    #[arg(env, long, default_value = "127.0.0.1:5060")]
    sip_addr: SocketAddr,
}

#[async_std::main]
async fn main() {
    let args: Args = Args::parse();
    env_logger::builder().format_module_path(false).format_timestamp_millis().init();
    match args.node_id {
        Some(node_id) => {
            let cluster = ServerAtm0s::new(node_id, ServerAtm0sConfig { neighbours: args.neighbours }).await;
            server::start_server(cluster, args.sip_addr).await;
        }
        None => {
            let cluster = ServerLocal::new();
            server::start_server(cluster, args.sip_addr).await;
        }
    }
}
