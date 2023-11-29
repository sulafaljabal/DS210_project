extern crate tsv;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;


fn main() {
    println!("Hello, world!?");
    //let mut wins = HashMap::<String,u16>::new();

    let x = "soc-redditHyperlinks-body.tsv";
    let file = File::open(x).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    let header = buf_reader.next();

    let mut origin_sub: Vec<String> = Vec::new(); // origin subreddit
    let mut dest_sub: Vec<String> = Vec::new(); // destination subreddit
    let mut post_id: Vec<String> = Vec::new(); // post_id

    let mut n_sent: Vec<i32> = Vec::new(); // happy/neutral sentiments
    let mut a_sent: Vec<i32> = Vec::new(); // angry sentiments
    let mut total_sent: Vec<i32> = Vec::new(); // total sentiment regardless of emotion


    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split("\t").collect();

        origin_sub.push(v[0].parse::<String>().unwrap());
        dest_sub.push(v[1].parse::<String>().unwrap());
        post_id.push(v[2].parse::<String>().unwrap());
        total_sent.push(v[3].parse::<i32>().unwrap())
        //println!("{:?} {:?} {:?} {:?}", x,y,z,a);
    }

    let v = 
    println!("{:?}", total_sent.len());

}

