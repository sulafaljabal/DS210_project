//extern crate tsv;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
//use itertools::Itertools; // 0.10.3


// Code needs to be split into modules
struct Graph {
    n: usize,
    outedges: AdjacencyList,
}
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyList = Vec<Vec<Vertex>>;

impl Graph {
    fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
}
////////
struct Subreddit {
    name: HashMap<String, usize>,
    count: usize,
}
impl Subbreddit {
    fn create(sub_name: HashMap<String, usize>, num_hyperlinks:usize) -> Subreddit{
        Subreddit {
            name: sub_name,
            count: num_hyperlinks,
        }
    }
    fn increment_count(&mut self) {
        self.count = self.count + 1;
    }
}


fn main() {
    println!("Hello, world!?");
    //let mut wins = HashMap::<String,u16>::new();

    let filename = "soc-redditHyperlinks-body.tsv";
    let file = File::open(filename).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    let header = buf_reader.next();

    let mut origin_sub = HashMap::new(); // origin subreddit
    let mut dest_sub: Vec<String> = Vec::new(); // destination subreddit
    let mut post_id = Vec<String> = Vec::new(); // post_id, every time a post_id is found, 
    //we increment its value by 1
    // I will be graphing post_id as the posts themselves represent the relationshps between subreddits


    //let mut n_sent: Vec<i32> = Vec::new(); // happy/neutral sentiments
    //let mut a_sent: Vec<i32> = Vec::new(); // angry sentiments
    let mut total_sent: Vec<i32> = Vec::new(); // total sentiment regardless of emotion

    let mut counter: usize = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split("\t").collect();

        //origin_sub.push(v[0].parse::<String>().unwrap());

        let x = v[0].parse::<String>().unwrap();
        if let Some(current_sub) = origin_sub.get_mut(&x) {
            *current_sub += 1; // incrementing subreddit's number of hyperlinks/vertices
        } else {
            origin_sub.insert(x, counter); // creating new key-value pair with name of subreddit as key and counter value as
            // value

        }

        dest_sub.push(v[1].parse::<String>().unwrap());
        post_id.push(v[2].parse::<String>().unwrap());
        total_sent.push(v[3].parse::<i32>().unwrap());
    }

    println!("Number of posts {:?}\nNumber of entries {:?}", post_id.len(), origin_sub.len());

    let sum_1: Vec<_> = post_id.iter()
        .filter(|(&ref key, &value) | value == 1)
        .map(|(&ref key, &value)| value)
        .collect::<Vec<_>>();

    println!("{:?}, {:?}", sum_1.len(), post_id.len());
    let unique_post_num: Vec<_> = post_id.iter()
        .map(|(&ref key, &value)| value)
        .collect::<Vec<_>>();
    unique_post_num.into_iter().unique();
    // Have to write this inefficiently because I am not sure how to use built in functions
    //let mut unique_post_num: Vec<String> = vec![];
    //for (c,i) in zip(0..len(post_id),post_id.keys()) {
     //   if !unique_post_num.contains(&i) {
      //      unique_post_num.push(i.to_string());
       //     println!("{:?}, {:?}",c, i);
        //} // else move on
    //}
    println!("Number of unique hyperlinks {:?}", unique_post_num.len());

}

