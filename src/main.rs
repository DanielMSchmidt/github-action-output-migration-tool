use clap::Parser;
use std::io::{Read, Write};
use std::{fs::File, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file / folder to read
    path: String,
}

fn main() {
    let args = Args::parse();
    // TODO: handle errors
    flat_list_all_files(Path::new(&args.path))
        .iter()
        .for_each(|file| migrate_file(file))
}

fn migrate_file(file_path: &String) {
    let content = read_file(Path::new(file_path));
    write_file(Path::new(&file_path), &migrate(content));
}

fn flat_list_all_files(path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut flat_list_all_files(&path));
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    files
}

fn read_file(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn write_file(path: &Path, contents: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn migrate(contents: String) -> String {
    println!("Migrating file: {}", contents);

    contents
        .split('\n')
        .map(migrate_line)
        .collect::<Vec<String>>()
        .join("\n")
}

fn migrate_line(line: &str) -> String {
    let mut mutable_line = String::from(line);
    let echo_start = "echo \"";
    let name_start = "::set-output name=";
    let name_value_seperator = "::";
    let end_marker = "\"";

    //  before_echo_index     before_name_index         after_end_index
    //     |                         | after_name_index      |
    //     | after_echo_index        |  | before_value_index |
    //     |       |                 |  | |        after_value_index
    //     |       |                 |  | |                | |
    // run: echo \"::set-output name=dir::$(yarn cache dir)\"
    if mutable_line.contains(echo_start) && mutable_line.contains(name_start) {
        let before_echo_index = mutable_line.find(echo_start).unwrap();
        let after_echo_index = before_echo_index + echo_start.len();

        let before_name_index = mutable_line[after_echo_index..].find(name_start).unwrap()
            + after_echo_index
            + name_start.len();
        let after_name_index = mutable_line[before_name_index..]
            .find(name_value_seperator)
            .unwrap()
            + before_name_index;

        let before_value_index = after_name_index + name_value_seperator.len();

        let after_value_index =
            mutable_line[before_value_index..].find(end_marker).unwrap() + before_value_index;
        let after_end_index = after_value_index + end_marker.len();

        let prefix = &mutable_line[..before_echo_index];
        let suffix = if after_end_index < mutable_line.len() {
            &mutable_line[after_end_index..]
        } else {
            ""
        };

        let name = &mutable_line[before_name_index..after_name_index];
        let value = if after_value_index < mutable_line.len() {
            &mutable_line[before_value_index..after_value_index]
        } else {
            &mutable_line[before_value_index..]
        };

        println!(
            "{}echo \"{}={}\" >> $GITHUB_OUTPUT{}",
            prefix, name, value, suffix
        );

        mutable_line = format!(
            "{}echo \"{}={}\" >> $GITHUB_OUTPUT{}",
            prefix, name, value, suffix
        );
    }

    mutable_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_migrates_single_line() {
        let input = "run: echo \"::set-output name=dir::$(yarn cache dir)\"";
        let expected = "run: echo \"dir=$(yarn cache dir)\" >> $GITHUB_OUTPUT";

        assert_eq!(migrate_line(input.into()), expected);
    }

    #[test]
    fn it_migrates_workflow_file() {
        let input = "jobs:
        unit-tests:
          steps:
            - name: Get yarn cache directory path
              id: yarn-cache-dir-path
              run: echo \"::set-output name=dir::$(yarn cache dir)\"";

        let expected = "jobs:
        unit-tests:
          steps:
            - name: Get yarn cache directory path
              id: yarn-cache-dir-path
              run: echo \"dir=$(yarn cache dir)\" >> $GITHUB_OUTPUT";

        assert_eq!(migrate(input.into()), expected);
    }
}
