use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    gossipsub::{self},
    noise,
    swarm::SwarmEvent,
    tcp,
    yamux,
    Multiaddr,
};
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Create a custom topic for our messages
    let message_topic = gossipsub::IdentTopic::new("chat");

    // Build the swarm with gossipsub behavior
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .build()
                .expect("Valid config");

            let mut gossipsub: gossipsub::Behaviour<gossipsub::IdentityTransform> = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )
            .expect("Valid configuration");

            gossipsub.subscribe(&message_topic).unwrap();
            gossipsub
        })?
        .build();

    // Listen on all interfaces
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    println!("Commands available:");
    println!("  /dial <multiaddr> - Connect to a peer");
    println!("  /msg <message> - Send a message to connected peers");
    println!("  /quit - Exit the program");

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                match line.split_once(' ') {
                    Some(("/dial", addr)) => {
                        let addr: Multiaddr = addr.parse()?;
                        println!("Dialing {addr}");
                        swarm.dial(addr)?;
                    }
                    Some(("/msg", message)) => {
                        if let Err(e) = swarm
                            .behaviour_mut()
                            .publish(message_topic.clone(), message.as_bytes()) {
                            println!("Publishing error: {e:?}");
                        }
                    }
                    Some(("/quit", _)) => break,
                    _ => println!("Unknown command. Use /dial, /msg, or /quit"),
                }
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {address}");
                    }
                    SwarmEvent::Behaviour(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message,
                        ..
                    }) => {
                        println!(
                            "Got message: '{}' from {:?}",
                            String::from_utf8_lossy(&message.data),
                            peer_id
                        );
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        println!("Connected to {peer_id}");
                    }
                    SwarmEvent::ConnectionClosed { peer_id, .. } => {
                        println!("Disconnected from {peer_id}");
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
} 