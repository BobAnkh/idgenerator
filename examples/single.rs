use idgenerator::*;
use std::time::Instant;

fn main() -> Result<(), OptionError> {
    let mut new_id: i64 = 0;
    let mut times = 500000;
    // Setup the option for the id generator instance.
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);

    // Initialize the id generator instance with the option.
    // Other options not set will be given the default value.
    let _ = IdInstance::init(options)?;

    // Get the option from the id generator instance.
    let options = IdInstance::get_options();
    println!("First setting: {:?}", options);

    // Setup another option
    let options = IdGeneratorOptions::new().seq_bit_len(12);
    // Use `set_options` will only change the options you have set.
    // Other options will not change if not set.
    // If new options are not compatible with the old options, it will return an error.
    let _ = IdInstance::set_options(options)?;

    // Get the option from the id generator instance to see what have change and what remains the same as you set first time.
    let options = IdInstance::get_options();
    println!("Second setting: {:?}", options);

    println!("Start to generate new unique id");
    let start = Instant::now();
    while times > 0 {
        // Call `next_id` to generate a new unique id.
        new_id = IdInstance::next_id();
        times -= 1;
    }
    let duration = start.elapsed();
    println!(
        "Program finished after {} seconds! Last id {}",
        duration.as_secs(),
        new_id
    );
    Ok(())
}
