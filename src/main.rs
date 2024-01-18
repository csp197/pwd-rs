use std::{env, io};

fn get_current_working_directory() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}

fn main() {
    let mut buf: String = String::new();
    let _n: usize = io::stdin().read_line(&mut buf).unwrap();
    match buf.as_str() {
        "-L\n" => get_current_working_directory(),
        "-P\n" => println!("-P flag detected!"),
        _ => println!("No flag detected... => {}", buf),
    }
    // let _ = get_current_working_directory();
    // println!("{:?}",path);
}
