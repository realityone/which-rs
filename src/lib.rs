use std::path::{Path, PathBuf};
use std::os::unix::prelude::*;
use std::collections::HashMap;

pub fn which<'a, 'b>(files: &'a [&'a str],
                     paths: &'a [&'a Path],
                     match_all: bool,
                     mut result: Option<&'b mut HashMap<&'a str, Vec<PathBuf>>>) -> Result<bool, String> {
    let mut all_matched = true;
    for f in files {
        let mut matched = false;

        for p in paths {
            let mut target = p.to_path_buf();
            target.push(f);

            // file not exists
            if !target.exists() {
                continue;
            }

            let metadata =
                target.metadata().map_err(|e| format!("read metadata failed: {}", e))?;
            if metadata.mode() & 0o111 == 0 {
                // Not an executable file
                continue;
            }

            // Find an executable file
            matched = true;
            match result {
                Some(ref mut r) => {
                    r.entry(f).or_insert(Vec::new());
                    if let Some(targets) = r.get_mut(f) {
                        targets.push(target);
                    }
                }
                None => {}
            }

            if !match_all {
                break;
            }
        }

        all_matched &= matched;
    }

    Ok(all_matched)
}