use serde_json::Value;
use tokio::task::{self, JoinSet};
use tokio::sync::mpsc;
use std::collections::VecDeque;

use crate::ast::Node;
use crate::commit::{self, batch_commit};
use crate::commit::generator::PrimitiveEntry;
use crate::commit::single_commit::SingleCommit;


use p256::{ProjectivePoint, NistP256};
use elliptic_curve::hash2curve::{ExpandMsgXmd, GroupDigest};

use sha2::Sha256;

pub struct BatchCommit;

impl BatchCommit {
    /// Traverse the JSON AST using a queue, 
    /// collect the prefix path for each primitive, 
    /// and calculate the final commitment.
    pub async fn commit(node: Node) {
        // new a channel
        let (tx, mut rx) = mpsc::channel(100);
        let mut queue = VecDeque::new();
        
        // initial queue
        queue.push_back((node, Vec::new()));

        let mut _tasks = JoinSet::new();

        while let Some((current_node, current_path)) = queue.pop_front() {
            match current_node {
                Node::Object(fields) => {
                    for (k, v) in fields {
                        let mut next_path = current_path.clone();
                        next_path.push(("obj".to_string(), k));
                        queue.push_back((v, next_path));
                    }
                }
                Node::Array(elements) => {
                    for (i, v) in elements.into_iter().enumerate() {
                        let mut next_path = current_path.clone();
                        next_path.push(("arr".to_string(), i.to_string()));
                        queue.push_back((v, next_path));
                    }
                }
                Node::Primitive(value) => {
                    let tx = tx.clone();
                    let entry = PrimitiveEntry {
                        path: current_path,
                        value,
                    };
                    
                    // start an async task
                    _tasks.spawn(async move {
                        entry.func().await;
                        let commit = match SingleCommit::commit(entry).await {
                            Ok(v) => v,
                            Err(e) => {
                                println!("Something went wrong: {:?}", e)
                            }
                        };
                        let _ = tx.send(commit).await;
                    });
                }
            }
        }

        // drop the channel
        drop(tx);

        // init final commit with h^r
        // ...
        // eg: let mut batch_commit = h.mul(r);

        // accept commit from tasks, add them to final commit
        while let Some(commit) = rx.recv().await {
            // ... 
            // eg: batch_commit.add(commit);
        }

        // return final commit
        // batch_commit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;

    #[tokio::test]
    async fn test_batch_commit_concurrent() {
        let json = r#"{"user": {"name": "Alice", "ids": [101, 102]}}"#;
        let ast = Node::parse_str(json).unwrap();
        
        let entries = BatchCommit::commit(ast).await;
    }
}