use crate::idgen::*;

pub struct DefaultIdGenerator {
    pub worker: SnowWorkerM1,
}

impl DefaultIdGenerator {
    pub fn default() -> DefaultIdGenerator {
        DefaultIdGenerator { worker: SnowWorkerM1::default() }
    }
}
