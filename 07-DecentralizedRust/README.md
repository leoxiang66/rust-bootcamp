# Rust P2P Network Examples

This project demonstrates various peer-to-peer networking concepts using Rust and libp2p.

## Basic P2P Example

The main example demonstrates:
- Peer identification using Ed25519 keypairs
- TCP transport with Noise encryption
- Yamux multiplexing
- Basic ping protocol implementation
- Interactive command-line interface

### Running the Example

1. Build and run the example:
```bash
RUST_LOG=info cargo run
```

2. The program will display your local peer ID and listening address.

3. Open another terminal and run another instance of the program.

4. In either terminal, use the `/dial` command with the multiaddr shown in the other terminal:
```
/dial /ip4/127.0.0.1/tcp/<port>
```

5. You should see ping messages being exchanged between the peers.

6. Use `/quit` to exit the program.

## Key Concepts Demonstrated

1. **Peer Identity**
   - Each peer generates a unique Ed25519 keypair
   - PeerID is derived from the public key

2. **Transport**
   - TCP transport with automatic port selection
   - Noise protocol for encryption
   - Yamux for stream multiplexing

3. **Network Behavior**
   - Ping protocol implementation
   - Event-driven architecture
   - Asynchronous I/O handling

4. **Command Interface**
   - Interactive command-line interface
   - Manual peer connection
   - Connection status monitoring

## Next Steps

Future examples will demonstrate:
- Custom protocol implementation
- Peer discovery using Kademlia DHT
- QUIC transport usage
- Secure messaging

## Project Structure 

### Examples

1. Basic Communication (`basic-communication/`)
   - Simple ping example demonstrating basic peer-to-peer communication
   - Tests peer discovery and connection establishment
   - Run test: `cargo test -p basic-communication`

2. MPC (Multi-Party Computation) (`mpc-libp2p/`)
   - Demonstrates gossipsub protocol for distributed computation
   - Implements a simple sum computation across multiple peers
   - Run test: `cargo test -p mpc-libp2p`

3. QUIC Protocol (`protocols/src/bin/quic_example.rs`)
   - Shows QUIC transport usage in libp2p
   - Implements basic peer discovery and connection handling
   - Run example: `cargo run --bin quic_example`

### Running Tests

To run all tests:
```
cargo test --workspace
```

