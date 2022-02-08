# IdGenerator

[![github-repo](https://img.shields.io/badge/github-BobAnkh/idgenerator-f5dc23?logo=github)](https://github.com/BobAnkh/idgenerator)
[![LICENSE Apache-2.0](https://img.shields.io/github/license/BobAnkh/idgenerator?logo=Apache&color=green)](https://github.com/BobAnkh/idgenerator/blob/main/LICENSE)

[![docs.rs](https://img.shields.io/badge/docs.rs-idgenerator-blue?logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/idgenerator)
[![crates.io](https://img.shields.io/crates/v/idgenerator.svg?logo=rust&color=orange)](https://crates.io/crates/idgenerator)
[![codecov](https://codecov.io/gh/BobAnkh/idgenerator/branch/main/graph/badge.svg?token=XD4EOZI5ZH)](https://codecov.io/gh/BobAnkh/idgenerator)

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
