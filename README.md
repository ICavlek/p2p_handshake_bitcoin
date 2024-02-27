# p2p_handshake_bitcoin
P2P handshake in Bitcoin

## Example usage:

To make a handshake, it is necessary to have a node running on Bitcoin network with which communication can be established.
To communicate with it, ip address of the node has to be provided. Additionally, optional timeout parameter can be added:

```bash
$ cargo run 45.9.148.241:8333 -t 1000
```

If the handshake was successful, following message should appear:

```bash
[2024-02-27T18:25:25.068Z]  INFO: p2p_handshake_bitcoin/17798 on PC: Successfully performed handshake for Node 45.9.148.241:8333 (file=src/bitcoin/client_pool.rs,line=72,target=p2p_handshake_bitcoin::bitcoin::client_pool)
```

If the handshake was not successful, based on the error, something similar should appear:

```bash
[2024-02-27T18:28:43.598Z] ERROR: p2p_handshake_bitcoin/18030 on PC: Error with Node 45.9.148.241:8336 (file=src/bitcoin/client_pool.rs,line=75,target=p2p_handshake_bitcoin::bitcoin::client_pool)
    error.cause_chain: Failed to initialize TCP stream, deadline has elapsed
    --
    error.message: Failed to initialize TCP stream, deadline has elapsed

```

To run handshakes on multiple nodes, following command has to be run:

```bash
$ cargo run 45.9.148.241:8333 95.105.172.171:8333 46.17.99.26:8333
```

It is possible to also run it with the bunyan formatter which would output a nice looking log:

```bash
$ cargo run 45.9.148.241:8333 95.105.172.171:8333 46.17.99.26:8333 | bunyan
```
