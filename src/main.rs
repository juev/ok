extern crate getopts;

use getopts::*;
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
    let verbose = matches.opt_present("v");

    if matches.opt_present("h") {
        print_usage(verbose);
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
                print_commands.push(line.clone());
            } else {
                commands.push(line.clone());
                let line = format!("{}. {}", count, line);
                count += 1;
                print_commands.push(line.clone());
            }
        }

        // get positionsal arg
        if !matches.free.is_empty() {
            let input = matches.free[0].clone();
            match input.parse::<i32>() {
                Ok(input) => {
                    if input > 0 && input <= commands.len() as i32 {
                        let line: &String = &commands[(input - 1) as usize];
                        println!("{}", line);
                        run_command(line);
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

fn run_command(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &command[..]])
            .output()
            .expect("failed to execute process")
    } else {
        // TODO: get program from env (bash,zsh,sh,fish)
        // TODO: add verbose
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    if !output.status.success() {
        println!("Command executed with failing error code");
    }

    // String::from_utf8(output.stdout)
    //     .unwrap()
    //     .lines()
    //     .for_each(|x| println!("{}", x));
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

// TODO: add all commands
pub fn create_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help screen");
    opts.optflag("c", "comment_align", "N  Level of comment alignment.");
    opts.optflag(
        "v",
        "verbose",
        "Show more output, mostly errors. Also it shows environment-variables in this screen.",
    );
    opts.optflag(
        "q",
        "quiet",
        "Only show really necessary output, so surpress echoing the command.",
    );
    opts.optopt(
        "f",
        "file",
        "Use a custom file instead of '.ok'; use '-' for stdin",
        "file",
    );
    opts.optopt(
        "a", 
        "alias", 
        "When using 'ok' in an alias, <name> is used to keep the history correct when used with 'list-prompt'.", 
        "name");
    opts.optflag("V", "version", "Show version number and exit");
    opts
}

pub fn print_usage(verbose: bool) {
    println!("Usage: ok [options] <number> [script-arguments..]
       ok command [options]
       
command (use one):
  <number>            Run the <number>th command from the '.ok' file.
  l, list             Show the list from the '.ok' file. Default command.
  L, list-once        Same as list, but only show when pwd is different from when the list was last shown.
  p, list-prompt      Show the list and wait for input at the ok-prompt (like --list and <number> in one command).
  h, help             Show this usage page.

options:
  -c, --comment_align N  Level of comment alignment. See $_OK_COMMENT_ALIGN
  -v, --verbose       Show more output, mostly errors. Also it shows environment-variables in this screen.
  -q, --quiet         Only show really necessary output, so surpress echoing the command.
  -f, --file <file>   Use a custom file instead of '.ok'; use '-' for stdin
  -a, --alias <name>  When using 'ok' in an alias, <name> is used to keep the history correct when used with 'list-prompt'.
  -V, --version       Show version number and exit
  -h, --help          Show this help screen
script-arguments:
  ...                 These are passed through, when a line is executed (you can enter these too at the ok-prompt)");

    if verbose {
        println!("environment variables (used for colored output; current colors are shown):
  _OK_C_HEADING      Color-code for lines starting with a comment (heading). Defaults to red.
  _OK_C_NUMBER       Color-code for numbering. Defaults to cyan.
  _OK_C_COMMENT      Color-code for comments after commands. Defaults to blue.
  _OK_C_COMMAND      Color-code for commands. Defaults to color-reset.
  _OK_C_PROMPT       Color-code for prompt (both input as command confirmation). Defaults to color for numbering.
environment variables (other configuration):
  _OK_COMMENT_ALIGN  Level (unset) of comment alignment. 0=no alignment, 1=align consecutive lines (Default), 2=including whitespace, 3 align all.
  _OK_PROMPT         String (unset) used as prompt (both input as command confirmation). Defaults to '$ '.
  _OK_PROMPT_DEFAULT Setting (unset) if the prompt is default shown. 1=use command list-prompt when issuing no command, otherwise use list.
  _OK_VERBOSE        Level (unset) of feedback ok provides. 0=quiet, 1=normal, 2=verbose. Defaults to 1. Can be overriden with --verbose or --quiet.
environment variables (for internal use):
  _OK__LAST_PWD      Remember the path (/path/to/some/place/with/an/.ok/file) that was last listed, for use with the list-once command.
  _OK__PATH_TO_ME    The path (/path/to/ok-bash) to the location of this script.");
    }
}
