#![cfg(feature = "swifft")]

use core::mem;

use ark_serialize::CanonicalSerialize;
use swifft::{Block, Key, State, BLOCK_LEN, KEY_LEN, STATE_LEN};

use super::{traits::CommitmentScheme as CommitmentSchemeTrait, CommitmentError};

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

/// Reusable buffers for SWIFFT commitments to avoid per-call allocations.
#[derive(Clone, Debug)]
pub struct SwifftCommitmentBuffer {
    state: State,
    block: Block,
}

impl Default for SwifftCommitmentBuffer {
    fn default() -> Self {
        Self {
            state: State::default(),
            block: Block::default(),
        }
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

    /// Compress arbitrary bytes using a reusable buffer.
    pub fn commit_bytes_with_buffer(
        &self,
        message: &[u8],
        buffer: &mut SwifftCommitmentBuffer,
    ) -> SwifftCommitment {
        buffer.state = State::default();

        for chunk in message.chunks(BLOCK_LEN) {
            buffer.block.0.fill(0);
            buffer.block.0[..chunk.len()].copy_from_slice(chunk);
            buffer.state.compress(&self.key, &buffer.block);
        }

        let state = mem::replace(&mut buffer.state, State::default());
        SwifftCommitment { state }
    }

    /// Compress arbitrary bytes by chunking into 56-byte SWIFFT blocks.
    /// Currently pads the final block with zeros.
    pub fn commit_bytes(&self, message: &[u8]) -> SwifftCommitment {
        let mut buffer = SwifftCommitmentBuffer::default();
        self.commit_bytes_with_buffer(message, &mut buffer)
    }

    /// Commit to an object that implements `CanonicalSerialize` by serializing it.
    pub fn commit_serialized<S: CanonicalSerialize>(
        &self,
        serializable: &S,
    ) -> Result<SwifftCommitment, CommitmentError> {
        let mut buf = Vec::new();
        let mut buffer = SwifftCommitmentBuffer::default();
        self.commit_serialized_with_buffer(serializable, &mut buf, &mut buffer)
    }

    /// Commit to a serializable value using reusable buffers.
    pub fn commit_serialized_with_buffer<S: CanonicalSerialize>(
        &self,
        serializable: &S,
        buf: &mut Vec<u8>,
        buffer: &mut SwifftCommitmentBuffer,
    ) -> Result<SwifftCommitment, CommitmentError> {
        buf.clear();
        serializable.serialize_compressed(&mut *buf)?;
        Ok(self.commit_bytes_with_buffer(buf, buffer))
    }

    /// Convenience helper to commit to a slice of serializable items.
    pub fn commit_serialized_slice<S: CanonicalSerialize>(
        &self,
        slice: &[S],
    ) -> Result<SwifftCommitment, CommitmentError> {
        let mut buf = Vec::new();
        let mut buffer = SwifftCommitmentBuffer::default();
        self.commit_serialized_slice_with_buffer(slice, &mut buf, &mut buffer)
    }

    /// Convenience helper to commit to a slice of serializable items using reusable buffers.
    pub fn commit_serialized_slice_with_buffer<S: CanonicalSerialize>(
        &self,
        slice: &[S],
        buf: &mut Vec<u8>,
        buffer: &mut SwifftCommitmentBuffer,
    ) -> Result<SwifftCommitment, CommitmentError> {
        buf.clear();
        for item in slice {
            item.serialize_compressed(&mut *buf)?;
        }
        Ok(self.commit_bytes_with_buffer(buf, buffer))
    }
}

impl CommitmentSchemeTrait for SwifftCommitmentScheme {
    type Witness = [u8];
    type Commitment = SwifftCommitment;

    fn kappa(&self) -> usize {
        STATE_LEN
    }

    fn width(&self) -> usize {
        BLOCK_LEN
    }

    fn commit(&self, witness: &Self::Witness) -> Result<Self::Commitment, CommitmentError> {
        Ok(self.commit_bytes(witness))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_bytes_with_buffer_matches_default() {
        let scheme = SwifftCommitmentScheme::new(Key::from([7u8; KEY_LEN]));
        let message = b"swifft buffer test message";
        let mut buffer = SwifftCommitmentBuffer::default();

        let from_default = scheme.commit_bytes(message);
        let from_buffer = scheme.commit_bytes_with_buffer(message, &mut buffer);

        assert_eq!(from_default.as_bytes(), from_buffer.as_bytes());
    }
}
