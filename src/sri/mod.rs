use crate::{
    logger::{log, LogLevel},
    // println,
};
use core::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Protocol {
    /// References a folder/file on a `device://harddrive`
    File,
    /// System call
    Syscall,
    /// References a device connected to the computer
    Device,
}
struct SRI<'a> {
    protocol: Protocol,
    path: &'a str,
    query: &'a str,
}

impl<'a> fmt::Display for SRI<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}://{}?{}", self.protocol, self.path, self.query)
    }
}

pub fn init() {
    log(LogLevel::Info);
    // println!("SRI interface loading");

    let url = SRI {
        protocol: Protocol::File,
        path: "banner.txt",
        query: "read",
    };
    // println!("> {}", url);
    log(LogLevel::Success);
    // println!("SRI interface loaded");
}

#[test_case]
fn sri_test() {
    let url = SRI {
        protocol: Protocol::File,
        path: "test",
        query: "read",
    };
    // println!("{}", url);
}
