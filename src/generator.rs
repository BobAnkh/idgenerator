//! # Generator
//!
//! Core of the unique id generator
//!
//! Contains the implementation of the snowflake algorithm and the wrapper as `CoreIdGenerator`

use crate::IdGeneratorOptions;
use chrono::Utc;
use std::thread::sleep;

use super::error::OptionError;

/// Wrapper of the snowflake algorithm worker.
/// Provide public interfaces to initialize a generator, set options, get options and get a unique id.
#[derive(Debug, Default)]
pub struct CoreIdGenerator {
    worker: SnowFlake,
}

impl CoreIdGenerator {
    pub fn init(&mut self, options: IdGeneratorOptions) -> Result<(), OptionError> {
        let instance_options: InstanceOptions = options.into();
        self.worker.init_options(instance_options)
    }

    pub fn get_options(&self) -> IdGeneratorOptions {
        self.worker.get_options()
    }

    pub fn set_options(&mut self, options: IdGeneratorOptions) -> Result<(), OptionError> {
        self.worker.set_options(options)
    }

    pub fn next_id(&mut self) -> i64 {
        self.worker.next_id()
    }
}

/// The options interact with the snowflake worker inside the CoreIdGenerator
#[derive(Debug, Clone)]
struct InstanceOptions {
    pub method: u8,
    pub base_time: i64,
    pub worker_id: u32,
    pub worker_id_bit_len: u8,
    pub seq_bit_len: u8,
    pub max_seq_num: u32,
    pub min_seq_num: u32,
    pub top_over_cost_count: u32,
}

impl From<IdGeneratorOptions> for InstanceOptions {
    fn from(options: IdGeneratorOptions) -> Self {
        InstanceOptions {
            method: options.method.unwrap_or(1),
            base_time: options.base_time.unwrap_or(1582136402000),
            worker_id: options.worker_id.unwrap_or(0),
            worker_id_bit_len: options.worker_id_bit_len.unwrap_or(8),
            seq_bit_len: options.seq_bit_len.unwrap_or(8),
            max_seq_num: options.max_seq_num.unwrap_or(0),
            min_seq_num: options.min_seq_num.unwrap_or(5),
            top_over_cost_count: options.top_over_cost_count.unwrap_or(2000),
        }
    }
}

impl Default for InstanceOptions {
    fn default() -> Self {
        InstanceOptions {
            method: 1,
            base_time: 1582136402000,
            worker_id: 0,
            worker_id_bit_len: 8,
            seq_bit_len: 8,
            max_seq_num: 0,
            min_seq_num: 5,
            top_over_cost_count: 2000,
        }
    }
}

/// The inner worker implementing the snowflake algorithm
#[derive(Debug)]
struct SnowFlake {
    // external options
    /// Method
    method: u8,
    /// Base time
    base_time: i64,
    /// Machine code
    worker_id: u32,
    /// Length of machine code length(in bit)
    worker_id_bit_len: u8,
    /// Length of the self-increment sequence(int bit)
    seq_bit_len: u8,
    /// The max sequence number(included)
    max_seq_num: u32,
    /// The min sequence number(included)
    min_seq_num: u32,
    /// The max shift count
    top_over_cost_count: u32,

    // inner variables
    timestamp_shift: u8,
    current_seq_number: u32,
    last_time_tick: i64,
    turn_back_time_tick: i64,
    turn_back_index: u8,
    is_over_cost: bool,
    over_cost_count_in_one_term: u32,
    gen_count_in_one_term: u32,
    term_index: u32,
}

impl Default for SnowFlake {
    fn default() -> SnowFlake {
        let options = InstanceOptions::default();
        let mut worker = SnowFlake {
            method: 1,
            base_time: 1582136402000,
            worker_id_bit_len: 0,
            worker_id: 0,
            seq_bit_len: 0,
            max_seq_num: 0,
            min_seq_num: 0,
            top_over_cost_count: 0,
            timestamp_shift: 0,
            current_seq_number: 0,

            last_time_tick: 0,
            turn_back_time_tick: 0,
            turn_back_index: 0,
            is_over_cost: false,
            over_cost_count_in_one_term: 0,
            gen_count_in_one_term: 0,
            term_index: 0,
        };
        worker.init_options(options).unwrap();
        worker
    }
}

impl SnowFlake {
    pub fn init_options(&mut self, mut options: InstanceOptions) -> Result<(), OptionError> {
        self.check_options(&mut options)?;
        self.set_instance_options(options);
        Ok(())
    }

