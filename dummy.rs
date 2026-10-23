fn main() {
    let mut buf = vec![b'x'; 300];
    let end = 250;
    let trailing = &buf[end..];
    for &b in trailing {
        if b != b'\r' && b != b'\n' && b != b' ' && b != b'%' && b != b'E' && b != b'O' && b != b'F' && b != 0 {
            println!("Tampered by byte: {}", b);
            return;
        }
    }
    println!("OK");
}
