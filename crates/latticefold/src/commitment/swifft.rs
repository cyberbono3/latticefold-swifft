#![cfg(feature = "swifft")]

use rand::RngCore;
use swifft::{Block, Key, State, BLOCK_LEN, KEY_LEN};

/// SWIFFT-backed commitment digest (currently a thin wrapper over `State`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwifftCommitment {
    state: State,
}

impl SwifftCommitment {
    pub fn as_bytes(&self) -> &[u8; swifft::STATE_LEN] {
        self.state.as_bytes()
    }

    pub fn into_inner(self) -> State {
        self.state
    }
}

impl AsRef<[u8]> for SwifftCommitment {
    fn as_ref(&self) -> &[u8] {
        self.state.as_bytes()
    }
}

/// SWIFFT compression wrapper to align with the commitment interface.
#[derive(Clone, Debug)]
pub struct SwifftCommitmentScheme {
    key: Key,
}

impl SwifftCommitmentScheme {
    pub fn new(key: Key) -> Self {
        Self { key }
    }

    /// Sample a random SWIFFT key (1024 bytes modulo 257).
    pub fn rand<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let mut key_bytes = [0u8; KEY_LEN];
        rng.fill_bytes(&mut key_bytes);
        Self {
            key: Key::from(key_bytes),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    /// Compress arbitrary bytes by chunking into 56-byte SWIFFT blocks.
    /// Currently pads the final block with zeros.
    pub fn commit_bytes(&self, message: &[u8]) -> SwifftCommitment {
        let mut state = State::default();
        let mut block_bytes = [0u8; BLOCK_LEN];

        for chunk in message.chunks(BLOCK_LEN) {
            block_bytes.fill(0);
            block_bytes[..chunk.len()].copy_from_slice(chunk);
            let block = Block::from(block_bytes);
            state.compress(&self.key, &block);
        }

        SwifftCommitment { state }
    }
}
