//! Core types and structures for LlmFlow
//!
//! This module contains the fundamental types used throughout the library.
use std::sync::Arc;
use thiserror::Error;

const MAX_RETRIES: u8 = 10;
const MAX_WAIT_SECONDS: u8 = 60;

/// A node in the graph
///
/// This struct represents a node in the graph. It contains a name and a pointer to the next node in the graph.
#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    next: Option<Arc<Node>>,
    max_retries: u8,
    wait: u8,
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Node execution failed: {0}")]
    ExecutionError(String),

    #[error("Invalid retry count: {0}. Maximum allowed retries is {1}")]
    InvalidRetryCount(u8, u8),

    #[error("Invalid wait time: {0} seconds. Maximum allowed wait time is {1} seconds")]
    InvalidWaitTime(u8, u8),

    #[error("Node name cannot be empty")]
    EmptyNodeName,

    #[error("Execution retry limit reached after {attempts} attempts: {message}")]
    RetryLimitExceeded { attempts: u8, message: String },
}

impl Node {
    /// Creates a new node with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node
    pub fn new(name: Option<&str>) -> Result<Self, NodeError> {
        let name = name.unwrap_or("default").to_string();
        if name.is_empty() {
            return Err(NodeError::EmptyNodeName);
        }
        Ok(Self {
            name,
            next: None,
            max_retries: 0,
            wait: 0,
        })
    }

    /// Sets the next node and returns a new Node with updated next
    pub fn with_next(mut self, node: Arc<Node>) -> Self {
        self.next = Some(node);
        self
    }

    /// Sets the max number of retries
    pub fn with_retries(mut self, retries: u8) -> Result<Self, NodeError> {
        if retries > MAX_RETRIES {
            return Err(NodeError::InvalidRetryCount(retries, MAX_RETRIES));
        }
        self.max_retries = retries;
        Ok(self)
    }

    /// Sets the wait time in secs between retries
    pub fn with_wait(mut self, wait: u8) -> Result<Self, NodeError> {
        if wait > MAX_WAIT_SECONDS {
            return Err(NodeError::InvalidWaitTime(wait, MAX_WAIT_SECONDS));
        }
        self.wait = wait;
        Ok(self)
    }

    /// Returns a reference to the next node in the graph
    pub fn next(&self) -> Option<&Arc<Node>> {
        self.next.as_ref()
    }

    /// Executes the node's logic with retry capability
    pub fn exec(&self) -> Result<(), NodeError> {
        println!("Executing node {}", self.name);
        let mut attempts = 0;
        while attempts <= self.max_retries {
            match self.execute_logic() {
                Ok(_) => break,
                Err(e) if attempts < self.max_retries => {
                    std::thread::sleep(std::time::Duration::from_secs(self.wait as u64));
                    attempts += 1;
                    continue;
                }
                Err(e) => {
                    return Err(NodeError::RetryLimitExceeded {
                        attempts,
                        message: e.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Internal method to execute the node's actual logic
    fn execute_logic(&self) -> Result<(), NodeError> {
        // Placeholder for actual node execution logic
        Ok(())
    }
}
