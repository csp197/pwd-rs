use std::{env, path::PathBuf};

fn get_current_working_directory() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(path)
}

fn main() {
    let _ = get_current_working_directory();
    // println!("{:?}",path);
}
