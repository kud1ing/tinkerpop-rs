extern crate rucaja;

use rucaja::{jobject, jvalue_from_jobject, Jvm, JvmClass};
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

        let graph_add_vertex = jvm.get_static_method(
            &class, "graph_add_vertex",
            "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)Lorg/apache/tinkerpop/gremlin/structure/Vertex;"
        ).expect("Could not find Java method `graph_add_vertex()`");


        let tinkergraph_new = jvm.get_static_method(
            &class, "tinkergraph_new",
            "()Lorg/apache/tinkerpop/gremlin/structure/Graph;"
        ).expect("Could not find Java method `tinkergraph_new()`");

        let println = jvm.get_static_method(
            &class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find Java method `println()`");

        let vertex_add_edge = jvm.get_static_method(
            &class, "vertex_add_edge",
            "(Lorg/apache/tinkerpop/gremlin/structure/Vertex;Ljava/lang/String;Lorg/apache/tinkerpop/gremlin/structure/Vertex;)Lorg/apache/tinkerpop/gremlin/structure/Edge;"
        ).expect("Could not find Java method `vertex_add_edge()`");


        println!("* create a `TinkerGraph`");
        let graph = jvm.call_static_object_method(&class, &tinkergraph_new, null());

        println!("* add two vertices");
        let args = vec![jvalue_from_jobject(graph)];
        let vertex1 = jvm.call_static_object_method(&class, &graph_add_vertex, args.as_ptr());
        let vertex2 = jvm.call_static_object_method(&class, &graph_add_vertex, args.as_ptr());

        // TODO: Adding edges requires Java strings. Rucaja's `new_jstring()` curently returns internd strings, see
        // https://github.com/kud1ing/rucaja/issues/9
        /*
        println!("* add an edge between the vertices");
        let args = vec![
            jvalue_from_jobject(vertex1),
            jvalue_from_jobject(jvm.new_jstring("likes")),
            jvalue_from_jobject(vertex2),
        ];
        let edge = jvm.call_static_object_method(&class, &vertex_add_edge, args.as_ptr());
        */

        println!("* print the `TinkerGraph` object using Java's `System.out.println()`:");
        let args = vec![jvalue_from_jobject(graph)];
        jvm.call_static_void_method(&class, &println, args.as_ptr());
    }
}
