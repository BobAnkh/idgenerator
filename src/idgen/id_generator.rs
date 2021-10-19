use crate::idgen::*;
use chrono::Utc;
use std::thread::sleep;

pub struct DefaultIdGenerator {
    pub worker: SnowWorker,
}

impl DefaultIdGenerator {
    pub fn default() -> DefaultIdGenerator {
        DefaultIdGenerator { worker: SnowWorker::default() }
    }
}

pub struct SnowWorker {
    /// Base time
    pub base_time: i64,
    /// Machine code
    pub worker_id: u32,
    /// Length of machine code length(in bit)
    pub worker_id_bit_len: u8,
    /// Length of the self-increment sequence(int bit)
    pub seq_bit_len: u8,
    /// The max sequence number(included)
    pub max_seq_num: u32,
    /// The min sequence number(included)
    pub min_seq_num: u32,
    /// The max shift count
    pub top_over_cost_count: u32,

    _timestamp_shift: u8,
    _current_seq_number: u32,
    _last_time_tick: i64,
    _turn_back_time_tick: i64,
    _turn_back_index: u8,
    _is_over_cost: bool,
    _over_cost_count_in_one_term: u32,
    _gen_count_in_one_term: u32,
    _term_index: u32,
}

impl SnowWorker {
    pub fn default() -> SnowWorker {
        let options = IdGeneratorOptions::new(1);
        return SnowWorker::new(options);
    }

    pub fn set_options(&mut self, options: IdGeneratorOptions) {
        // 1. BaseTime
        if options.base_time == 0 {
            self.base_time = 1582136402000;
        } else if options.base_time < 631123200000
            || options.base_time > Utc::now().timestamp_millis()
        {
            panic!("[ERROR]: Base time error.");
        } else {
            self.base_time = options.base_time;
        }

        // 2.WorkerIdBitLength
        if options.worker_id_bit_len <= 0 {
            panic!("[ERROR]: worker_id_bit_len error.(range:[1, 21])");
        }
        if options.seq_bit_len + options.worker_id_bit_len > 22 {
            panic!("[ERROR]: worker_id_bit_len + seq_bit_len <= 22");
        } else {
            self.worker_id_bit_len = if options.worker_id_bit_len <= 0 {
                6
            } else {
                options.worker_id_bit_len
            };
        }

        // 3.WorkerId
        let mut max_worker_id_number = (1 << options.worker_id_bit_len) - 1;
        if max_worker_id_number == 0 {
            max_worker_id_number = 63;
        }
        if options.worker_id > max_worker_id_number {
            panic!("[ERROR]: worker_id error. (range:[0, {} ]", max_worker_id_number);
        } else {
            self.worker_id = options.worker_id;
        }

        // 4.SeqBitLength
        if options.seq_bit_len < 2 || options.seq_bit_len > 21 {
            panic!("[ERROR]: seq_bit_len error. (range:[2, 21])");
        } else {
            self.seq_bit_len = if options.seq_bit_len <= 0 {
                6
            } else {
                options.seq_bit_len
            };
        }

        // 5.MaxSeqNumber
        let mut max_seq_number = (1 << options.seq_bit_len) - 1;
        if max_seq_number == 0 {
            max_seq_number = 63;
        }
        if options.max_seq_num > max_seq_number {
            panic!("[ERROR]: max_seq_num error. (range:[1, {}]", max_seq_number);
        } else {
            self.max_seq_num = if options.max_seq_num == 0 {
                max_seq_number
            } else {
                options.max_seq_num
            };
        }

        // 6.MinSeqNumber
        if options.min_seq_num < 5 || options.min_seq_num > max_seq_number {
            panic!("[ERROR]: min_seq_num error. (range:[5, {}]", max_seq_number);
        } else {
            self.min_seq_num = options.min_seq_num;
        }

        // 7.Others
        self.top_over_cost_count = if options.top_over_cost_count == 0 {
            2000
        } else {
            options.top_over_cost_count
        };
        self._timestamp_shift = self.worker_id_bit_len + self.seq_bit_len;
        self._current_seq_number = self.min_seq_num;

        if options.method == 1 {
            sleep(std::time::Duration::from_millis(500));
        }
    }

    pub fn new(options: IdGeneratorOptions) -> SnowWorker {
        let mut worker = SnowWorker {
            base_time: 1582136402000,
            worker_id_bit_len: 0,
            worker_id: 0,
            seq_bit_len: 0,
            max_seq_num: 0,
            min_seq_num: 0,
            top_over_cost_count: 0,
            _timestamp_shift: 0,
            _current_seq_number: 0,

            _last_time_tick: 0,
            _turn_back_time_tick: 0,
            _turn_back_index: 0,
            _is_over_cost: false,
            _over_cost_count_in_one_term: 0,
            _gen_count_in_one_term: 0,
            _term_index: 0,
        };

        worker.set_options(options);
        return worker;
    }

