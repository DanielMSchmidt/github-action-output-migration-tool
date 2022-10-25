# Github Action Output Migration Tool [![Rust](https://github.com/DanielMSchmidt/github-action-output-migration-tool/actions/workflows/rust.yml/badge.svg)](https://github.com/DanielMSchmidt/github-action-output-migration-tool/actions/workflows/rust.yml)

CLI to migrate the deprectated `echo "::set-output name=dir::$(yarn cache dir)"` output syntax to the newer `echo "dir=$(yarn cache dir)" >> $GITHUB_OUTPUT` syntax.

## Why?

Wanted to write some Rust that is useful to me. Yeah, a Regex search and replace would have been faster.

## License

MIT
