// Copyright 2019. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fmt::{Display, Error, Formatter};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use tari_common_types::types::{BlockHash, PrivateKey};
use tari_utilities::hex::Hex;

use crate::{blocks::block_header::BlockHeader, proof_of_work::ProofOfWork};
/// The NewBlockHeaderTemplate is used for the construction of a new mineable block. It contains all the metadata for
/// the block that the Base Node is able to complete on behalf of a Miner.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NewBlockHeaderTemplate {
    /// Version of the block
    pub version: u16,
    /// Height of this block since the genesis block (height 0)
    pub height: u64,
    /// Hash of the block previous to this in the chain.
    pub prev_hash: BlockHash,
    /// Total accumulated sum of kernel offsets since genesis block. We can derive the kernel offset sum for *this*
    /// block from the total kernel offset of the previous block header.
    pub total_kernel_offset: PrivateKey,
    /// Sum of script offsets for all kernels in this block.
    pub total_script_offset: PrivateKey,
    /// Proof of work summary
    pub pow: ProofOfWork,
}

impl NewBlockHeaderTemplate {
    pub(crate) fn from_header(header: BlockHeader) -> Self {
        Self {
            version: header.version,
            height: header.height,
            prev_hash: header.prev_hash,
            total_kernel_offset: header.total_kernel_offset,
            total_script_offset: header.total_script_offset,
            pow: header.pow,
        }
    }

    pub fn empty() -> Self {
        Self {
            version: 0,
            height: 0,
            prev_hash: Default::default(),
            total_kernel_offset: Default::default(),
            total_script_offset: Default::default(),
            pow: Default::default(),
        }
    }
}

impl Display for NewBlockHeaderTemplate {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        let msg = format!(
            "Version: {}\nBlock height: {}\nPrevious block hash: {}\n",
            self.version, self.height, self.prev_hash,
        );
        fmt.write_str(&msg)?;
        fmt.write_str(&format!(
            "Total offset: {}\nTotal script offset: {}\nProof of work: {}",
            self.total_kernel_offset.to_hex(),
            self.total_script_offset.to_hex(),
            self.pow
        ))
    }
}
