use std::env;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process;
use tempfile::NamedTempFile;

/// Find the executable.
fn find_exe() -> PathBuf {
    // Tests exe is in target/debug/deps, the exe is in target/debug
    let root = env::current_exe()
        .expect("tests executable")
        .parent()
        .expect("tests executable directory")
        .parent()
        .expect("fd executable directory")
        .to_path_buf();

    let exe_name = "ccwc";

    root.join(exe_name)
}

/// Format an error message for when *fd* did not exit successfully.
fn format_exit_error(args: &[&str], output: &process::Output) -> String {
    format!(
        "`ccwc {}` did not exit successfully.\nstdout:\n---\n{}---\nstderr:\n---\n{}---",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Environment for the integration tests.
pub struct TestEnv {
    temp_file: NamedTempFile,

    /// Path to the executable.
    exe: PathBuf,
}

impl TestEnv {
    pub fn new(file_contents: &str) -> TestEnv {
        let exe = find_exe();

        let mut temp_file = tempfile::Builder::new()
            .prefix("ccwc-tests")
            .tempfile()
            .expect("test working directory");

        write!(temp_file, "{file_contents}").unwrap();

        // Seek to start
        temp_file.seek(SeekFrom::Start(0)).unwrap();

        TestEnv { temp_file, exe }
    }

    fn temp_file_path(&self) -> &str {
        self.temp_file
            .path()
            .to_str()
            .expect("converting temp file path")
    }

    fn run_command(&self, args: &[&str]) -> process::Output {
        // Setup command
        let mut cmd = process::Command::new(&self.exe);
        cmd.args(args);

        // Run command
        cmd.output().expect("command output")
    }

    fn assert_output(&self, args: &[&str], expected: &str) {
        let output = self.run_command(&args);
        if !output.status.success() {
            panic!("{}", format_exit_error(&args, &output));
        }
        assert_eq!(
            String::from_utf8(output.stdout)
                .ok()
                .expect("Converting stdout to string"),
            expected
        )
    }
}

#[test]
fn it_returns_the_byte_word_and_char_counts_by_default() {
    let env = TestEnv::new("This is some test file input");

    let args = [env.temp_file_path()];
    let expected_output = format!("1 6 28 {filename}\n", filename = env.temp_file_path());

    env.assert_output(&args, &expected_output);
}
