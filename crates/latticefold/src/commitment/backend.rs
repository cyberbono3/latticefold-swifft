//! Transitional helper to pick between Ajtai commitments and SWIFFT hashing.
//!
//! This keeps the legacy Ajtai path intact while allowing feature-gated
//! SWIFFT-based hashing of witnesses without touching protocol code yet.

use cyclotomic_rings::rings::SuitableRing;
use stark_rings::Ring;

use super::{AjtaiCommitmentScheme, Commitment, CommitmentError};
use crate::{arith::Witness, decomposition_parameters::DecompositionParams};

/// Unified commitment output.
#[derive(Clone, Debug)]
pub enum CommitmentOutput<NTT: Ring> {
    Ajtai(Commitment<NTT>),
    #[cfg(feature = "swifft")]
    Swifft(super::SwifftCommitment),
}

/// Backend selector for commitment generation.
pub enum CommitmentBackend<'a, NTT: SuitableRing> {
    Ajtai(&'a AjtaiCommitmentScheme<NTT>),
    #[cfg(feature = "swifft")]
    Swifft(&'a super::SwifftCommitmentScheme),
}

impl<'a, NTT: SuitableRing> CommitmentBackend<'a, NTT> {
    /// Commit to a witness using the selected backend.
    pub fn commit<P: DecompositionParams>(
        &self,
        witness: &Witness<NTT>,
    ) -> Result<CommitmentOutput<NTT>, CommitmentError>
    where
        NTT: ark_serialize::CanonicalSerialize,
    {
        match self {
            Self::Ajtai(scheme) => Ok(CommitmentOutput::Ajtai(witness.commit::<P>(scheme)?)),
            #[cfg(feature = "swifft")]
            Self::Swifft(scheme) => Ok(CommitmentOutput::Swifft(witness.swifft_commit(scheme)?)),
        }
    }
}
