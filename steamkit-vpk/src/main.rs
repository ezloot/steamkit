use std::{fs::File, io::Read};

use steamkit_vpk::*;

fn main() {
    let mut buf = vec![0; 4 * 7];
    let mut file = File::open("assets/pak01_dir.vpk").unwrap();
    file.read_exact(&mut buf).unwrap();

    let (_, head) = header(&buf).unwrap();

    let mut buf = vec![0; head.tree_length() as usize];
    file.read_exact(&mut buf).unwrap();

    let (_, s) = tree(&buf).unwrap();

    // println!("{:?}", s);
}
