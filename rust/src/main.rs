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
        println!("* Instantiate the embedded JVM.");
        let jvm = Jvm::new(&jvm_options);

        println!("* Get a reference to the Java wrapper class `TinkerPopWrapper`.");
        let class = jvm.get_class("TinkerPopWrapper").expect("Could not find `TinkerPopWrapper`");


        println!("* Get references to Java methods in that Java wrapper class.");

        let graph_add_vertex = jvm.get_static_method(
            &class, "graph_add_vertex",
            "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)Lorg/apache/tinkerpop/gremlin/structure/Vertex;"
        ).expect("Could not find `tinkergraph_new()`");

        let tinkergraph_new = jvm.get_static_method(
            &class, "tinkergraph_new",
            "()Lorg/apache/tinkerpop/gremlin/structure/Graph;"
        ).expect("Could not find `tinkergraph_new()`");

        let println = jvm.get_static_method(
            &class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find `println()`");


        println!("* instantiate a Java `TinkerGraph` object");
        let graph = jvm.call_static_object_method(&class, &tinkergraph_new, null());

        {
            println!("* add a vertex");
            let args = vec![jvalue_from_jobject(graph)];
            let vertex = jvm.call_static_object_method(&class, &graph_add_vertex, args.as_ptr());
        }

        {
            println!("* print the `TinkerGraph` object using Java's `System.out.println()`:");
            let args = vec![jvalue_from_jobject(graph)];
            jvm.call_static_void_method(&class, &println, args.as_ptr());
        }
    }
}
