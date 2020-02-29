use std::process::Command;

fn main() {
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

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| println!("{}", x));
}