    pub fn get_options(&self) -> IdGeneratorOptions {
        IdGeneratorOptions {
            method: Some(self.method),
            base_time: Some(self.base_time),
            worker_id: Some(self.worker_id),
            worker_id_bit_len: Some(self.worker_id_bit_len),
            seq_bit_len: Some(self.seq_bit_len),
            max_seq_num: Some(self.max_seq_num),
            min_seq_num: Some(self.min_seq_num),
            top_over_cost_count: Some(self.top_over_cost_count),
        }
    }

    pub fn set_options(&mut self, options: IdGeneratorOptions) -> Result<(), OptionError> {
        let mut instance_options: InstanceOptions = InstanceOptions {
            method: options.method.unwrap_or(self.method),
            base_time: options.base_time.unwrap_or(self.base_time),
            worker_id: options.worker_id.unwrap_or(self.worker_id),
            worker_id_bit_len: options.worker_id_bit_len.unwrap_or(self.worker_id_bit_len),
            seq_bit_len: options.seq_bit_len.unwrap_or(self.seq_bit_len),
            max_seq_num: options.max_seq_num.unwrap_or(self.max_seq_num),
            min_seq_num: options.min_seq_num.unwrap_or(self.min_seq_num),
            top_over_cost_count: options
                .top_over_cost_count
                .unwrap_or(self.top_over_cost_count),
        };
        self.check_options(&mut instance_options)?;
        self.set_instance_options(instance_options);
        Ok(())
    }

    pub fn next_id(&mut self) -> i64 {
        if self.is_over_cost {
            self.next_over_cost_id()
        } else {
            self.next_normal_id()
        }
    }

    fn check_options(&self, options: &mut InstanceOptions) -> Result<(), OptionError> {
        // 1. Check base time
        if options.base_time == 0 {
            options.base_time = 1582136402000;
        } else if options.base_time < 631123200000
            || options.base_time > Utc::now().timestamp_millis()
        {
            return Err(OptionError::InvalidBaseTime);
        }

        // 2. Check worker id bit length
        if options.worker_id_bit_len == 0 {
            return Err(OptionError::InvalidWorkerIdBitLen(
                "should have worker_id_bit_len in range [1, 21]".to_string(),
            ));
        } else if options.seq_bit_len + options.worker_id_bit_len > 22 {
            return Err(OptionError::BitLenOverflow(
                "should have worker_id_bit_len + seq_bit_len <= 22".to_string(),
            ));
        }

        // 3. Check worker id
        let mut max_worker_id_number = (1 << options.worker_id_bit_len) - 1;
        if max_worker_id_number == 0 {
            max_worker_id_number = 63;
        }
        if options.worker_id > max_worker_id_number {
            return Err(OptionError::InvalidWorkerId(format!(
                "should in range [0, {max_worker_id_number}]"
            )));
        }

        // 4. Check sequence bit length
        if options.seq_bit_len < 2 || options.seq_bit_len > 21 {
            return Err(OptionError::InvalidSeqBitLen(
                "should have seq_bit_len in range [2, 21]".to_string(),
            ));
        }

        // 5. Check max sequence number
        let mut max_seq_number = (1 << options.seq_bit_len) - 1;
        if max_seq_number == 0 {
            max_seq_number = 63;
        }
        if options.max_seq_num > max_seq_number {
            return Err(OptionError::InvalidMaxSeqNum(format!(
                "should in range [1, {max_seq_number}]"
            )));
        } else if options.max_seq_num == 0 {
            options.max_seq_num = max_seq_number
        }

        // 6. Checkk min sequence number
        if options.min_seq_num < 5 || options.min_seq_num > max_seq_number {
            return Err(OptionError::InvalidMinSeqNum(format!(
                "should in range [5, {max_seq_number}]"
            )));
        }

        // 7. Check top over cost count
        if options.top_over_cost_count == 0 {
            options.top_over_cost_count = 2000;
        }

        Ok(())
    }

    fn set_instance_options(&mut self, options: InstanceOptions) {
        self.method = options.method;
        self.base_time = options.base_time;
        self.worker_id_bit_len = options.worker_id_bit_len;
        self.worker_id = options.worker_id;
        self.seq_bit_len = options.seq_bit_len;
        self.max_seq_num = options.max_seq_num;
        self.min_seq_num = options.min_seq_num;
        self.top_over_cost_count = options.top_over_cost_count;

        self.timestamp_shift = self.worker_id_bit_len + self.seq_bit_len;
        self.current_seq_number = self.min_seq_num;
        if self.method == 1 {
            sleep(std::time::Duration::from_millis(500));
        }
    }

