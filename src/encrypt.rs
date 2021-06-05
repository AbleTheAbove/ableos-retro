use crate::{
    success
};

pub fn aes_detect() -> bool {
    if core_detect::is_x86_feature_detected!("aes") {
        success!("AES is available");
        true
    } else {
        // log(LogLevel::Debug);
        // println!("AES is not available");
        false
    }
}
