use std::process::Command;

/// Path to the workspace root, resolved at compile time from the cli crate's
/// manifest directory.
fn workspace_root() -> String {
    // cli/Cargo.toml is at <root>/cli/Cargo.toml, so .. gets us to the root.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

/// Run `cargo run -p cli -- report --manifest-path <fixture>` and return
/// (exit_success, stdout, stderr).
fn run_profiler(fixture_name: &str) -> (bool, String, String) {
    let root = workspace_root();
    let fixture_path = format!("{root}/tests/fixtures/{fixture_name}");

    let output = Command::new("cargo")
        .args([
            "run", "-p", "cli", "--", "report", "--manifest-path", &fixture_path,
        ])
        .current_dir(&root)
        .output()
        .expect("failed to spawn cargo");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.success(), stdout, stderr)
}

// ---------------------------------------------------------------------------
// Happy path: normal function, profiler produces output
// ---------------------------------------------------------------------------

#[test]
fn happy_path_produces_records() {
    let (success, stdout, stderr) = run_profiler("happy-path-contract");

    assert!(success, "CLI should succeed on happy-path contract\nstderr: {stderr}");

    // The profiler should find exactly 2 cost records (add + noop).
    assert!(
        stdout.contains("collected 2 cost record(s)"),
        "expected 2 cost records, stdout:\n{stdout}"
    );

    // Both labels should appear in the output.
    assert!(stdout.contains("add"), "missing 'add' label\nstdout: {stdout}");
    assert!(stdout.contains("noop"), "missing 'noop' label\nstdout: {stdout}");
}

#[test]
fn happy_path_html_report() {
    let root = workspace_root();
    let fixture_path = format!("{root}/tests/fixtures/happy-path-contract");
    let html_path = format!("{root}/target/test-reports/happy-path.html");

    // Ensure the output directory exists.
    std::fs::create_dir_all(format!("{root}/target/test-reports")).unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cli",
            "--",
            "report",
            "--manifest-path",
            &fixture_path,
            "--html",
            &html_path,
        ])
        .current_dir(&root)
        .output()
        .expect("failed to spawn cargo");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        output.status.success(),
        "CLI should succeed\nstderr: {stderr}"
    );
    assert!(
        stdout.contains("HTML report written to"),
        "expected HTML report confirmation\nstdout: {stdout}"
    );

    let html = std::fs::read_to_string(&html_path).expect("HTML report should exist");
    assert!(
        html.contains("Soroban Cost Report"),
        "HTML should contain title"
    );
    assert!(html.contains("add"), "HTML should contain 'add' card");
    assert!(html.contains("noop"), "HTML should contain 'noop' card");
}

// ---------------------------------------------------------------------------
// Panic: test panics mid-execution — profiler should fail gracefully
// ---------------------------------------------------------------------------

#[test]
fn panic_contract_fails_gracefully() {
    let (success, stdout, stderr) = run_profiler("panic-contract");

    // The CLI should fail because cargo test failed.
    assert!(
        !success,
        "CLI should return non-zero exit code when a test panics\nstdout: {stdout}"
    );

    // Even though a test panicked, the profiler should still collect the
    // record from the non-panicking test (measure_double).
    assert!(
        stdout.contains("collected 1 cost record(s)"),
        "should collect 1 record from the passing test\nstdout: {stdout}"
    );
    assert!(
        stdout.contains("double"),
        "should contain the 'double' label from the passing test\nstdout: {stdout}"
    );

    // The panic message should be surfaced, not swallowed.
    assert!(
        stdout.contains("intentional panic")
            || stderr.contains("intentional panic")
            || stdout.contains("test failed")
            || stderr.contains("test failed"),
        "panic message or test failure should be visible\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Duplicate labels: two tests use the same label — profiler should
// disambiguate them, not silently overwrite.
// ---------------------------------------------------------------------------

#[test]
fn duplicate_labels_are_disambiguated() {
    let (success, stdout, stderr) = run_profiler("duplicate-labels-contract");

    assert!(
        success,
        "CLI should succeed on duplicate-labels contract\nstderr: {stderr}"
    );

    // Both records should be collected.
    assert!(
        stdout.contains("collected 2 cost record(s)"),
        "should collect 2 records\nstdout: {stdout}"
    );

    // The labels should be disambiguated with suffixes.
    assert!(
        stdout.contains("add_0") || stdout.contains("add_1"),
        "labels should be disambiguated with numeric suffixes\nstdout: {stdout}"
    );

    // Both disambiguated labels should appear.
    let has_add_0 = stdout.contains("add_0");
    let has_add_1 = stdout.contains("add_1");
    assert!(
        has_add_0 && has_add_1,
        "should have both add_0 and add_1 in output\nstdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Compile error: cargo test fails to compile — profiler should surface
// the compiler error clearly, not swallow it.
// ---------------------------------------------------------------------------

#[test]
fn compile_error_surfaces_compiler_message() {
    let (success, stdout, stderr) = run_profiler("compile-error-contract");

    // The CLI should fail because cargo test failed to compile.
    assert!(
        !success,
        "CLI should return non-zero exit code on compile error\nstdout: {stdout}\nstderr: {stderr}"
    );

    // No cost records should be collected (nothing compiled).
    assert!(
        !stdout.contains("collected") || stdout.contains("collected 0 cost"),
        "should not collect any records from a compile error\nstdout: {stdout}"
    );

    // The compiler error should be visible in either stdout or stderr.
    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("error")
            || combined.contains("expected")
            || combined.contains("syntax"),
        "compiler error message should be surfaced\nstdout: {stdout}\nstderr: {stderr}"
    );
}
