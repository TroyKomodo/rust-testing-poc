test *args:
    #!/bin/bash
    set -e -o pipefail 


    cargo +nightly llvm-cov nextest --branch --no-report {{args}}
    cargo +nightly llvm-cov report --html
    cargo +nightly llvm-cov report --lcov --output-path ./lcov.info

coverage:
    http-server ./target/llvm-cov/html/
