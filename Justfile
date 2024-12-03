test *args:
    #!/bin/bash
    set -e -o pipefail 

    if [ -z "$CI" ]; then
        export INSTA_FORCE_PASS=1
    fi

    cargo +nightly llvm-cov nextest --branch --no-report {{args}}

    # Do not generate the coverage report on CI
    if [ -z "$CI" ]; then
        cargo insta review
        cargo +nightly llvm-cov report --html
        cargo +nightly llvm-cov report --lcov --output-path ./lcov.info
    else
        cargo +nightly llvm-cov report --codecov --output-path ./codecov.json
    fi


coverage:
    http-server ./target/llvm-cov/html/
