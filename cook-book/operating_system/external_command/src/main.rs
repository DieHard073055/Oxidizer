use regex::Regex;
use std::process::Command;

// ![NOTE] For this function to work you will need git installed
#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn run_external_command_and_process_stdout() {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("Command failed to execute successfully");
    }
    let pattern = Regex::new(
        r"(?x)
                                ([0-9a-fA-F]+) # commit hash
                                (.*)           # The commit message",
    )
    .unwrap();
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));
}

fn main() {
    run_external_command_and_process_stdout();
}
