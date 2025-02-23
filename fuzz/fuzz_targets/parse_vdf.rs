#![no_main]

use libfuzzer_sys::fuzz_target;
use steamkit::vdf;

fuzz_target!(|data: &str| {
    let _ = vdf::from_str(&data, &vdf::Options::default());
});
