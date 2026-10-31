use super::super::bench::read_run;
use std::process::Output;

fn out(stdout: &str, code: i32) -> Output {
    use std::os::unix::process::ExitStatusExt;
    Output {
        status: std::process::ExitStatus::from_raw(code << 8),
        stdout: stdout.as_bytes().to_vec(),
        stderr: Vec::new(),
    }
}

/// A run's failing tests are named and its passing ones counted.
#[test]
fn read_run_names_failures_and_counts_passes() {
    let o = out(
        "running 3 tests\ntest a::b ... ok\ntest a::c ... FAILED\ntest a::d ... ok\n\nfailures:\n    a::c\n",
        101,
    );
    assert_eq!(read_run(&o), (vec!["a::c".to_string()], 2));
}
