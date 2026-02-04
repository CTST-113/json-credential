use crate::ast::Node;
use crate::errors::JcError;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task::{self, JoinSet};
use std::collections::VecDeque;


use p256::{ProjectivePoint, NistP256};
use elliptic_curve::hash2curve::{ExpandMsgXmd, GroupDigest};

use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct PrimitiveEntry {
    /// path，like [("obj", "key"), ("arr", "0")]
    pub path: Vec<(String, String)>,
    pub value: Value,
}

impl PrimitiveEntry {
    /// a simple async function
    pub async fn func(&self) {
        println!("Processing primitive at {:?}", self.path);
    }

    /// hash2curve async function
    pub async fn to_generator(&self) -> Result<(), JcError> {
        // ...
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use p256::NistP256;
    use elliptic_curve::hash2curve::{ExpandMsgXmd, GroupDigest};
    
    use sha2::Sha256;

    #[test]
    fn func() {
        let msg = b"Hello, world!";
        let dst = b"MY_APP_DOMAIN_SEPARATION_TAG";

        let point = NistP256::hash_from_bytes::<ExpandMsgXmd<Sha256>>(
            &[msg],
            &[dst]
        ).expect("hashing failed");

        println!("Hashed Point: {:?}", point);
    }
}
