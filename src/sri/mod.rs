use crate::{debug, info, success};
use alloc::string::{String, ToString};

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
struct SRI {
    protocol: Protocol,
    path: String,
    query: String,
}
impl fmt::Display for SRI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}://{}?{}", self.protocol, self.path, self.query)
    }
}

pub fn init() {
    info!("SRI interface loading");

    let url = SRI {
        protocol: Protocol::File,
        path: "banner.txt".to_string(),
        query: "read".to_string(),
    };
    debug!("{}", url);
    success!("SRI interface loaded")
}

#[test_case]
fn sri_test() {
    let _url = SRI {
        protocol: Protocol::File,
        path: "test",
        query: "read",
    };
    // println!("{}", url);
}
