use std::error::Error;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;



fn main(){
    let mut map = creategraph(); // Creates result ( Graph struct with data )

    let unwrap_map = map.unwrap(); // Unwraps to just Graph struct
    let (sum, pair_count) = findsumofdistance(&unwrap_map);
    println!("{}", pair_count);
    let degrees = sum as f64 / pair_count as f64; // sum divided by pair count, gives avg distance
    println!("The usual degrees of separation is : {}", degrees);
    

}

type Vertex = String;


#[derive(Debug)]
struct Graph {
    vertices: Vec<String>,
    neighbors: HashMap<Vertex, Vec<Vertex>>,
}

fn creategraph() -> Result<Graph, Box<dyn Error>> { // Creates a graph structure from a CSV with two columns of strings
    // Read the CSV file
    let filename = "dataset1.csv";
    let contents = fs::read_to_string(filename)?;

    // CSV data into a Graph struct
    let mut graph = Graph {
        vertices: Vec::new(),
        neighbors: HashMap::new(),
    };
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let source = parts[0].to_string();
        let target = parts[1].to_string();
        if !graph.vertices.contains(&source) {
            graph.vertices.push(source.clone());
        }
        let neighbors = graph.neighbors.entry(source).or_insert(Vec::new());
        if !neighbors.contains(&target) {
            neighbors.push(target);
     
        }
    }

    Ok(graph)
}



fn compute_and_print_distance_bfs(start: Vertex, graph: &Graph) -> usize { // Cycles through vertices using BFS
    let mut vertex_to_index: HashMap<Vertex, usize> = HashMap::new();
    for (i, vertex) in graph.vertices.iter().enumerate() {
        vertex_to_index.insert(vertex.clone(), i);
    }

    let start_index = match vertex_to_index.get(&start) {
        Some(index) => *index,
        None => return 0,
    };
    let mut distance: Vec<Option<u32>> = vec![None; graph.vertices.len()];
    distance[start_index] = Some(0); // <= we know this distance
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        let v_index = match vertex_to_index.get(&v) {
            Some(index) => *index,
            None => continue,
        };
        for u in match graph.neighbors.get(&v) {
            Some(neighbors) => neighbors,
            None => continue,
        } {
            let u_index = match vertex_to_index.get(u) {
                Some(index) => *index,
                None => continue,
            };
            if let None = distance[u_index] { // consider all unprocessed neighbors of v
                distance[u_index] = Some(distance[v_index].unwrap() + 1);
                queue.push_back(u.clone());
            }
        }
    }
    let mut sum = 0;
    for y in distance {
        sum += y.unwrap_or(0); // sum distances 
    }
    return sum.try_into().unwrap();
}



fn findsumofdistance(graph: &Graph) -> (usize, usize) { // Runs compute_and_print for every vertices and sums outputs. 
    let mut sum: usize = 0;
    let mut pair_count: usize = 0;
    for vertex in &graph.vertices {
        let distance = compute_and_print_distance_bfs(vertex.clone(), &graph);
        if distance > 0 {
            pair_count += 1; // add for every non-zero distance
        }
        sum += distance; //sum sums of distances
    }
    return (sum, pair_count);
}

