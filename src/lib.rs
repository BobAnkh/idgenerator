//! # idgenerator
//!
//! A powerful unique id generator.
//!
//! ## Out-of-the-box instances
//!
//! This lib has provided two out-of-the-box implementations:
//!
//! - `IdInstance`: a instance with only one generator. See [examples/single.rs](https://github.com/BobAnkh/idgenerator/blob/main/examples/single.rs) for usage example.
//! - `IdVecInstance`: a instance with multiple generators. See [examples/multiple.rs](https://github.com/BobAnkh/idgenerator/blob/main/examples/multiple.rs) for usage example.
//!
//! ## Self-implement instance
//!
//! This lib wraps the snowflake algorithm inside the `CoreIdGenerator` struct.
//! You can wrap this struct inside your own instance of unique id generator.
//!
//! ## Others
//!
//! - `IdGeneratorOptions`: options for id generator.
//! - `OptionError`: errors for setting options.

pub mod error;
pub mod generator;
pub mod instance;
pub mod options;

pub use error::OptionError;
pub use generator::CoreIdGenerator;
pub use instance::{IdInstance, IdVecInstance};
pub use options::IdGeneratorOptions;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashSet;
    use std::time::Instant;

    #[test]
    fn test_single_instance() {
        let mut new_id: i64 = 0;
        let mut times = 500000;
        let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
        let res = IdInstance::init(options);
        assert!(res.is_ok());
        let options = IdInstance::get_options();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdGeneratorOptions::new().seq_bit_len(12);
        let res = IdInstance::set_options(options);
        assert!(res.is_ok());
        let options = IdInstance::get_options();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let start = Instant::now();
        while times > 0 {
            new_id = IdInstance::next_id();
            times -= 1;
        }

        let duration = start.elapsed();
        println!(
            "Program finished after {} seconds! Last id {}",
            duration.as_secs(),
            new_id
        );
    }

    #[test]
    fn test_single_instance_check() {
        let mut set: HashSet<i64> = HashSet::new();
        let mut times = 500000;
        let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
        let res = IdInstance::init(options);
        assert!(res.is_ok());
        let options = IdInstance::get_options();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdGeneratorOptions::new().seq_bit_len(12);
        let res = IdInstance::set_options(options);
        assert!(res.is_ok());
        let options = IdInstance::get_options();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let start = Instant::now();
        while times > 0 {
            let new_id = IdInstance::next_id();
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }

        let duration = start.elapsed();
        println!("Program finished after {} seconds!", duration.as_secs());
    }

    #[test]
    fn test_vec_instance() {
        let mut new_id: i64 = 0;
        let mut new_id_snd: i64 = 0;
        let mut times = 500000;
        let options = vec![
            IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6),
            IdGeneratorOptions::new().worker_id(2).worker_id_bit_len(6),
        ];
        let res = IdVecInstance::init(options);
        assert!(res.is_ok());
        let options = IdVecInstance::get_options(0).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdVecInstance::get_options(1).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(2),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdGeneratorOptions::new().seq_bit_len(12);
        let res = IdVecInstance::set_options(0, options.clone());
        assert!(res.is_ok());
        let res = IdVecInstance::set_options(1, options);
        assert!(res.is_ok());
        let options = IdVecInstance::get_options(0).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdVecInstance::get_options(1).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(2),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let start = Instant::now();
        while times > 0 {
            new_id = IdVecInstance::next_id(0);
            new_id_snd = IdVecInstance::next_id(1);
            times -= 1;
        }

        let duration = start.elapsed();
        println!(
            "Program finished after {} seconds! Last id {}, {}",
            duration.as_secs(),
            new_id,
            new_id_snd,
        );
    }

    #[test]
    fn test_vec_instance_check() {
        let mut set: HashSet<i64> = HashSet::new();
        let mut times = 500000;
        let options = vec![
            IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6),
            IdGeneratorOptions::new().worker_id(2).worker_id_bit_len(6),
        ];
        let res = IdVecInstance::init(options);
        assert!(res.is_ok());
        let options = IdVecInstance::get_options(0).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdVecInstance::get_options(1).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(2),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(8),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdGeneratorOptions::new().seq_bit_len(12);
        let res = IdVecInstance::set_options(0, options.clone());
        assert!(res.is_ok());
        let res = IdVecInstance::set_options(1, options);
        assert!(res.is_ok());
        let options = IdVecInstance::get_options(0).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(1),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let options = IdVecInstance::get_options(1).unwrap();
        assert_eq!(
            options,
            IdGeneratorOptions {
                method: Some(1),
                base_time: Some(1582136402000),
                worker_id: Some(2),
                worker_id_bit_len: Some(6),
                seq_bit_len: Some(12),
                max_seq_num: Some(255),
                min_seq_num: Some(5),
                top_over_cost_count: Some(2000),
            }
        );
        let start = Instant::now();
        while times > 0 {
            let new_id = IdVecInstance::next_id(0);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            let new_id = IdVecInstance::next_id(1);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }

        let duration = start.elapsed();
        println!("Program finished after {} seconds!", duration.as_secs());
    }
}
