//! # Instance
//!
//! Provide two out-of-the-box implementations of id generators:
//!
//! - `IdInstance`: a instance with only one generator. See [examples/single.rs](https://github.com/BobAnkh/idgenerator/blob/main/examples/single.rs) for usage example.
//! - `IdVecInstance`: a instance with multiple generators. See [examples/multiple.rs](https://github.com/BobAnkh/idgenerator/blob/main/examples/multiple.rs) for usage example.

use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;

use crate::CoreIdGenerator;
use crate::IdGeneratorOptions;
use crate::OptionError;

/// Instance of only one generator
pub struct IdInstance;

impl IdInstance {
    /// Initialize the instance
    pub fn init(options: IdGeneratorOptions) -> Result<(), OptionError> {
        IdInstance::get_instance().lock().init(options)
    }

    /// Set instance options
    pub fn set_options(options: IdGeneratorOptions) -> Result<(), OptionError> {
        IdInstance::get_instance().lock().set_options(options)
    }

    /// Get instance options
    pub fn get_options() -> IdGeneratorOptions {
        IdInstance::get_instance().lock().get_options()
    }

    /// Get a unique id
    pub fn next_id() -> i64 {
        IdInstance::get_instance().lock().next_id()
    }

    fn get_instance() -> &'static Mutex<CoreIdGenerator> {
        static INSTANCE: OnceCell<Mutex<CoreIdGenerator>> = OnceCell::new();
        INSTANCE.get_or_init(|| Mutex::new(CoreIdGenerator::default()))
    }
}

/// Instance of multiple generators contained in a vector
pub struct IdVecInstance;

impl IdVecInstance {
    /// Initialize the instance
    ///
    /// Every time you call this function will drop all the previous generators in the instance.
    pub fn init(mut options: Vec<IdGeneratorOptions>) -> Result<(), OptionError> {
        if options.is_empty() {
            return Err(OptionError::InvalidVecLen(0));
        }
        let mut instances = IdVecInstance::get_instance().write();
        instances.clear();
        for option in options.drain(..) {
            let mut instance = CoreIdGenerator::default();
            instance.init(option)?;
            instances.push(Arc::new(Mutex::new(instance)));
        }
        Ok(())
    }

    /// Set instance options of the given index
    pub fn set_options(index: usize, options: IdGeneratorOptions) -> Result<(), OptionError> {
        let reader = {
            let r = IdVecInstance::get_instance().read();
            if index >= r.len() {
                return Err(OptionError::IndexOutOfRange(index));
            }
            Arc::clone(&r[index])
        };
        reader.lock().set_options(options)?;
        Ok(())
    }

    /// Get instance options of the given index
    pub fn get_options(index: usize) -> Result<IdGeneratorOptions, OptionError> {
        let reader = {
            let r = IdVecInstance::get_instance().read();
            if index >= r.len() {
                return Err(OptionError::IndexOutOfRange(index));
            }
            Arc::clone(&r[index])
        };
        let options = reader.lock().get_options();
        Ok(options)
    }

    /// Get a unique id
    pub fn next_id(index: usize) -> i64 {
        // Because this step matters the speed a lot,
        // so we won't check the index and let it panic
        let reader = {
            let r = IdVecInstance::get_instance().read();
            Arc::clone(&r[index])
        };
        let id = reader.lock().next_id();
        id
    }

    fn get_instance() -> &'static RwLock<Vec<Arc<Mutex<CoreIdGenerator>>>> {
        static INSTANCE: OnceCell<RwLock<Vec<Arc<Mutex<CoreIdGenerator>>>>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(Vec::new()))
    }
}
