extern crate tsv;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;


fn main() {
    println!("Hello, world!?");
    //let mut wins = HashMap::<String,u16>::new();

    let mut x = "soc-redditHyperlinks-body.tsv";
    let file = File::open(x).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    println!("{:?}", file);

}

