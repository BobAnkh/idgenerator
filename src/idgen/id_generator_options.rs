pub struct IdGeneratorOptions {
    /// Snowflake 1 for shift
    pub method: u8,

    /// base time (in milliseconds), can not exceed the current system time\
    pub base_time: i64,

    /// should be decided externally, smaller than 2^worker_id_bit_len-1
    pub worker_id: u32,

    /// worker_id_bit_len + seq_bit_len should be less than 22
    /// default to 8, in range [1, 15]
    pub worker_id_bit_len: u8,

    /// default to 8, in range [3, 21]
    pub seq_bit_len: u8,

    /// set the range of [min_seq_num, 2^seq_bit_len-1], default to 0 meaning 2^seq_bit_len-1
    pub max_seq_num: u32,

    /// default to 5, range [5, max_seq_num], reserved for manually value and time turned back
    pub min_seq_num: u32,

    /// max shift count(included), default to 2000, recommended range is [500, 20000] (associated with computing ability)
    pub top_over_cost_count: u32,
}

impl IdGeneratorOptions {
    pub fn new(worker_id: u32) -> IdGeneratorOptions {
        return IdGeneratorOptions {
            method: 1,
            worker_id: worker_id,
            base_time: 1582136402000,
            worker_id_bit_len: 8,
            seq_bit_len: 8,
            max_seq_num: 0,
            min_seq_num: 5,
            top_over_cost_count: 2000,
        };
    }
}