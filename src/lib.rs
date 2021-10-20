// #[macro_use]
// extern crate lazy_static;

pub mod idgen;

pub use idgen::id_generator::DefaultIdGenerator;
pub use idgen::id_generator_options::IdGeneratorOptions;
pub use idgen::id_helper::IdHelper;

#[cfg(test)]
mod tests {
    use crate::{IdHelper, IdGeneratorOptions};
    use chrono::Utc;
    use std::collections::HashSet;

    #[test]
    fn single_thread_test() {
        let mut times = 5000000;
        IdHelper::init();
        let mut options = IdGeneratorOptions::new(1);
        options.seq_bit_len = 10;
        IdHelper::set_id_generator(options);
        let start = Utc::now().timestamp_millis();
        let mut new_id: i64 = 0;
        while times > 0 {
            new_id = IdHelper::next_id();
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Last one {}-ID: {}", 50000 - times, new_id);
        println!("Single thread time: {} ms", end - start);
    }

    #[test]
    fn single_thread_check() {
        let mut set: HashSet<i64> = HashSet::new();
        let mut times = 500000;
        IdHelper::init();
        let options = IdGeneratorOptions::new(1);
        IdHelper::set_id_generator(options);
        let start = Utc::now().timestamp_millis();
        while times > 0 {
            let new_id = IdHelper::next_id();
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Single thread check time: {} ms", end - start);
    }

    #[test]
    fn single_thread_test_map() {
        let mut times = 5000000;
        let worker_id: u32 = 1;
        IdHelper::init_map(vec![worker_id]);
        let mut options = IdGeneratorOptions::new(worker_id);
        options.seq_bit_len = 10;
        IdHelper::set_id_generator_map(options);
        let start = Utc::now().timestamp_millis();
        let mut new_id: i64 = 0;
        while times > 0 {
            new_id = IdHelper::next_id_map(worker_id);
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Last one {}-ID: {}", 500000 - times, new_id);
        println!("Single thread time: {} ms", end - start);
    }

    #[test]
    fn single_thread_check_map() {
        let mut set: HashSet<i64> = HashSet::new();
        let mut times = 500000;
        let worker_id: u32 = 1;
        IdHelper::init_map(vec![worker_id, worker_id+1]);
        let options = IdGeneratorOptions::new(worker_id);
        IdHelper::set_id_generator_map(options);
        let start = Utc::now().timestamp_millis();
        while times > 0 {
            let new_id = IdHelper::next_id_map(worker_id);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Single thread check time: {} ms", end - start);
        let options = IdGeneratorOptions::new(worker_id+1);
        IdHelper::set_id_generator_map(options);
        times = 500000;
        let start = Utc::now().timestamp_millis();
        while times > 0 {
            let new_id = IdHelper::next_id_map(worker_id+1);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Single thread check time: {} ms", end - start);
    }

    #[test]
    fn single_thread_test_vec() {
        let mut times = 5000000;
        let worker_id: u32 = 1;
        IdHelper::init_vec(worker_id, 2, 2);
        let mut options = IdGeneratorOptions::new(worker_id+2);
        options.seq_bit_len = 10;
        IdHelper::set_id_generator_vec(options);
        let start = Utc::now().timestamp_millis();
        let mut new_id: i64 = 0;
        while times > 0 {
            new_id = IdHelper::next_id_vec(worker_id+2);
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Last one {}-ID: {}", 500000 - times, new_id);
        println!("Single thread time: {} ms", end - start);
    }

    #[test]
    fn single_thread_check_vec() {
        let mut set: HashSet<i64> = HashSet::new();
        let mut times = 500000;
        let worker_id: u32 = 1;
        IdHelper::init_vec(worker_id, 2, 3);
        let options = IdGeneratorOptions::new(worker_id);
        IdHelper::set_id_generator_vec(options);
        let start = Utc::now().timestamp_millis();
        while times > 0 {
            let new_id = IdHelper::next_id_vec(worker_id);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Single thread check time: {} ms", end - start);
        times = 500000;
        let options = IdGeneratorOptions::new(worker_id+4);
        IdHelper::set_id_generator_vec(options);
        let start = Utc::now().timestamp_millis();
        while times > 0 {
            let new_id = IdHelper::next_id_vec(worker_id+4);
            if !set.contains(&new_id) {
                set.insert(new_id);
            } else {
                panic!("Check fails! Same id!");
            }
            times -= 1;
        }
        let end = Utc::now().timestamp_millis();
        println!("Single thread check time: {} ms", end - start);
    }
}
