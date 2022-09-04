use std::{process::Command, path::PathBuf, str::FromStr};

pub fn run_ensure_outputs(expectation: &str) {
    // either pick specific target dir (coverage) - or default at ../target
    let target_bin = option_env!("COVERAGE_BIN").unwrap_or("../target/debug/mycode");
    let target_bin = PathBuf::from_str(target_bin).expect("path");

    if !target_bin.exists() {
        panic!("no mycode binary built {:?}", target_bin)
    }

    let stderr = Command::new(target_bin)
        .env("LLVM_PROFILE_FILE", "traces/mycode.%p.profraw")
        .output()
        .expect("command ran")
        .stderr;

    let stderr = std::str::from_utf8(&stderr).expect("had stderr");
    if !stderr.contains(expectation) {
        panic!("==== FAIL: cant find `{}` in stderr\nstderr: {}", expectation, stderr);
    }
}
