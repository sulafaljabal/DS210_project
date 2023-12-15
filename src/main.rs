// code from Prof.
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
//////// end of prof's code

fn main() {
    let results = file_and_hashmap_stuff::read_file();
    let sub_hash = results.0;
    println!("Length of subreddit hashmap: {:?}\nLength of subreddit pairs: {:?}", sub_hash.len(), results.1.len());
    let graph: Graph = graph_stuff::create_graph(sub_hash.clone(), results.1); // making graph
    println!("Length of graph = {:?}", graph.n);
    // connections between origin and destination subreddit
    let connections: Vec<(usize, usize)> = graph_stuff::grab_connections(&graph);
    //printing first 10 subreddits in graph (not ranked or anything like that)
    graph_stuff::output_n_graph(&graph, 10, &sub_hash);
    //creating coordinates for graph
    let my_xy: Vec<(usize, usize)> = graph_stuff::create_xy(connections);
    //plotting
    let _ = plotting_stuff::plot_graph(my_xy.clone());
    //statistics on number of subreddits that have no outgoing hyperlinks
    let mut zero_counter: usize = 0;
    for (_i, j) in &my_xy{
        if *j == 0{ zero_counter += 1;}
    }
    println!("Number of subreddits with zero outgoing connections: {}", zero_counter);
    //grabbing subreddit with largest number of outgoing hyperlinks
    for (name, number) in &sub_hash {
        if *number == 122 as usize{println!("Subreddit {:?} had the most outgoing links.", name);}
    }
 
}
pub mod file_and_hashmap_stuff {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufRead;
    use std::env;

    pub fn read_file() -> (HashMap<String, usize>, Vec<(String, String)>){
        let _directory = env::set_current_dir("src"); //setting directory to src to get files
        let filename = "soc-redditHyperlinks-body.tsv";
        let file = File::open(filename).expect("Could not open file");
        let mut buf_reader = std::io::BufReader::new(file).lines();
        buf_reader.next(); // don't need header

        //each line of the file has origin and destination subreddit, I will use sub_pairs to put them together
        let mut sub_pairs: Vec<(String, String)> = Vec::new();
        let mut counter: usize = 0; // counter will be used to keep track of subreddit numbers
        //this hashmap will be used to create unique identification numbers for all the subreddits,0 to n
        let mut subreddit_info: HashMap<String, usize> = HashMap::new(); 

        //going through file line by line
        for line in buf_reader {
            //reading line and splitting
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split("\t").collect();

            let x = v[0].parse::<String>().unwrap();
            let y = v[1].parse::<String>().unwrap();
            sub_pairs.push((x,y)); //pushing names of origin and destination subreddit into vector to be used later
            ///////// \
            // starting to map out origin subreddit names to numbers
    
            let x = v[0].parse::<String>().unwrap();
            let y = v[1].parse::<String>().unwrap();

            for i in [x,y]{ //going through x and y and creating new instance of subreddit in hashmap
                match subreddit_info.get(&i){
                    None => { // if the current subreddit doesn't exist then we create a new one and it gets its own number
                        subreddit_info.insert(i, counter);
                        counter += 1; } // incrementing counter if subreddit doesn't exist in hashmap
                    _ => {} // doing nothing if it already exists
                }
            }
        } 
        (subreddit_info, sub_pairs) //returning hashmap and subpairs for main()
    }

    pub fn _output_hashmap(sub_hash: HashMap<String,usize>) {
        //function to print out hashmap
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
        //creating a new instance of an adjacency list, this will be used to create the graph
        let mut my_adjacency_list: ListOfEdges = Vec::new(); 
        //going through sub_pairs and getting their unique ID numbers from hashmap called sub_hash
        for (i,j) in sub_pairs {
            let x = sub_hash.get(&i).unwrap(); //getting number for origin subreddit
            let y = sub_hash.get(&j).unwrap(); // getting number for desination subreddit
            my_adjacency_list.push((*x,*y)); //pushing new vertex pair into adjacency list
        }
        my_adjacency_list.sort(); //sorting
        let my_graph: Graph = Graph::create_directed(sub_hash.len(), &my_adjacency_list); //creating new instance of graph
        my_graph 
    }
    pub fn output_whole_graph(graph: &Graph){
        // function to output whole graph
        for (i, l) in graph.outedges.iter().enumerate() {
            println!("{} {:?}", i, *l);
        }
    }
    pub fn output_n_graph(graph: &Graph, n: usize, sub_hash: &HashMap<String, usize>){
        //function that outputs first n subreddits from graph, this isn't ordered, it is just the first n elements that 
        // were pushed into the graph upon its creation
        println!("\nPrinting first {:?} subreddits from the graph:", n);
        for (i, l) in graph.outedges.iter().take(n).enumerate() {
            let mut name = "";
            for (key, value) in sub_hash{ //going through sub_hash with ID numbers and checking if graph number matches
                // to value in hashmap
                if *value == i {name = key;} // if this is the case, then grab the name 
            }
            println!("Subreddit {:?} with {:?} connections", name, l.len());//print sub name and number of connections they have
        }
        println!("End of printing\n");
    }

    pub fn grab_connections(graph: &Graph) -> Vec<(usize, usize)> {
        // gets number of outgoing edges from each subreddit and returns it
        // returns x where x = (subreddit unique ID number, number of outgoing hyperlinks)
        let mut x: Vec<(usize, usize)> = Vec::new();
        for (i, l) in graph.outedges.iter().enumerate() {// traversing through the graph to do this
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
        let reverse_connections: Vec<usize> = connections.iter().map(|&(_first, second)|second).collect();

        // getting maximum number of connections within this new vector just obtained
        let max_connections: usize = *reverse_connections.iter().max().unwrap();
        // creating empty vector of tuples where length ends at the max_connections as there are no subreddits that have
        // more than the maximum number of outgoing connections
        let mut xy: Vec<(usize, usize)> = (0..=max_connections).map(|x| (x, 0)).collect();
        // empty vector: [(0,0), (1,0), (2,0), ..., (n,0)], n = max_connections

        //grabbing name of subreddit with highest number of connections out of 
        for (i,j) in &connections {
            if *j == max_connections {
                println!("Subreddit with ID: {:?} has {:?} outgoing connections, this is the highest number of outgoing hyperlinks", i, max_connections);}
        }

        // finalizing this new vector of tuples
        for (_i,j) in connections {
            // i = subreddit number, j = number of outgoing nodes from subreddit i
            xy[j].1 += 1; // incrementing connection j by 1
        }
        xy
    }
}
pub mod plotting_stuff{
    // got this code from github
    use plotters::prelude::*;
    pub fn plot_graph(xy: Vec<(usize, usize)>) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("graphs/0.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Subreddit and hyperlink distribution", ("sans-serif", 50).into_font())
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
            .label("Line showing outgoing hyperlink distribution")
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
#[cfg(test)]
mod tests{
    use crate::file_and_hashmap_stuff;

    #[test]
    pub fn test_number_of_sub_pairs(){
        // this should be equal to the number of records in the data, which is 286561
        let results = file_and_hashmap_stuff::read_file();
        assert_eq!(286561, results.1.len())
    }
    #[test]
    pub fn test_number_of_unique_subs(){
        // this should be equal to the number of unique subreddits, which is 35776
        let results = file_and_hashmap_stuff::read_file();
        assert_eq!(35776, results.0.len())
    }
}

