/// Create padding with spaces.
pub fn padding(indent: usize, inc: usize) -> String {
    let size = indent * inc;
    let mut buf = String::with_capacity(size);
    for _ in 0..size {
        buf.push_str(" ");
    }
    buf
}
