// Code needs to be split into modules
pub struct Graph {
    n: usize,
    outedges: AdjacencyList,
}
pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyList = Vec<Vec<Vertex>>;

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

fn main() {
    println!("Hello, world!?");

    //println!("Number of posts {:?}\nNumber of unique subreddits {:?}", post_id.len(), subreddit_info.len());
    //println!("{:?}, {:?}", subreddit_info.values().min(), subreddit_info.values().max());
    let results = file_and_hashmap_stuff::read_file();
    let sub_hash = results.0;
    println!("{:?}, {:?}", sub_hash.len(), results.1.len());
    let graph: Graph = graph_stuff::create_graph(sub_hash, results.1);
    let connections: Vec<(usize, usize)> = graph_stuff::grab_connections(&graph);
    //graph_stuff::output_n_graph(&graph, 10);
 
    ////////////////////////////
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
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::env;
    use crate::Graph;
    use crate::ListOfEdges;
    // These static variables will be used to help create adjacency lists, provides a max number of subreddits origin
    // subreddit is linked to 

    pub fn read_file() -> (HashMap<String, usize>, Vec<(String, String)>){
        let directory = env::set_current_dir("src"); //setting directory to src to get files

        let filename = "soc-redditHyperlinks-body.tsv";
        let file = File::open(filename).expect("Could not open file");
        let mut buf_reader = std::io::BufReader::new(file).lines();
        let _header = buf_reader.next();

        let mut post_id: Vec<String> = Vec::new(); // post_id, every time a post_id is found, 
        //we increment its value by 1
    
        let mut total_sent: Vec<i32> = Vec::new(); // total sentiment regardless of emotion
        let mut sub_pairs: Vec<(String, String)> = Vec::new();
     
        let mut counter: usize = 0; // counter will be used to keep track of subreddit numbers
        let mut subreddit_info: HashMap<String, usize> = HashMap::new();
        // used to store max connections from origin subreddit to outgoing
        // nodes, this will be useful for 
        for line in buf_reader {
            //reading line and splitting
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split("\t").collect();
            // going to keep track of everything in vectors in case I need them

            let x = v[0].parse::<String>().unwrap();
            let y = v[1].parse::<String>().unwrap();
            sub_pairs.push((x,y));
            post_id.push(v[2].parse::<String>().unwrap());
            total_sent.push(v[3].parse::<i32>().unwrap());
            ///////// \
            // starting to map out origin subreddit names to numbers
    
            let x = v[0].parse::<String>().unwrap();
            let y = v[1].parse::<String>().unwrap();

            for i in [x,y]{ //going through x and y
                match subreddit_info.get(&i){
                    None => {
                        subreddit_info.insert(i, counter);
                        counter += 1; } // incrementing counter if subreddit doesn't exist in hashmap
                    _ => {} // doing nothing if it already exists
                }
            }
        } 
        println!("Counter = {:?}", counter);
        (subreddit_info, sub_pairs)
    }

    pub fn _output_hashmap(sub_hash: HashMap<String,usize>) {
        for (key, value) in &sub_hash {
            println!("{:?}, {:?}", key, value);
        }
    }


}
pub mod graph_stuff {
    use std::collections::HashMap;
    use crate::Graph;
    use crate::ListOfEdges;
    pub fn create_graph(sub_hash: HashMap<String, usize>, sub_pairs: Vec<(String, String)>) -> Graph {
        // creates secondary hash which takes all subreddits and puts max???
        let mut my_adjacency_list: ListOfEdges = Vec::new(); 
        let mut n: usize = 0;

        for (i,j) in sub_pairs {
            let x = sub_hash.get(&i).unwrap(); //getting number for origin subreddit
            let y = sub_hash.get(&j).unwrap(); // getting number for desination subreddit
            my_adjacency_list.push((*x,*y)); //pushing new vertex pair into adjacency list
            n += 1;
        }
        //my_adjacency_list.sort();
        my_adjacency_list.sort();
        let my_graph: Graph = Graph::create_directed(n, &my_adjacency_list);
        my_graph
    }
    pub fn output_whole_graph(graph: &Graph){
        for (i, l) in graph.outedges.iter().enumerate() {
            println!("{} {:?}", i, *l);
        }
    }
    pub fn output_n_graph(graph: &Graph, n: usize){
        for (i, l) in graph.outedges.iter().take(n).enumerate() {
            println!("{} {:?}", i, *l);
        }
    }
    pub fn grab_connections(graph: &Graph) -> Vec<(usize, usize)> {
        // gets number of outgoing edges from each subreddit and returns it
        let mut x: Vec<(usize, usize)> = Vec::new();
        for (i, l) in graph.outedges.iter().enumerate() {
            x.push((i,l.len()));
        }
        x
    }
    pub fn create_xy(connections: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        // gets vector of usize tuples from grab_connections and creates x and y vectors for plotting purposes
        // x = values from 0 - number of subreddits
        // y = number of subreddits which have outgoing edges corresponding to x
        let mut xy_vec: Vec<(usize, usize)> = vec![vec![]];
        let mut xy_hash: HashMap<usize, usize> = HashMap::new();

        for (_i,j) in connections {
            // i = subreddit number, j = number of outgoing nodes from subreddit i
            if let Some(value) = xy_hash.get_mut(&j) { // checking if value j exists in hashmap
                *value += 1; // if it does then increment by one
            } else { // if it doesn't then create first instance of it 
                xy_hash.insert(&j, 1);
            }
        }
    }
}
pub mod plotting_stuff{
    //??? hello?
}


