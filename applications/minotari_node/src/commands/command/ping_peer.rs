//  Copyright 2022, The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use anyhow::Error;
use async_trait::async_trait;
use clap::Parser;
use minotari_app_utilities::utilities::UniNodeId;
use tari_comms::peer_manager::NodeId;
use tari_p2p::services::liveness::LivenessEvent;
use tokio::{sync::broadcast::error::RecvError, task};

use super::{CommandContext, HandleCommand};

/// Send a ping to a known peer and wait for a pong reply
#[derive(Debug, Parser)]
pub struct Args {
    /// hex public key or emoji id
    node_id: UniNodeId,
}

#[async_trait]
impl HandleCommand<Args> for CommandContext {
    async fn handle_command(&mut self, args: Args) -> Result<(), Error> {
        self.ping_peer(args.node_id.into()).await
    }
}

impl CommandContext {
    /// Function to process the ping-peer command
    pub async fn ping_peer(&mut self, dest_node_id: NodeId) -> Result<(), Error> {
        let mut liveness_events = self.liveness.get_event_stream();
        let mut liveness = self.liveness.clone();
        task::spawn(async move {
            match liveness.send_ping(dest_node_id.clone()).await {
                Ok(nonce) => {
                    println!("🏓 Pinging peer {} with nonce {} ...", dest_node_id, nonce);
                    loop {
                        match liveness_events.recv().await {
                            Ok(event) => {
                                if let LivenessEvent::ReceivedPong(pong) = &*event {
                                    if pong.node_id == dest_node_id && pong.nonce == nonce {
                                        println!(
                                            "🏓️ Pong: peer {} responded with nonce {}, round-trip-time is {:.2?}!",
                                            pong.node_id,
                                            pong.nonce,
                                            pong.latency.unwrap_or_default()
                                        );
                                        break;
                                    }
                                }
                            },
                            Err(RecvError::Closed) => {
                                break;
                            },
                            Err(RecvError::Lagged(_)) => {},
                        }
                    }
                },
                Err(e) => {
                    println!("🏓 Ping failed to send to {}: {}", dest_node_id, e);
                },
            }
        });
        Ok(())
    }
}
