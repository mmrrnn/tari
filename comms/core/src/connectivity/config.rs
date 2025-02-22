//  Copyright 2020, The Tari Project
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

use std::time::Duration;

/// Connectivity actor configuration
#[derive(Debug, Clone, Copy)]
pub struct ConnectivityConfig {
    /// The minimum number of connected nodes before connectivity is transitioned to ONLINE
    /// Default: 1
    pub min_connectivity: usize,
    /// Interval to check the connection pool, including reaping inactive connections and retrying failed managed peer
    /// connections. Default: 60s
    pub connection_pool_refresh_interval: Duration,
    /// True if connection reaping is enabled, otherwise false (default: true)
    pub is_connection_reaping_enabled: bool,
    /// The minimum number of connections that must exist before any connections may be reaped
    /// Default: 50
    pub reaper_min_connection_threshold: usize,
    /// The minimum age of the connection before it can be reaped. This prevents a connection that has just been
    /// established from being reaped due to inactivity. Default: 20 minutes
    pub reaper_min_inactive_age: Duration,
    /// The number of connection failures before a peer is considered offline
    /// Default: 1
    pub max_failures_mark_offline: usize,
    /// The length of time to wait before disconnecting a connection that failed tie breaking.
    /// Default: 1s
    pub connection_tie_break_linger: Duration,
    /// If the peer has not been seen within this interval, it will be removed from the peer list on the
    /// next connection attempt.
    /// Default: 24 hours
    pub expire_peer_last_seen_duration: Duration,
    /// The closest number of peer connections to maintain; connections above the threshold will be removed
    /// (default: disabled)
    pub maintain_n_closest_connections_only: Option<usize>,
}

impl Default for ConnectivityConfig {
    fn default() -> Self {
        Self {
            min_connectivity: 1,
            connection_pool_refresh_interval: Duration::from_secs(60),
            reaper_min_inactive_age: Duration::from_secs(20 * 60),
            reaper_min_connection_threshold: 50,
            is_connection_reaping_enabled: true,
            max_failures_mark_offline: 1,
            connection_tie_break_linger: Duration::from_secs(2),
            expire_peer_last_seen_duration: Duration::from_secs(24 * 60 * 60),
            maintain_n_closest_connections_only: None,
        }
    }
}
