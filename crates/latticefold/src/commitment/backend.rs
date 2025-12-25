//! Transitional helper to pick between Ajtai commitments and SWIFFT hashing.
//!
//! This keeps the legacy Ajtai path intact while allowing feature-gated
//! SWIFFT-based hashing of witnesses without touching protocol code yet.

use cyclotomic_rings::rings::SuitableRing;
use super::{AjtaiCommitmentScheme, CommitmentDigest, CommitmentError};
use crate::{arith::Witness, decomposition_parameters::DecompositionParams};

#[cfg(feature = "swifft")]
mod swifft_cache {
    use core::cell::RefCell;

    use super::super::{SwifftCommitment, SwifftCommitmentBuffer, SwifftCommitmentScheme};
    use crate::{arith::Witness, commitment::CommitmentError, decomposition_parameters::DecompositionParams};
    use cyclotomic_rings::rings::SuitableRing;

    thread_local! {
        static BUFFERS: RefCell<(Vec<u8>, SwifftCommitmentBuffer)> =
            RefCell::new((Vec::new(), SwifftCommitmentBuffer::default()));
    }

    pub fn commit_with_cached_buffers<NTT: SuitableRing, P: DecompositionParams>(
        witness: &Witness<NTT>,
        scheme: &SwifftCommitmentScheme,
    ) -> Result<SwifftCommitment, CommitmentError>
    where
        NTT: ark_serialize::CanonicalSerialize,
    {
        BUFFERS.with(|buffers| {
            let mut buffers = buffers.borrow_mut();
            let (bytes, buffer) = &mut *buffers;
            witness.swifft_commit_with_buffer::<P>(scheme, bytes, buffer)
        })
    }
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
    ) -> Result<CommitmentDigest<NTT>, CommitmentError>
    where
        NTT: ark_serialize::CanonicalSerialize,
    {
        match self {
            Self::Ajtai(scheme) => Ok(CommitmentDigest::Ajtai(witness.commit::<P>(scheme)?)),
            #[cfg(feature = "swifft")]
            Self::Swifft(scheme) => Ok(CommitmentDigest::Swifft(
                swifft_cache::commit_with_cached_buffers::<NTT, P>(witness, scheme)?,
            )),
        }
    }
}
