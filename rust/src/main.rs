extern crate rucaja;
extern crate tinkerpop_rs;

use rucaja::{Jvm, JvmAttachment, JvmString};
use tinkerpop_rs::graph::Graph;


fn main() {

    // The class path must contain the fat JAR.
    let class_path = "-Djava.class.path=../java/build/libs/tinkerpop.jar";

    let jvm_options = [class_path];

    // Instantiate the embedded JVM.
    let jvm = Jvm::new(&jvm_options);

    // Attach the current native thread to the JVM.
    let jvm_attachment = JvmAttachment::new(jvm.jvm());

    // Instantiate a `TinkerGraph`.
    let graph = Graph::new(&jvm_attachment).expect("Could not instantiate graph");

    // Add vertices.
    let v1 = graph.add_vertex().expect("Could not add vertex to graph");
    let v2 = graph.add_vertex().expect("Could not add vertex to graph");

    // Add an edge between those vertices.
    let p = JvmString::new(&jvm_attachment, "likes").expect("Could not create string");
    graph.add_edge(&v1, &p, &v2).expect("Could not add edge");

    // Print the graph using Java's `System.out.println()`.
    graph.println();
}
