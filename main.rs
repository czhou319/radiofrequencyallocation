mod read_csv;
mod graph_maker;

fn main() {
    read_csv::check_csv("celltowers_cleaned.csv");
    let data = read_csv::read_file("celltowers_cleaned.csv");
    let limit = 100.0;
    let graph = graph_maker::initialize(data.len(), &data, limit);
    let mut graph_colors = graph_coloring(graph.len(), &graph);
    println!("colored verticies: {:?}", graph_colors);
    graph_colors.sort();
    println!("number of radio frequencies needed: {:?}", graph_colors[graph.len()-1]);
}
 
fn graph_coloring(n: usize, graph: &Vec<Vec<f32>>) -> Vec<usize> {
    let mut vertices = vec![0;n];
    vertices[0] = 1;
    let mut colors = vec![1];
    for v in 1..n {
        let mut unavailable = Vec::new();
        for past_v in 0..v {
            if graph[past_v][v] > 0.0 {
                if unavailable.contains(&vertices[past_v]) == false {
                    unavailable.push(vertices[past_v]);
                }
            }
        }
        unavailable.sort();
        if unavailable == colors {
            let new = colors.len()+1;
            colors.push(new);
            vertices[v] = new; 
        }
        else {
            for i in 0..colors.len() {
                if unavailable.contains(&colors[i]) == false {
                    vertices[v] = colors[i];
                }
            }
        }
    }
    return vertices
}
