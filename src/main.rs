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
    name: String,
    n: usize,
    count: usize,
    outedges: AdjacencyList
}


fn main() {
    println!("Hello, world!?");

    //println!("Number of posts {:?}\nNumber of unique subreddits {:?}", post_id.len(), subreddit_info.len());
    //println!("{:?}, {:?}", subreddit_info.values().min(), subreddit_info.values().max());
    let sub_hash = file_and_hashmap_stuff::read_file();
    file_and_hashmap_stuff::output_hashmap(sub_hash);
    //let sum_1: Vec<_> = post_id.iter()
      //  .filter(|(&ref key, &value) | value == 1)
        //.map(|(&ref key, &value)| value)
        //.collect::<Vec<_>>();

   // println!("{:?}, {:?}", sum_1.len(), post_id.len());
    // let unique_post_num: Vec<_> = post_id.iter()
     //   .map(|(&ref key, &value)| value)
      //  .collect::<Vec<_>>();
    //unique_post_num.into_iter().unique();
    // Have to write this inefficiently because I am not sure how to use built in functions
    //let mut unique_post_num: Vec<String> = vec![];
    //for (c,i) in zip(0..len(post_id),post_id.keys()) {
     //   if !unique_post_num.contains(&i) {
      //      unique_post_num.push(i.to_string());
       //     println!("{:?}, {:?}",c, i);
        //} // else move on
    //}
    //println!("Number of unique hyperlinks {:?}", unique_post_num.len());

}
mod file_and_hashmap_stuff {
    //extern crate tsv;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::env;
    static origin_sub: Vec<String> = Vec::new();
    static dest_sub: Vec<String> = Vec::new();
    // These static variables will be used to help create adjacency lists, provides a max number of subreddits origin
    // subreddit is linked to 

    pub fn read_file() -> HashMap<String, usize>{
        let directory = env::set_current_dir("src"); //setting directory to src to get files

        let filename = "soc-redditHyperlinks-body.tsv";
        let file = File::open(filename).expect("Could not open file");
        let mut buf_reader = std::io::BufReader::new(file).lines();
        let _header = buf_reader.next();

        let mut post_id: Vec<String> = Vec::new(); // post_id, every time a post_id is found, 
        //we increment its value by 1
        // I will be graphing post_id as the posts themselves represent the relationshps between subreddits
    
        let mut total_sent: Vec<i32> = Vec::new(); // total sentiment regardless of emotion
     
        let mut counter: usize = 0; // counter will be used to keep track of subreddit numbers
        let mut max_subreddit_vertices:HashMap<String, usize> = HashMap::new(); 
        // used to store max connections from origin subreddit to outgoing
        // nodes, this will be useful for 
        for line in buf_reader {
            //reading line and splitting
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split("\t").collect();
            // going to keep track of everything in vectors in case I need them
            origin_sub.push(v[0].parse::<String>().unwrap());
            dest_sub.push(v[1].parse::<String>().unwrap());
            post_id.push(v[2].parse::<String>().unwrap());
            total_sent.push(v[3].parse::<i32>().unwrap());
            ///////// \
            // starting to map out origin subreddit names to numbers
    
            let x = v[0].parse::<String>().unwrap();
            subreddit_info = insert_hash(x, subreddit_info, counter);
            counter +=1;

            let y = v[1].parse::<String>().unwrap();
            subreddit_info = insert_hash(y, subreddit_info, counter);
            counter +=1;
        } 
        subreddit_info
    }

    pub fn output_hashmap(sub_hash: HashMap<String,usize>) {
        for (key, value) in &sub_hash {
            println!("{:?}, {:?}", key, value);
        }
    }

    pub fn insert_hash(x: String, mut sub_hash: HashMap<String, usize>, mut c: usize) -> HashMap<String, usize> {
        match sub_hash.get(&x) {
            None => {
                sub_hash.insert(x, c);
                c = c + 1;
            }
            _ => {}
        }
        sub_hash
    }
    pub fn create_adjacency_list() {
        // uses helper function create adjacency vector 
        // hashmap of original subreddit - (hashmaps of subreddits connected)
    }
    pub fn create_adjacency_vector(current_sub: HashMap<String, usize>, origin_sub: Vec<String>, dest_sub:Vec<String>) {
        //-> Vec<(HashMap<String, usize>, Vec<HashMap<String, usize>>)>
        //creates each vector in adjacency_list
        // uses 

    }
    pub fn create_helper_hash() {
        // creates secondary hash which takes all subreddits and 
    }
}

