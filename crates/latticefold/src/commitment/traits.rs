/// Minimal abstraction to allow swapping different commitment backends.
///
/// It is intentionally lightweight: width/kappa are informational and may be
/// interpreted differently by concrete schemes (e.g. Ajtai uses them for
/// dimension checks; SWIFFT treats them as parameter hints/hashes).
pub trait CommitmentScheme {
    type Witness;
    type Commitment;

    /// Number of rows / output length.
    fn kappa(&self) -> usize;
    /// Number of columns / expected witness length, if applicable.
    fn width(&self) -> usize;
    /// Commit to a witness.
    fn commit(
        &self,
        witness: &Self::Witness,
    ) -> Result<Self::Commitment, crate::commitment::CommitmentError>;
}
