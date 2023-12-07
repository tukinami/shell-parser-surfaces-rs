# SERIKO Parser

[GitHub Repository](https://github.com/tukinami/seriko-parser)

Parse `SERIKO` for ukagaka as `surfaces.txt`.

## Usage

``` rust
use std::{fs::File, io::Read, path::PathBuf};
use seriko_parser::{decode_bytes, parse};

let file_path =
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/surfaces/surface01.txt");
let mut file = match File::open(file_path) {
    Ok(v) => v,
    Err(e) => {
        eprintln!("{:?}", e);
        return;
    }
};
let mut buffer = Vec::new();

if let Err(e) = file.read_to_end(&mut buffer) {
    eprintln!("{:?}", e);
    return;
};

let content = match decode_bytes(&buffer) {
    Ok(v) => v,
    Err(e) => {
        eprintln!("{:?}", e);
        return;
    }
};

let seriko = match parse(&content) {
    Ok(v) => v,
    Err(e) => {
        eprintln!("{:?}", e);
        return;
    }
};
assert!(!seriko.braces().is_empty());
```

## Licese

MIT

## Author

月波 清火(tukinami seika)

[GitHub](https://github.com/tukinami)
