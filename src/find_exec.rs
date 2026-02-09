use std::{env, fs};

pub fn find_executable(cmd: &str) -> Option<String> {
    let path_var = env::var("PATH").ok()?;

    for path in env::split_paths(&path_var) {
        let full_path = path.join(cmd);

        if full_path.is_file() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return full_path.to_str().map(|s| s.to_string());
                }
            }
        }
    }

    None
}