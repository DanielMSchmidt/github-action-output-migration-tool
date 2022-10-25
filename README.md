# Github Action Output Migration Tool

CLI to migrate the deprectated `echo "::set-output name=dir::$(yarn cache dir)"` output syntax to the newer `echo "dir=$(yarn cache dir)" >> $GITHUB_OUTPUT` syntax.

## Why?

Wanted to write some Rust that is useful to me. Yeah, a Regex search and replace would have been faster.

## License

MIT
