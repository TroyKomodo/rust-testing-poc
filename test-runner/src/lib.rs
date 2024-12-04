#![cfg_attr(all(coverage_nightly), feature(coverage_attribute))]
#![cfg_attr(all(coverage_nightly), coverage(off))]

use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
    process::{Command, ExitStatus}, sync::Arc,
};

mod deps;
mod features;

#[derive(Debug)]
pub struct TestOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl std::fmt::Display for TestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.status)?;
        if !self.stderr.is_empty() {
            write!(f, "--- stderr \n{}\n", self.stderr)?;
        }
        if !self.stdout.is_empty() {
            write!(f, "--- stdout \n{}\n", self.stdout)?;
        }
        Ok(())
    }
}

fn run(test: &Test, config: &Config) -> Result<(), libtest_mimic::Failed> {
    let mut program = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()));
    program.env("RUSTC_BOOTSTRAP", "1");
    program.arg("-Zunpretty=expanded");

    let rust_flags = std::env::var_os("RUSTFLAGS");

    if let Some(rust_flags) = &rust_flags {
        program.args(
            rust_flags
                .as_encoded_bytes()
                .split(|&b| b == b' ')
                .map(|flag| OsString::from(OsStr::from_bytes(flag))),
        );
    }

    program.arg("--out-dir").arg(&config.target_dir);

    program.arg(&test.path);
    program.envs(std::env::vars());

    if let Err(err) = deps::build_dependencies(config, &mut program) {
        eprintln!(
            "Failed to build dependencies: \ncommand: {}\nerrors: {:?}\nstderr: {}\nstdout: {}",
            err.command,
            err.errors,
            String::from_utf8_lossy(&err.stderr),
            String::from_utf8_lossy(&err.stdout)
        );
        std::process::exit(1);
    }

    program.stderr(std::process::Stdio::piped());
    program.stdout(std::process::Stdio::piped());

    let child = program.spawn().unwrap();

    let output = child.wait_with_output().unwrap();

    let output = TestOutput {
        status: output.status,
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };

    let mut settings = insta::Settings::new();
    settings.set_input_file(&test.path);
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        (config.compare_fn)(&output, test);
    });

    Ok(())
}

#[derive(Debug)]
pub struct Test {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct Config {
    pub target_dir: PathBuf,
    pub manifest: PathBuf,
    pub compare_fn: Arc<dyn Fn(&TestOutput, &Test) + Send + Sync>,
}

#[macro_export]
macro_rules! config {
    () => {{
        $crate::Config {
            target_dir: ::std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .parent()
                .unwrap()
                .to_path_buf(),
            manifest: ::std::path::PathBuf::from(env!("CARGO_MANIFEST_PATH")),
            compare_fn: ::std::sync::Arc::new(|output, test| {
                insta::assert_snapshot!(test.name.as_str(), output)
            }),
        }
    }};
}

pub fn harness(tests: Vec<Test>, config: Config) {
    let tests = tests.into_iter().map(|test| {
        let config = config.clone();
        libtest_mimic::Trial::test(test.name.clone(), move || {
            run(&test, &config)
        })
    }).collect();

    libtest_mimic::run(&libtest_mimic::Arguments::from_args(), tests).exit()
}
