use field::Field;

mod dir;
mod field;
mod graph;
mod pos;

fn main() {
    let mut field = Field::load_from_file("input");
    println!(
        "ans (p1): {}",
        field.to_graph().solve()
    );

    field.remove_slopes();
    let graph = field.to_graph();
    eprintln!("generated graph!");
    graph.generate_dot("graph0_p2.dot").unwrap();

    println!(
        "ans (p2): {}",
        graph.solve()
    );
}
