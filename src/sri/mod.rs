use crate::{debug, info, success};
use alloc::string::{String, ToString};

use core::fmt;

struct SRI {
    protocol: String,
    path: String,
    fragment: String,
    query: String,
}
impl fmt::Display for SRI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}://{}#{}?{}",
            self.protocol, self.path, self.fragment, self.query
        )
    }
}

pub fn init() {
    info!("SRI interface loading");

    let url = SRI {
        protocol: "File".to_string(),
        path: "banner.txt".to_string(),
        fragment: "l1c2".to_string(),
        query: "read".to_string(),
    };
    debug!("{}", url);

    success!("SRI interface loaded")
}

#[test_case]
fn sri_test() {
    let _url = SRI {
        protocol: "test".to_string(),
        path: "test".to_string(),
        query: "read".to_string(),
    };
    // println!("{}", url);
}
