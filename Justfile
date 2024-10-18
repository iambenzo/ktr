_default:
    just -l

# info: https://github.com/watchexec/cargo-watch
# start cargo watch
watch:
    cargo watch -x check

# code coverage report
cover:
    cargo tarpaulin -o html
    firefox ./tarpaulin-report.html

# install/update git hooks
hooks:
    cp ./git-hooks/* ./.git/hooks/

# format and check for syntax enhancements
tidy:
    cargo fmt
    cargo clippy
