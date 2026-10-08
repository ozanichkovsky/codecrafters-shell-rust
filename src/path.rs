use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub(crate) fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = env::var("PATH")
        .ok() // Convert Result to Option
        .and_then(|path_var| {
            // Use iterator to find the first directory containing an executable file
            env::split_paths(&path_var)
                .find(|path| is_executable(&path.join(name)))
        });
    match path {
        Some(p) => {
            Some(p.join(name))
        },
        None => {
            None
        }
    }
}

fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub(crate) fn get_path(path: &str) -> String {
    let home = env::var("HOME");
    if let Ok(res) = &home {
        path.replace("~", &res)
    } else {
        path.into()
    }
}