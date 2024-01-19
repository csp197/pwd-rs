use std::{env, io};

fn get_logical_current_working_directory(env_var: String) -> String {
    let var_key: Result<String, env::VarError> = env::var(env_var.clone());
    let val: String = match var_key {
        Ok(v) => v,
        Err(err) => panic!("pwd-rs: ${} is not set: {}", env_var, err),
    };
    return val;
}

fn get_physical_current_working_directory() -> String {
    let current_dir: Result<std::path::PathBuf, io::Error> = env::current_dir();
    let path_buf: std::path::PathBuf = match current_dir {
        Ok(d) => d,
        Err(err) => panic!(
            "pwd-rs: physical current working directory not found: {}",
            err
        ),
    };
    return path_buf.into_os_string().to_str().unwrap().to_string();
}

fn main() {
    const ENV_VAR_NAME: &str = "PWD";
    let args: Vec<String> = env::args().collect();
    let mut flag: &str = "";
    if args.len() >= 2 {
        flag = &args[1];
    }
    match flag {
        "" => println!(
            "{}",
            get_logical_current_working_directory(ENV_VAR_NAME.to_string())
        ),
        "-L" => println!(
            "{}",
            get_logical_current_working_directory(ENV_VAR_NAME.to_string())
        ),
        "-P" => println!("{}", get_physical_current_working_directory()),
        _ => println!("pwd-rs: bad option: {}", flag),
    }
}
