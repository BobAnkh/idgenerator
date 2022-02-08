# Deprecated v1

## Simple Usage example

First, **global** initialize:

```rust
use idgenerator::{IdHelper, IdGeneratorOptions};
// Create a instance
IdHelper::init();
// Create IdGeneratorOptions, worker_id is the only parameter needed：
let mut options = IdGeneratorOptions::new(1);
// If you want to have a larger work_id range, set worker_id_bit_len to a larger number
options.worker_id_bit_len = 8; // default to 8, meaning the max number of work_id is 2^8 - 1
// Other options can be seen in IdGeneratorOptions

// You must save parameters before generating
IdHelper::set_id_generator(options);
```

Second, generate ID:

```rust
use idgenerator::IdHelper;
// call next_id() at where you want to generate the ID
let new_id: i64 = IdHelper::next_id();
println!("ID: {}", new_id);
```
