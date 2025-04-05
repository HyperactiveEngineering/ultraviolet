use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!(
        "cargo:rustc-env=SOURCE_DATE_EPOCH={}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
}
