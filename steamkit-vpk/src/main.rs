use std::{fs::File, io::Read};

use nom_derive::Parse;
use steamkit_vpk::*;

fn main() {
    let mut buf = vec![0; 4 * 7];
    let mut file = File::open("assets/pak01_dir.vpk").unwrap();
    file.read_exact(&mut buf).unwrap();

    // let (_, head) = header(&buf).unwrap();

    let (_, header) = Header::parse(&buf).unwrap();

    let mut buf = vec![0; header.data.tree_length() as usize];
    file.read_exact(&mut buf).unwrap();

    let (_, tree) = tree(&buf).unwrap();

    println!("{}", tree.len());

    // println!("{:?}", s);
}
