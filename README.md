# IdGenerator

A powerful unique id generator.

**ATTENTION**: Now this crate has upgraded to **v2**, meaning most of the interfaces in **v1** is deprecated. If you still want to use **v1**, please see branch `v1`. You can also see [deprecated.md](deprecated.md) for how to use **v1**.

Shorter ID and faster generation with a new snowflake drift algorithm. The core is to shorten the ID length, but also can have a very high instantaneous concurrent processing capacity (about 50W/0.1s), and powerful configuration capacity.

If you want to have such a high throughput, please set a higher `seq_bit_len` (e.g. 10 or 12).

## Usage

### Out-of-the-box Instances implemented by this lib

This lib has implemented two out-of-the-box instances:

- `IdInstance`: a instance with only one generator. See [examples/single.rs](examples/single.rs) for usage example.
- `IdVecInstance`: a instance with multiple generators. See [examples/multiple.rs](examples/multiple.rs) for usage example.

You can also see how lib test works in [src/lib.rs](src/lib.rs) or how benchmark works in [bench/id_bench.rs](bench/id_bench.rs).

The steps mainly can be described as:

1. Setup the options(i.e. configure the instance).
2. Initialize the instance or set its options.
3. Call the `next_id` method to generate unique id.

What you can configure about the instances is demonstrated as the struct `IdGeneratorOptions`:

- `method`: 1 means snowflake with shift.
- `base_time`: base time of the snowflake algorithm, in milliseconds, can not exceed the current system time.
- `worker_id`: should be decided externally, smaller than `2^worker_id_bit_len-1`.
- `worker_id_bit_len`: the bit length of worker_id, default to 8, in range \[1, 15\]. **`worker_id_bit_len + seq_bit_len` should be less than 22**.
- `seq_bit_len`: the bit length of sequence, default to 8, in range \[3, 21\].
- `max_seq_num`: set the range of \[min_seq_num, 2^seq_bit_len-1\], default to 0 meaning `2^seq_bit_len-1`.
- `min_seq_num`: default to 5, range \[5, max_seq_num\], reserved for manually value and time turned back.
- `top_over_cost_count`: max shift count(included), default to 2000, recommended range is [500, 20000] (associated with computing ability).

A very simple example:

```rust
use idgenerator::*;

fn main() {
    // Setup the option for the id generator instance.
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    // Initialize the id generator instance with the option.
    // Other options not set will be given the default value.
    let _ = IdInstance::init(options)?;
    // Call `next_id` to generate a new unique id.
    let id = IdInstance::next_id();
    println!("id is {}", id);
}
```

For more complex usage, see directory `examples`.

### Self-implement Instances

This lib wraps the snowflake algorithm inside the `CoreIdGenerator` struct. You can wrap this struct inside your own instance of unique id generator.

For more details, please refer to [Documentation](https://docs.rs/idgenerator).

## Credits

- Inspired by [yitter/IdGenerator](https://github.com/yitter/IdGenerator)
