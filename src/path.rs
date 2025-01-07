use std::env;
use std::path::PathBuf;

pub(crate) fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = env::var("PATH")
        .ok() // Convert Result to Option
        .and_then(|path_var| {
            // Use iterator to find the first directory containing the file
            env::split_paths(&path_var)
                .find(|path| path.join(name).is_file())
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

pub(crate) fn get_path(path: &str) -> String {
    let home = env::var("HOME");
    if let Ok(res) = &home {
        path.replace("~", &res)
    } else {
        path.into()
    }
}