    fn begin_over_cost_action(&self, _use_time_tick: i64) {}

    fn end_over_cost_action(&mut self, _use_time_tick: i64) {
        if self.term_index > 10000 {
            self.term_index = 0;
        }
    }

    fn begin_turn_back_action(&self, _use_time_tick: i64) {}

    fn end_turn_back_action(&self, _use_time_tick: i64) {}

    fn next_over_cost_id(&mut self) -> i64 {
        let current_time_tick = self.get_current_time_tick();

        if current_time_tick > self.last_time_tick {
            self.end_over_cost_action(current_time_tick);
            self.last_time_tick = current_time_tick;
            self.current_seq_number = self.min_seq_num;
            self.is_over_cost = false;
            self.over_cost_count_in_one_term = 0;
            self.gen_count_in_one_term = 0;
            self.calc_id(self.last_time_tick)
        } else if self.over_cost_count_in_one_term >= self.top_over_cost_count {
            self.end_over_cost_action(current_time_tick);
            self.last_time_tick = self.get_next_time_tick();
            self.current_seq_number = self.min_seq_num;
            self.is_over_cost = false;
            self.over_cost_count_in_one_term = 0;
            self.gen_count_in_one_term = 0;
            self.calc_id(self.last_time_tick)
        } else if self.current_seq_number > self.max_seq_num {
            self.last_time_tick += 1;
            self.current_seq_number = self.min_seq_num;
            self.is_over_cost = true;
            self.over_cost_count_in_one_term += 1;
            self.gen_count_in_one_term += 1;
            self.calc_id(self.last_time_tick)
        } else {
            self.gen_count_in_one_term += 1;
            self.calc_id(self.last_time_tick)
        }
    }

    fn next_normal_id(&mut self) -> i64 {
        let current_time_tick = self.get_current_time_tick();

        if current_time_tick < self.last_time_tick {
            if self.turn_back_time_tick < 1 {
                self.turn_back_time_tick = self.last_time_tick - 1;
                self.turn_back_index += 1;
                // The first 5 bits of sequence number in one millisecond is reserved
                // The index 0 for manually new number, index 1-4 for time turned back
                if self.turn_back_index > 4 {
                    self.turn_back_index = 1;
                }
                self.begin_turn_back_action(self.turn_back_time_tick);
            }
            return self.calc_turn_back_id(self.turn_back_time_tick);
        }

        // If the time is caught up, reset _turn_back_time_tick to zero
        if self.turn_back_time_tick > 0 {
            self.end_turn_back_action(self.turn_back_time_tick);
            self.turn_back_time_tick = 0;
        }

        if current_time_tick > self.last_time_tick {
            self.last_time_tick = current_time_tick;
            self.current_seq_number = self.min_seq_num;
            self.calc_id(self.last_time_tick)
        } else if self.current_seq_number > self.max_seq_num {
            self.begin_over_cost_action(current_time_tick);
            self.term_index += 1;
            self.last_time_tick += 1;
            self.current_seq_number = self.min_seq_num;
            self.is_over_cost = true;
            self.over_cost_count_in_one_term = 1;
            self.gen_count_in_one_term = 1;
            self.calc_id(self.last_time_tick)
        } else {
            self.calc_id(self.last_time_tick)
        }
    }

    fn calc_id(&mut self, use_time_tick: i64) -> i64 {
        let result = (use_time_tick << self.timestamp_shift)
            + (self.worker_id << self.seq_bit_len) as i64
            + (self.current_seq_number) as i64;
        self.current_seq_number += 1;
        result
    }

    fn calc_turn_back_id(&mut self, use_time_tick: i64) -> i64 {
        let result = (use_time_tick << self.timestamp_shift)
            + (self.worker_id << self.seq_bit_len) as i64
            + (self.turn_back_index) as i64;
        self.turn_back_time_tick -= 1;
        result
    }

    fn get_current_time_tick(&self) -> i64 {
        Utc::now().timestamp_millis() - self.base_time
    }

    fn get_next_time_tick(&self) -> i64 {
        let mut temp_time_ticker = self.get_current_time_tick();
        while temp_time_ticker <= self.last_time_tick {
            temp_time_ticker = self.get_current_time_tick();
        }
        temp_time_ticker
    }
}
