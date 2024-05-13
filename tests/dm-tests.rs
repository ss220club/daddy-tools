use std::process::{Command, Output};

#[cfg(feature = "text")]
#[test]
fn text() {
    run_dm_tests("text");
}

fn run_dm_tests(name: &str) {
    std::env::remove_var("RUST_BACKTRACE");

    let byond_bin = std::env::var("BYOND_BIN").expect("environment variable BYOND_BIN");
    let byondexec = format!("{}/byondexec", byond_bin);
    let dream_maker = format!("{}/DreamMaker", byond_bin);
    let dream_daemon = format!("{}/DreamDaemon", byond_bin);

    let dme = format!("tests/dm/{}.dme", name);
    let dmb = format!("tests/dm/{}.dmb", name);

    let target_dir = if cfg!(target_os = "linux") {
        "i686-unknown-linux-gnu"
    } else {
        "i686-pc-windows-gnu"
    };
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let fname = if cfg!(target_os = "linux") {
        "librust_utils.so"
    } else {
        "rust_utils.dll"
    };
    let rust_utils = format!("target/{}/{}/{}", target_dir, profile, fname);

    let output = Command::new("bash")
        .arg(&byondexec)
        .arg(&dream_maker)
        .arg(&dme)
        .output()
        .unwrap();
    dump(&output);
    generic_check(&output);

    let output = Command::new("bash")
        .arg(&byondexec)
        .arg(&dream_daemon)
        .arg(&dmb)
        .arg("-trusted")
        .arg("-cd")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .env("RUST_UTILS", &rust_utils)
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&dmb);
    dump(&output);
    generic_check(&output);
    runtime_check(&output);
}

fn dump(output: &Output) {
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

fn generic_check(output: &Output) {
    if !output.status.success() {
        panic!("process exited with {:?}", output.status);
    }
}

fn runtime_check(output: &Output) {
    for line in output.stderr.split(|&c| c == b'\n') {
        if line.starts_with(b"runtime error: ") {
            panic!("{}", String::from_utf8_lossy(line));
        }
    }
}
