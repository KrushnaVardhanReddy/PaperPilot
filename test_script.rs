use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("paperpilot-pdf/src/operations/pdf_remove_signatures.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents.lines().skip(335).take(40).collect::<Vec<_>>().join("\n"));
}
