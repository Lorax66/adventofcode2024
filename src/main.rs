use std::io::BufRead;
use std::{env, fs, io, path};
mod d03;

fn lines_from_file(filename: impl AsRef<path::Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let fname = &args[1];
    dbg!(&fname);
    d03::task1(&mut lines_from_file(fname));
}
