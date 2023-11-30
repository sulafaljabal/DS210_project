extern crate tsv;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use itertools::Itertools; // 0.10.3


// Code needs to be split into modules



fn main() {
    println!("Hello, world!?");
    //let mut wins = HashMap::<String,u16>::new();

    let filename = "soc-redditHyperlinks-body.tsv";
    let file = File::open(filename).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    let header = buf_reader.next();

    let mut origin_sub: Vec<String> = Vec::new(); // origin subreddit
    let mut dest_sub: Vec<String> = Vec::new(); // destination subreddit
    let mut post_id = HashMap::new(); // post_id, every time a post_id is found, 
    //we increment its value by 1
    // I will be graphing post_id as the posts themselves represent the relationshps between subreddits


    let mut n_sent: Vec<i32> = Vec::new(); // happy/neutral sentiments
    let mut a_sent: Vec<i32> = Vec::new(); // angry sentiments
    let mut total_sent: Vec<i32> = Vec::new(); // total sentiment regardless of emotion


    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split("\t").collect();

        origin_sub.push(v[0].parse::<String>().unwrap());
        dest_sub.push(v[1].parse::<String>().unwrap());
        //post_id.push(v[2].parse::<String>().unwrap());
        total_sent.push(v[3].parse::<i32>().unwrap());

        let x = v[2].parse::<String>().unwrap();
        if let Some(current_post) = post_id.get_mut(&x) {
            *current_post += 1; // incrementing post_id's number of hyperlinks/vertices
        } else {
            post_id.insert(x, 1); // creating new key-value pair and setting it to 1
        }
    }

    println!("Number of posts {:?}\nNumber of entries {:?}", post_id.len(), origin_sub.len());

    let sum_1: Vec<_> = post_id.iter()
        .filter(|(&ref key, &value) | value == 1)
        .map(|(&ref key, &value)| value)
        .collect::<Vec<_>>();

    println!("{:?}, {:?}", sum_1.len(), post_id.len());
    //let unique_post_num: Vec<_> = post_id.iter().map(|(&ref key, &value)| value).collect::<Vec<_>>();
    //unique_post_num.unique();
    println!("Number of unique hyperlinks {:?}", post_id.values().len());

}

