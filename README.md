Rust toolchain: 
---------------
1.47.0-x86_64-unknown-linux-gnu

In directory of the project:
----------------------------
    docker build --force-rm --file Dockerfile.geth --tag geth .
    docker run --detach --name geth0 --publish 8545:8545 geth
    cargo run --release

Output:
-------
    Pending the transaction...
    just hangs no matter which transport to use
