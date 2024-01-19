use std::{env, io};

fn get_logical_current_working_directory(var_name: String) {
    println!("Line 4 => {}", var_name);
}

fn get_physical_current_working_directory() {
    let current_dir: Result<std::path::PathBuf, io::Error> = env::current_dir();
    let path: std::path::PathBuf = match current_dir {
        Ok(d) => d,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("{}", path.display());
}

fn main() {
    const ENV_VAR_NAME: &str = "PWD";

    let args: Vec<String> = env::args().collect();

    let mut flag: &str = "";
    if args.len() >= 2 {
        flag = &args[1];
    }
    match flag {
        "-L" => println!(
            "{:?}",
            get_logical_current_working_directory(ENV_VAR_NAME.to_string())
        ),
        "-P" => println!("{:?}", get_physical_current_working_directory()),
        _ => println!(
            "Line 33 => {:?}",
            get_logical_current_working_directory(ENV_VAR_NAME.to_string())
        ),
    }
}
