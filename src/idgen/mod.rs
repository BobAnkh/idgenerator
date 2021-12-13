//! # idgen
//!
//! Core of the unique id generator
//!
//! Different Id helper will provide different support for worker id, see documentation for each
//!
//! IdGeneratorOptions will provide you a interface for setting generators' options
//!
//! ## Example
//!
//! ### Use one id generator with a unique worker id externally given
//!
//! ```rust
//! use idgenerator::{IdHelper, IdGeneratorOptions};
//! // Create a instance
//! IdHelper::init();
//! // Create IdGeneratorOptions, worker_id is the only parameter needed：
//! let mut options = IdGeneratorOptions::new(1);
//! // If you want to have a larger work_id range, set worker_id_bit_len to a larger number
//! options.worker_id_bit_len = 8; // default to 8, meaning the max number of work_id is 2^8 - 1
//! // Other options can be seen in IdGeneratorOptions
//!
//! // You must save parameters before generating
//! IdHelper::set_id_generator(options);
//!
//! // Then generate id
//! // call next_id() at where you want to generate the ID
//! let new_id: i64 = IdHelper::next_id();
//! println!("ID: {}", new_id);
//! ```
//!
//! ### Use arbitrary worker id given externally in individual generators using HashMap
//!
//! ```rust
//! use idgenerator::{IdMapHelper, IdGeneratorOptions};
//! // Create a instance
//! IdMapHelper::init(vec![10, 2]);
//! // Create IdGeneratorOptions, worker_id is the only parameter needed：
//! let mut options = IdGeneratorOptions::new(2);
//! // If you want to have a larger work_id range, set worker_id_bit_len to a larger number
//! options.worker_id_bit_len = 8; // default to 8, meaning the max number of work_id is 2^8 - 1
//! // Other options can be seen in IdGeneratorOptions
//!
//! // You must save parameters before generating
//! IdMapHelper::set_id_generator(options);
//!
//! // Then generate id
//! // call next_id() at where you want to generate the ID
//! let new_id: i64 = IdMapHelper::next_id(2);
//! println!("ID: {}", new_id);
//! ```
//!
//! ### Use worker id as arithmetic progression in individual generators using Vector
//!
//! ```rust
//! use idgenerator::{IdVecHelper, IdGeneratorOptions};
//! // Create a instance
//! // worker id = [10, 12, 14]
//! IdVecHelper::init(10, 2, 3);
//! // Create IdGeneratorOptions, worker_id is the only parameter needed：
//! let mut options = IdGeneratorOptions::new(12);
//! // If you want to have a larger work_id range, set worker_id_bit_len to a larger number
//! options.worker_id_bit_len = 8; // default to 8, meaning the max number of work_id is 2^8 - 1
//! // Other options can be seen in IdGeneratorOptions
//!
//! // You must save parameters before generating
//! IdVecHelper::set_id_generator(options);
//!
//! // Then generate id
//! // call next_id() at where you want to generate the ID
//! let new_id: i64 = IdVecHelper::next_id(12);
//! println!("ID: {}", new_id);
//! ```
pub mod id_generator;
pub mod id_generator_options;
pub mod id_helper;

pub use id_generator::SnowWorker;

pub use id_generator::DefaultIdGenerator;
pub use id_generator_options::IdGeneratorOptions;
pub use id_helper::{IdHelper, IdMapHelper, IdVecHelper};
