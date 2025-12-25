use ark_serialize::CanonicalSerialize;
use stark_rings::Ring;

use super::{Commitment, CommitmentError};
#[cfg(feature = "swifft")]
use super::SwifftCommitment;

/// Binding commitment digest used for backend selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommitmentDigest<NTT: Ring> {
    Ajtai(Commitment<NTT>),
    #[cfg(feature = "swifft")]
    Swifft(SwifftCommitment),
}

impl<NTT: Ring> CommitmentDigest<NTT> {
    pub fn to_bytes(&self) -> Result<Vec<u8>, CommitmentError>
    where
        NTT: CanonicalSerialize,
    {
        match self {
            CommitmentDigest::Ajtai(commitment) => {
                let mut buf = Vec::new();
                commitment.serialize_compressed(&mut buf)?;
                Ok(buf)
            }
            #[cfg(feature = "swifft")]
            CommitmentDigest::Swifft(commitment) => Ok(commitment.as_bytes().to_vec()),
        }
    }
}
