extern crate rucaja;

use rucaja::{jvalue_from_jobject, Jvm};
use std::ptr::null;


fn main() {

    let class_path = "-Djava.class.path=../java/build/libs/tinkerpop.jar";

    let jvm_options = [
        class_path,
        //"-verbose:class",
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        println!("* instantiate the embedded JVM.");
        let jvm = Jvm::new(&jvm_options);

        println!("* get a reference to the Java wrapper class `TinkerPopWrapper`.");
        let class = jvm.get_class("TinkerPopWrapper").expect("Could not find `TinkerPopWrapper`");


        println!("* get references to Java methods in that Java wrapper class.");

        let add_vertex_to_graph = jvm.get_static_method(
            &class, "add_vertex_to_graph",
            "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)Lorg/apache/tinkerpop/gremlin/structure/Vertex;"
        ).expect("Could not find Java method `add_vertex_to_graph()`");

        let new_tinkergraph = jvm.get_static_method(
            &class, "new_tinkergraph",
            "()Lorg/apache/tinkerpop/gremlin/structure/Graph;"
        ).expect("Could not find Java method `new_tinkergraph()`");

        let println = jvm.get_static_method(
            &class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find Java method `println()`");

        let add_edge_between_vertices = jvm.get_static_method(
            &class, "add_edge_between_vertices",
            "(Lorg/apache/tinkerpop/gremlin/structure/Vertex;Ljava/lang/String;Lorg/apache/tinkerpop/gremlin/structure/Vertex;)Lorg/apache/tinkerpop/gremlin/structure/Edge;"
        ).expect("Could not find Java method `add_edge_between_vertices()`");


        println!("* create a `TinkerGraph`");
        let graph = jvm.call_static_object_method(&class, &new_tinkergraph, null());

        println!("* add two vertices");
        let args = vec![jvalue_from_jobject(graph)];
        let vertex1 = jvm.call_static_object_method(&class, &add_vertex_to_graph, args.as_ptr());
        let vertex2 = jvm.call_static_object_method(&class, &add_vertex_to_graph, args.as_ptr());

        println!("* add an edge between the vertices");
        let args = vec![
            jvalue_from_jobject(vertex1),
            jvalue_from_jobject(*jvm.new_jvm_string("likes").unwrap().jvm_string_ptr()),
            jvalue_from_jobject(vertex2),
        ];
        let _ = jvm.call_static_object_method(&class, &add_edge_between_vertices, args.as_ptr());

        println!("* print the `TinkerGraph` object using Java's `System.out.println()`:");
        let args = vec![jvalue_from_jobject(graph)];
        jvm.call_static_void_method(&class, &println, args.as_ptr());
    }
}
