use crate::{info, success};
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
    info!("SRI interface loading");

    // let url = SRI {
    //     protocol: Protocol::File,
    //     path: "banner.txt",
    //     query: "read",
    // };
    // println!("> {}", url);
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
