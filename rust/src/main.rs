extern crate rucaja;

use rucaja::{jvalue_from_jobject, Jvm};
use std::ptr::null;


fn main() {

    let class_path = "-Djava.class.path=../java/build/libs/tinkerpop.jar";

    let jvm_options = [
        class_path
    ];

    unsafe {
        let jvm = Jvm::new(&jvm_options);

        // Resolve the Java wrapper class.
        let wrapper_class = jvm.get_class("TinkerPopWrapper").expect("Could not find `TinkerPopWrapper`");

        // Resolve Java methods in that wrapper class.
        let add_vertex_to_graph = jvm.get_static_method(
            &wrapper_class, "add_vertex_to_graph",
            "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)Lorg/apache/tinkerpop/gremlin/structure/Vertex;"
        ).expect("Could not find Java method `add_vertex_to_graph()`");

        let new_tinkergraph = jvm.get_static_method(
            &wrapper_class, "new_tinkergraph",
            "()Lorg/apache/tinkerpop/gremlin/structure/Graph;"
        ).expect("Could not find Java method `new_tinkergraph()`");

        let println = jvm.get_static_method(
            &wrapper_class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find Java method `println()`");

        let add_edge_between_vertices = jvm.get_static_method(
            &wrapper_class, "add_edge_between_vertices",
            "(Lorg/apache/tinkerpop/gremlin/structure/Vertex;Ljava/lang/String;Lorg/apache/tinkerpop/gremlin/structure/Vertex;)Lorg/apache/tinkerpop/gremlin/structure/Edge;"
        ).expect("Could not find Java method `add_edge_between_vertices()`");

        // Create a `TinkerGraph`.
        let graph = jvm.call_static_object_method(&wrapper_class, &new_tinkergraph, null()).unwrap();

        // Add 2 vertices to the graph.
        let args = vec![jvalue_from_jobject(*graph.jvm_object_ptr())];
        let vertex1 = jvm.call_static_object_method(&wrapper_class, &add_vertex_to_graph, args.as_ptr()).unwrap();
        let vertex2 = jvm.call_static_object_method(&wrapper_class, &add_vertex_to_graph, args.as_ptr()).unwrap();
        let predicate = jvm.new_jvm_string("likes").unwrap();

        // Add an edge between those vertices.
        let args = vec![
            jvalue_from_jobject(*vertex1.jvm_object_ptr()),
            jvalue_from_jobject(*predicate.jvm_string_ptr()),
            jvalue_from_jobject(*vertex2.jvm_object_ptr()),
        ];
        let _ = jvm.call_static_object_method(&wrapper_class, &add_edge_between_vertices, args.as_ptr());

        // Print the graph using Java's `System.out.println()`.
        let args = vec![jvalue_from_jobject(*graph.jvm_object_ptr())];
        jvm.call_static_void_method(&wrapper_class, &println, args.as_ptr());
    }
}
