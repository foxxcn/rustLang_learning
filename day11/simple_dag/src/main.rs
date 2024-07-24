fn main() {
    // This function can be used to manually test DAG functionality.
    let mut dag = simple_dag::DAG::new();
    dag.add_node("A");
    dag.add_node("B");
    dag.add_node("C");

    match dag.add_edge("A", "B") {
        Ok(_) => println!("Edge added successfully."),
        Err(e) => println!("Failed to add edge: {}", e),
    }

    match dag.add_edge("B", "C") {
        Ok(_) => println!("Edge added successfully."),
        Err(e) => println!("Failed to add edge: {}", e),
    }

    match dag.add_edge("C", "A") {
        Ok(_) => println!("Edge added successfully."),
        Err(e) => println!("Failed to add edge: {}", e),
    }

    println!("{:?}", dag);
}
