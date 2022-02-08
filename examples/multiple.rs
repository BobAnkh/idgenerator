use idgenerator::*;
use std::time::Instant;

fn main() -> Result<(), OptionError> {
    let mut new_id: i64 = 0;
    let mut new_id_snd: i64 = 0;
    let mut times = 500000;
    // Setup the options for the id vec generator instance.
    let options = vec![
        IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6),
        IdGeneratorOptions::new().worker_id(2).worker_id_bit_len(6),
    ];

    // Initialize the id vec generator instance with the options.
    // Other options not set will be given the default value.
    let _ = IdVecInstance::init(options)?;

    // Get the options from the id vec generator instance of the index 0.
    let options = IdVecInstance::get_options(0).unwrap();
    println!("First setting of index 0: {:?}", options);

    // Get the options from the id vec generator instance of the index 1.
    let options = IdVecInstance::get_options(1).unwrap();
    println!("First setting of index 1: {:?}", options);

    // Setup another option
    let options = IdGeneratorOptions::new().seq_bit_len(12);
    // Use `set_options` will only change the options you have set.
    // Other options will not change if not set.
    // If new options are not compatible with the old options, it will return an error.
    //
    // Set index 0.
    let _ = IdVecInstance::set_options(0, options.clone())?;
    // Set index 1.
    let _ = IdVecInstance::set_options(1, options)?;

    // Get the option from the id generator instance to see what have change and what remains the same as you set first time.
    let options = IdVecInstance::get_options(0).unwrap();
    println!("Second setting of index 0: {:?}", options);
    let options = IdVecInstance::get_options(1).unwrap();
    println!("Second setting of index 1: {:?}", options);

    println!("Start to generate new unique id");
    let start = Instant::now();
    while times > 0 {
        // Call `next_id` to generate a new unique id.
        //
        // Generate unique id from the index 0 generator
        new_id = IdVecInstance::next_id(0);
        // Generate unique id from the index 1 generator
        new_id_snd = IdVecInstance::next_id(1);
        times -= 1;
    }

    let duration = start.elapsed();
    println!(
        "Program finished after {} seconds! Last id {}, {}",
        duration.as_secs(),
        new_id,
        new_id_snd,
    );
    Ok(())
}
