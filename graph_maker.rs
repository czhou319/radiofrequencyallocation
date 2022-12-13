pub fn initialize(n: usize, data: &Vec<(u32, f32, f32)>, limit: f32) -> Vec<Vec<f32>> {
    let graph = fill_matrix(n, data, limit);
    return graph;
}
    
fn fill_matrix(n: usize, data: &Vec<(u32, f32, f32)>, limit: f32) -> Vec<Vec<f32>> {
    let mut graph = vec![vec![0.0;n];n];
    for i in 0..n {
        for j in 0..n {
            let p1 = (data[i].1, data[i].2);
            let p2 = (data[j].1, data[j].2);
            let edge_length = distance(p1, p2);
            if edge_length < limit {
                graph[i][j] = edge_length;
            }
        }
    }
    return graph;
}

fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    //Haversine formula
    let r = 6371.0;
    let point1 = (p1.0.to_radians(), p1.1.to_radians());
    let point2 = (p2.0.to_radians(), p2.1.to_radians());
    let delta_lat = (p2.0 - p1.0).to_radians();
    let delta_long = (p2.1 - p1.1).to_radians();
    let distance = 2.0*r*(((delta_lat/2.0).sin().powf(2.0)+((point1.0).cos()*(point2.0).cos()*((delta_long/2.0).sin().powf(2.0)))).powf(0.5)).asin();
    return distance
}