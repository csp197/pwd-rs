#[cfg(test)]
mod unittests {
    use crate::{get_logical_current_working_directory, get_physical_current_working_directory};
    use nix::unistd;
    use std::{env, iter, path::PathBuf};

    fn get_env_current_dir() -> PathBuf {
        let control: PathBuf = env::current_dir().unwrap();
        return control;
    }

    fn get_nix_cwd() -> PathBuf {
        let os_cwd: PathBuf = unistd::getcwd().unwrap();
        return os_cwd;
    }

    #[test]
    fn test_logical_cwd() {
        let env_var: &str = "PWD";
        let control: String = env::var(env_var).unwrap();
        let experiment: String = get_logical_current_working_directory(env_var.to_string());
        assert_eq!(control, experiment);
    }

    #[test]
    fn test_physical_cwd() {
        let control_1: PathBuf = get_env_current_dir();
        let control_2: PathBuf = get_nix_cwd();

        let test: String = get_physical_current_working_directory();

        let mut experiment: PathBuf = PathBuf::new();

        let strings = test
            .match_indices("/")
            .map(|(i, _)| &test[0..i])
            .chain(iter::once(test.as_str()));

        for substr in strings {
            experiment.push(substr);
        }
        assert_eq!(control_1, control_2);
        assert_eq!(control_1, experiment);
        assert_eq!(control_2, experiment);
    }
}
