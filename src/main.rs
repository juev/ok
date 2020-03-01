mod opts;
mod usage;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = opts::create_opts();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let mut verbose = false;
    if matches.opt_present("v") {
        verbose = true;
    }

    if matches.opt_present("h") {
        usage::print_usage(&verbose);
        return;
    }

    let file = matches.opt_str("f").unwrap_or(".ok".to_string());

    println!("{}", file);

    if Path::new(&file).exists() {
        println!("exist");
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
                                      // Show the line and its number.
            println!("{}. {}", index + 1, line);
            run_command(line);
        }
    } else {
        println!("not exist");
    }
}

fn run_command(command: String) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &command[..]])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };

    if !output.status.success() {
        println!("Command executed with failing error code");
    }

    println!();
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| println!("{}", x));
}
