use std::process::Command;

fn main() {
    println!("Hello, world!");
    let output = Command::new("ls")
            .arg("-lisa")
            //.arg("echo hello")
            .output()
            .expect("failed to execute process");

	println!("status: {}", output.status);
	println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
	println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
