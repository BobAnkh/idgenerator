# IdGenerator

Modified from [yitter/IdGenerator](https://github.com/yitter/IdGenerator). Please see it for original documentation.

## Usage example

First, **global** initialize:

```rust
// Create IdGeneratorOptions, worker_id is the only parameter needed：
let mut options = IdGeneratorOptions::new(1);
// If you want to have a larger work_id range, set worker_id_bit_len to a larger number
options.worker_id_bit_len = 6; // default to 6, meaning the max number of work_id is 2^6 - 1
// Other options can be seen in IdGeneratorOptions

// You must save parameters before generating
YitIdHelper::set_id_generator(options);
```

Second, generate ID:

```rust
// call next_id() at where you want to generate the ID
let new_id: i64 = YitIdHelper::next_id();
```
