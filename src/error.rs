use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum OptionError {
    #[error("Invalid method.")]
    InvalidMethod,
    #[error("dInvalid base time.")]
    InvalidBaseTime,
    #[error("Invalid worker id: {0}.")]
    InvalidWorkerId(String),
    #[error("Invalid worker id bit length: {0}.")]
    InvalidWorkerIdBitLen(String),
    #[error("Invalid sequence bit length: {0}.")]
    InvalidSeqBitLen(String),
    #[error("Invalid max sequence number: {0}.")]
    InvalidMaxSeqNum(String),
    #[error("Invalid min sequence number: {0}.")]
    InvalidMinSeqNum(String),
    #[error("Invalid top over cost count.")]
    InvalidTopOverCostCount,
    #[error("Bit length overflow: {0}.")]
    BitLenOverflow(String),
    #[error("Invalid Vector length: {0}.")]
    InvalidVecLen(u32),
    #[error("Invalid index: {0}.")]
    IndexOutOfRange(usize),
}
