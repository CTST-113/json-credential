
use p256::{ProjectivePoint, NistP256};
use crate::{commit::generator::PrimitiveEntry, errors::JcError};

pub struct SingleCommit {
    generator: ProjectivePoint,
    commit: ProjectivePoint,
}

impl SingleCommit {
    /// get commitment from an entry
    pub async fn commit(entry: PrimitiveEntry) -> Result<(), JcError> {
        // ...
        Ok(())
    }
}