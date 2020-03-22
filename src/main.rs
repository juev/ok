extern crate getopts;

use colored::*;
use getopts::*;
use regex::{Captures, Regex};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = create_opts();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage();
        return;
    }
    if matches.opt_present("V") {
        let version = env!("CARGO_PKG_VERSION");
        println!("Version of `ok`: {}", version);
        return;
    }

    let file = matches.opt_str("f").unwrap_or_else(|| ".ok".to_string());

    if Path::new(&file).exists() {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        let mut commands = Vec::new();
        let mut print_commands = Vec::new();

        let mut count = 1;
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.trim().chars().next().unwrap() == '#' {
                print_commands.push(line.clone().red().to_string());
            } else {
                commands.push(line.clone());
                let line = format!("{}. {}", count.to_string().bright_cyan(), line);
                count += 1;
                print_commands.push(line.clone());
            }
        }

        // get positionsal arg
        if !matches.free.is_empty() {
            let input = matches.free[0].clone();
            let args = matches.free.clone();
            match input.parse::<i32>() {
                Ok(input) => {
                    if input > 0 && input <= commands.len() as i32 {
                        let line: &String = &commands[(input - 1) as usize];
                        println!("$ {}", line);
                        run_command(line, args);
                    } else {
                        println!("Number not found: {}", input);
                    }
                }
                Err(e) => {
                    println!("Get error in parsing <number>: {}", e);
                    exit(1);
                }
            }
            return;
        }

        for command in print_commands {
            println!("{}", command);
        }
    } else {
        println!("File `{}` do not exist", &file);
    }
}

fn run_command(command: &str, args: Vec<String>) {
    let shell = match env::var("SHELL") {
        Ok(m) => m,
        Err(_) => "sh".to_string(),
    };
    let v: Vec<&str> = args.iter().map(String::as_ref).collect();
    let command = fill_placeholders(command, &v);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &command[..]])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(shell)
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub fn create_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help screen");
    opts.optflag(
        "v",
        "verbose",
        "Show more output, mostly errors. Also it shows environment-variables in this screen.",
    );
    opts.optopt(
        "f",
        "file",
        "Use a custom file instead of '.ok'; use '-' for stdin",
        "file",
    );
    opts.optflag("V", "version", "Show version number and exit");
    opts
}

pub fn print_usage() {
    println!("Usage: ok [options] <number> [script-arguments..]
       ok command [options]
       
command (use one):
  <number>            Run the <number>th command from the '.ok' file.
  l, list             Show the list from the '.ok' file. Default command.
  p, list-prompt      Show the list and wait for input at the ok-prompt (like --list and <number> in one command).
  h, help             Show this usage page.

options:
  -f, --file <file>   Use a custom file instead of '.ok'; use '-' for stdin
  -V, --version       Show version number and exit
  -h, --help          Show this help screen
script-arguments:
  ...                 These are passed through, when a line is executed (you can enter these too at the ok-prompt)");
}

fn fill_placeholders(template: &str, values: &[&str]) -> String {
    let regex = Regex::new(r#"\$(\d+)"#).unwrap();
    regex
        .replace_all(template, |captures: &Captures| {
            values.get(index(captures)).unwrap_or(&"")
        })
        .to_string()
}

fn index(captures: &Captures) -> usize {
    captures.get(1).unwrap().as_str().parse().unwrap()
}
