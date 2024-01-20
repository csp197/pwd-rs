mod tests;

use std::{env, io};

fn print_err_and_exit(msg: String) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn get_logical_current_working_directory(env_var: String) -> String {
    let var_key: String =
        env::var(env_var.clone()).expect(format!("pwd-rs: ${env_var} is not set").as_str());
    return var_key;
}

fn get_physical_current_working_directory() -> String {
    let current_dir: std::path::PathBuf =
        env::current_dir().expect("pwd-rs: physical current working directory not found");
    return current_dir.into_os_string().to_str().unwrap().to_string();
}

fn main() {
    const ENV_VAR_NAME: &str = "PWD";
    let args: Vec<String> = env::args().collect();
    let mut flag: &str = "";
    if args.len() >= 2 {
        flag = &args[1];
        if !flag.starts_with('-') {
            print_err_and_exit(format!("pwd-rs: too many arguments: {flag}"))
        }
    }
    match flag {
        "" | "-L" => println!(
            "{}",
            get_logical_current_working_directory(ENV_VAR_NAME.to_string())
        ),
        "-P" => println!("{}", get_physical_current_working_directory()),
        _ => print_err_and_exit(format!("pwd-rs: bad option: {flag}")),
    }
    std::process::exit(0);
}
