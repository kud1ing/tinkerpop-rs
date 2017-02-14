extern crate rucaja;
extern crate tinkerpop_rs;

use rucaja::Jvm;
use tinkerpop_rs::graph::Graph;


fn main() {

    // The class path must contain the fat JAR.
    let class_path = "-Djava.class.path=../java/build/libs/tinkerpop.jar";

    let jvm_options = [class_path];

    unsafe {
        // Instantiate the embedded JVM.
        let jvm = Jvm::new(&jvm_options);

        // Instantiate a `TinkerGraph`.
        let graph = Graph::new(&jvm).expect("Could not instantiate graph");

        // Add vertices.
        let v1 = graph.add_vertex().expect("Could not add vertex to graph");
        let v2 = graph.add_vertex().expect("Could not add vertex to graph");

        // Add an edge between those vertices.
        let p = jvm.new_jvm_string("likes").expect("Could not create string");
        graph.add_edge(&v1, &p, &v2).expect("Could not add edge");

        // Print the graph using Java's `System.out.println()`.
        graph.println();
    }
}
