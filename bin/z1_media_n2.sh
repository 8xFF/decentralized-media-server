RUST_LOG=info \
RUST_BACKTRACE=1 \
cargo run -- \
    --enable-private-ip \
    --sdn-zone-id 1 \
    --sdn-zone-node-id 2 \
    --seeds-from-url "http://localhost:4000/api/node/address" \
    --workers 2 \
    media \
        --webrtc-port-seed 11200 \
        --enable-token-api
