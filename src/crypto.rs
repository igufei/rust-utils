use crypto::{digest::Digest, sha1::Sha1};

pub fn sha1(input_str: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input_str);
    let hex = hasher.result_str();
    hex
}
