use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input(day: usize) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(format!("{:02}", day));
    let mut file = File::open(path).unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    str
}
