_default:
    just -l

# start tmux session
tmux:
    tmux new -Pd -s ktr -n code
    tmux send-keys -t ktr:1 "vim" Enter
    tmux neww -Pd -t ktr -F 2 -n watch
    tmux send-keys -t ktr:2 "just watch" Enter
    tmux attach -d -t ktr

# info: https://github.com/watchexec/cargo-watch
# start cargo watch
watch:
    cargo watch -x check

# code coverage report
cover:
    cargo tarpaulin -o html
    firefox ./tarpaulin-report.html

# format and check for syntax enhancements
tidy:
    cargo fmt --all
    cargo clippy

# remove test artifacts
clean:
    rm -r output/
