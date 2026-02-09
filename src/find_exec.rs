use std::env;

pub fn find_executable(cmd: &str) -> Option<String> {
    let path_var = env::var("PATH").ok()?;

    for path in env::split_paths(&path_var) {
        let full_path = path.join(cmd);

        if full_path.is_file() {
            return full_path.to_str().map(|s| s.to_string());
        }
    }

    None
}