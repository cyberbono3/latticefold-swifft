//! Provides utility for committing to witnesses.

use ark_serialize::SerializationError;
use thiserror::Error;

mod commitment_scheme;
mod homomorphic_commitment;
#[macro_use]
mod operations;
mod backend;
#[cfg(feature = "swifft")]
mod swifft;
mod traits;
pub use backend::{CommitmentBackend, CommitmentOutput};
pub use commitment_scheme::*;
pub use homomorphic_commitment::*;
#[cfg(feature = "swifft")]
pub use swifft::{SwifftCommitment, SwifftCommitmentScheme};
pub use traits::CommitmentScheme;

/// Errors that can occur in commitment operations.
#[derive(Debug, Error)]
pub enum CommitmentError {
    /// The witness to be committed has the wrong length.
    #[error("Wrong length of the witness: {0}, expected: {1}")]
    WrongWitnessLength(usize, usize),
    /// The commitment to a witness has the wrong length.
    #[error("Wrong length of the commitment: {0}, expected: {1}")]
    WrongCommitmentLength(usize, usize),
    /// The Ajtai commitment matrix has the wrong dimensions.
    ///
    /// An Ajtai matrix should have size commitment_length x witness_length.
    #[error("Ajtai matrix has dimensions: {0}x{1}, expected: {2}x{3}")]
    WrongAjtaiMatrixDimensions(usize, usize, usize, usize),
    /// Serialization error while preparing a commitment input.
    #[error("serialization error: {0}")]
    Serialization(#[from] SerializationError),
}