    pub fn next_id(&mut self) -> i64 {
        if self._is_over_cost {
            self.next_over_cost_id()
        } else {
            self.next_normal_id()
        }
    }

    fn begin_over_cost_action(&self, _use_time_tick: i64) {}

    fn end_over_cost_action(&mut self, _use_time_tick: i64) {
        if self._term_index > 10000 {
            self._term_index = 0;
        }
    }

    fn begin_turn_back_action(&self, _use_time_tick: i64) {}

    fn end_turn_back_action(&self, _use_time_tick: i64) {}

    fn next_over_cost_id(&mut self) -> i64 {
        let current_time_tick = self.get_current_time_tick();

        if current_time_tick > self._last_time_tick {
            self.end_over_cost_action(current_time_tick);

            self._last_time_tick = current_time_tick;
            self._current_seq_number = self.min_seq_num;
            self._is_over_cost = false;
            self._over_cost_count_in_one_term = 0;
            self._gen_count_in_one_term = 0;

            return self.calc_id(self._last_time_tick);
        }

        if self._over_cost_count_in_one_term >= self.top_over_cost_count {
            self.end_over_cost_action(current_time_tick);

            self._last_time_tick = self.get_next_time_tick();
            self._current_seq_number = self.min_seq_num;
            self._is_over_cost = false;
            self._over_cost_count_in_one_term = 0;
            self._gen_count_in_one_term = 0;

            return self.calc_id(self._last_time_tick);
        }

        if self._current_seq_number > self.max_seq_num {
            self._last_time_tick += 1;
            self._current_seq_number = self.min_seq_num;
            self._is_over_cost = true;
            self._over_cost_count_in_one_term += 1;
            self._gen_count_in_one_term += 1;

            return self.calc_id(self._last_time_tick);
        }

        self._gen_count_in_one_term += 1;
        return self.calc_id(self._last_time_tick);
    }

    fn next_normal_id(&mut self) -> i64 {
        let current_time_tick = self.get_current_time_tick();

        if current_time_tick < self._last_time_tick {
            if self._turn_back_time_tick < 1 {
                self._turn_back_time_tick = self._last_time_tick - 1;
                self._turn_back_index += 1;

                // The first 5 bits of sequence number in one millisecond is reserved
                // The index 0 for manually new number, index 1-4 for time turned back
                if self._turn_back_index > 4 {
                    self._turn_back_index = 1;
                }
                self.begin_turn_back_action(self._turn_back_time_tick);
            }

            // thread::sleep(std::time::Duration::from_millis(1));
            return self.calc_turn_back_id(self._turn_back_time_tick);
        }

        // If the time is caught up, reset _turn_back_time_tick to zero
        if self._turn_back_time_tick > 0 {
            self.end_turn_back_action(self._turn_back_time_tick);
            self._turn_back_time_tick = 0;
        }

        if current_time_tick > self._last_time_tick {
            self._last_time_tick = current_time_tick;
            self._current_seq_number = self.min_seq_num;

            return self.calc_id(self._last_time_tick);
        }

        if self._current_seq_number > self.max_seq_num {
            self.begin_over_cost_action(current_time_tick);

            self._term_index += 1;
            self._last_time_tick += 1;
            self._current_seq_number = self.min_seq_num;
            self._is_over_cost = true;
            self._over_cost_count_in_one_term = 1;
            self._gen_count_in_one_term = 1;

            return self.calc_id(self._last_time_tick);
        }

        return self.calc_id(self._last_time_tick);
    }

    fn calc_id(&mut self, use_time_tick: i64) -> i64 {
        let result = (use_time_tick << self._timestamp_shift)
            + (self.worker_id << self.seq_bit_len) as i64
            + (self._current_seq_number) as i64;
        self._current_seq_number += 1;
        return result;
    }

    fn calc_turn_back_id(&mut self, use_time_tick: i64) -> i64 {
        let result = (use_time_tick << self._timestamp_shift)
            + (self.worker_id << self.seq_bit_len) as i64
            + (self._turn_back_index) as i64;
        self._turn_back_time_tick -= 1;
        return result;
    }

    fn get_current_time_tick(&self) -> i64 {
        return Utc::now().timestamp_millis() - self.base_time;
    }

    fn get_next_time_tick(&self) -> i64 {
        let mut temp_time_ticker = self.get_current_time_tick();

        while temp_time_ticker <= self._last_time_tick {
            temp_time_ticker = self.get_current_time_tick();
        }

        return temp_time_ticker;
    }
}
