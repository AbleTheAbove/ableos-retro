use crate::{
    logger::{log, LogLevel},
    println,
};

pub fn init() {
    log(LogLevel::Success);
    println!("URI interface loaded");
}
