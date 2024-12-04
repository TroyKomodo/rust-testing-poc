#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]
#![cfg_attr(all(coverage_nightly, test), coverage(off))]

use test_runner::{config, harness, Test};

fn main() {
    let tests = std::fs::read_dir("ui/tests")
        .into_iter()
        .flat_map(|dir| dir.into_iter())
        .filter_map(|dir| dir.ok())
        .filter(|dir| dir.file_type().unwrap().is_file())
        .filter(|dir| dir.path().extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|dir| {
            Some(Test {
                name: dir
                    .path()
                    .to_string_lossy()
                    .strip_suffix(".rs")?
                    .replace("/", "::"),
                path: dir.path(),
            })
        })
        .collect();

    harness(tests, config!());
}
