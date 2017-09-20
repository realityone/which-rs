# which-rs
Check for and locate installed executables from Rust.

Just demostrate how `which` command works.

## Using as a library

Put this crate in your `Cargo.toml`.

```Toml
[dependencies]
which-rs = "*"
```

### Usage

```Rust
extern crate which;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

fn main() {
    let ref file = ["python",];
    let ref paths = [Path::new("/usr/local/bin"), Path::new("/usr/bin")];
    let mut find_path = HashMap::new();

    let all_find = which::which(file, paths, true, Some(&mut find_path));
    println!("All binary is found: {:?}", all_find);
    println!("Located at {:?}", find_path);
}
```

## License
which-rs is licensed under the MIT License - see the 
[LICENSE](https://github.com/realityone/container-what/blob/master/LICENSE) file for details