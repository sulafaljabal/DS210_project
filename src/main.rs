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
    let graph: Graph = graph_stuff::create_graph(sub_hash.clone(), results.1);
    let connections: Vec<(usize, usize)> = graph_stuff::grab_connections(&graph);
    //graph_stuff::output_n_graph(&graph, 10);
    let my_xy: Vec<(usize, usize)> = graph_stuff::create_xy(connections);
    println!("{:?}", my_xy);
    println!("{:?}",plotting_stuff::plot_graph(my_xy.clone()));
    let mut zero_counter: usize = 0;
    for (i, j) in &my_xy{
        if *j == 0{ zero_counter += 1;}
    }
    println!("{}", zero_counter);
 
}
mod file_and_hashmap_stuff {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::env;
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
        // returns x where x = (subreddit identifying number, number of outgoing hyperlinks)
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

        // reversing connections to get largest number of connections from each subreddit
        // not taking the subreddit identification number so variable is a vector of usizes
        let reverse_connections: Vec<usize> = connections.iter().map(|&(first, second)|second).collect();

        // getting maximum number of connections within this new vector just obtained
        let max_connections: usize = *reverse_connections.iter().max().unwrap();
        let mut xy: Vec<(usize, usize)> = (0..=max_connections).map(|x| (x, 0)).collect();
        // empty vector: [(0,0), (1,0), (2,0), ..., (n,0)]
        println!("{:?}", max_connections);
        //println!("{:?}", reverse_connections.iter().take(20).collect::<Vec<_>>());
        for (i,j) in &connections {
            if *j == max_connections {println!("{:?}", i);}
        }

        for (_i,j) in connections {
            // i = subreddit number, j = number of outgoing nodes from subreddit i
            xy[j].1 += 1; // incrementing connection j by 1
        }
        xy
    }
}
pub mod plotting_stuff{
    use plotters::prelude::*;
    pub fn plot_graph(xy: Vec<(usize, usize)>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("graphs/0.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Graphs??", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f32..4666f32, 0f32..100f32)?;
    
        chart.configure_mesh().draw()?;
    
        chart
            .draw_series(LineSeries::new(
                //(-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                xy.iter().map(|(x,y)| (*x as f32, *y as f32)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
    
        root.present()?;
    
        Ok(())
    }
}


