//! LlmFlow - Flow-based programming for Rust
//!
//! A library for creating and running flows of connected nodes, inspired by flow-based programming.
//! Supports both synchronous and asynchronous execution patterns.
//!
//! # Features
//!
//! * Basic Node and Flow abstractions
//! * Retry mechanisms with configurable backoff
//! * Batch processing
//! * Async support via Tokio
//! * Parallel processing capabilities
//!
//! # Example
//!
//! ```rust
//! use llmflow::prelude::*;
//! use std::collections::HashMap;
//!
//! struct MyNode;
//!
//! impl Node for MyNode {
//!     fn exec(&self, prep_res: SharedData) -> Result<NodeOutput, NodeError> {
//!         println!("Processing data: {:?}", prep_res);
//!         Ok(NodeOutput::Action("default".to_string()))
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut flow = Flow::new();
//!     let node = Box::new(MyNode);
//!
//!     flow.start(node);
//!
//!     let mut shared = HashMap::new();
//!     shared.insert("key".to_string(), "value".to_string());
//!
//!     let result = flow.run(shared);
//!     println!("Result: {:?}", result);
//! }
//! ```

pub mod core;

/// Re-export of the most commonly used types and traits
pub mod prelude {
    pub use crate::core::Node;
}
