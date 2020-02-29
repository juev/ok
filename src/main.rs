mod opts;
mod usage;

use std::env;
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

    let mut dir = env::current_dir().unwrap();
    let file = matches.opt_str("f").unwrap_or(".ok".to_string());

    dir.push(file);
    let path = Path::new(dir.to_str().unwrap()).canonicalize().unwrap();

    print!("{}", path.display());

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };
    // let hello = output.stdout;

    if !output.status.success() {
        println!("Command executed with failing error code");
    }

    println!();
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| println!("{}", x));
}
