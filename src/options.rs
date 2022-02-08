//! # IdGeneratorOptions
//!
//! IdGeneratorOptions will provide you a interface for setting generators' options

/// Options for CoreIdGenerator
///
/// ## Parameters
///
/// - `method`: 1 means snowflake with shift.
/// - `base_time`: base time of the snowflake algorithm, in milliseconds, can not exceed the current system time.
/// - `worker_id`: should be decided externally, smaller than `2^worker_id_bit_len-1`.
/// - `worker_id_bit_len`: the bit length of worker_id, default to 8, in range \[1, 15\]. **`worker_id_bit_len + seq_bit_len` should be less than 22**.
/// - `seq_bit_len`: the bit length of sequence, default to 8, in range \[3, 21\].
/// - `max_seq_num`: set the range of \[min_seq_num, 2^seq_bit_len-1\], default to 0 meaning `2^seq_bit_len-1`.
/// - `min_seq_num`: default to 5, range \[5, max_seq_num\], reserved for manually value and time turned back.
/// - `top_over_cost_count`: max shift count(included), default to 2000, recommended range is [500, 20000] (associated with computing ability).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IdGeneratorOptions {
    /// Snowflake 1 for shift
    pub method: Option<u8>,

    /// base time (in milliseconds), can not exceed the current system time
    pub base_time: Option<i64>,

    /// should be decided externally, smaller than `2^worker_id_bit_len-1`
    pub worker_id: Option<u32>,

    /// `worker_id_bit_len + seq_bit_len` should be less than 22
    /// default to 8, in range \[1, 15\]
    pub worker_id_bit_len: Option<u8>,

    /// default to 8, in range \[3, 21\]
    pub seq_bit_len: Option<u8>,

    /// set the range of \[min_seq_num, 2^seq_bit_len-1\], default to 0 meaning `2^seq_bit_len-1`
    pub max_seq_num: Option<u32>,

    /// default to 5, range [5, max_seq_num], reserved for manually value and time turned back
    pub min_seq_num: Option<u32>,

    /// max shift count(included), default to 2000, recommended range is [500, 20000] (associated with computing ability)
    pub top_over_cost_count: Option<u32>,
}

impl IdGeneratorOptions {
    pub fn new() -> Self {
        IdGeneratorOptions {
            method: None,
            base_time: None,
            worker_id: None,
            worker_id_bit_len: None,
            seq_bit_len: None,
            max_seq_num: None,
            min_seq_num: None,
            top_over_cost_count: None,
        }
    }

    pub fn method(mut self, method: u8) -> Self {
        self.method = Some(method);
        self
    }

    pub fn base_time(mut self, base_time: i64) -> Self {
        self.base_time = Some(base_time);
        self
    }

    pub fn worker_id(mut self, worker_id: u32) -> Self {
        self.worker_id = Some(worker_id);
        self
    }

    pub fn worker_id_bit_len(mut self, worker_id_bit_len: u8) -> Self {
        self.worker_id_bit_len = Some(worker_id_bit_len);
        self
    }

    pub fn seq_bit_len(mut self, seq_bit_len: u8) -> Self {
        self.seq_bit_len = Some(seq_bit_len);
        self
    }

    pub fn max_seq_num(mut self, max_seq_num: u32) -> Self {
        self.max_seq_num = Some(max_seq_num);
        self
    }

    pub fn min_seq_num(mut self, min_seq_num: u32) -> Self {
        self.min_seq_num = Some(min_seq_num);
        self
    }

    pub fn top_over_cost_count(mut self, top_over_cost_count: u32) -> Self {
        self.top_over_cost_count = Some(top_over_cost_count);
        self
    }
}
