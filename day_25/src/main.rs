use graph::Graph;

mod graph;

fn main() {
    let graph = Graph::load_from_file("input");
    let (ours, theirs) = graph.scan(1);
    println!("ans (p1): {}", ours.len() * theirs.len());
}
