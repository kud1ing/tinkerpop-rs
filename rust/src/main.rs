extern crate rucaja;

use rucaja::{Jvm, jvalue_from_jobject};
use std::ptr::null;

fn main() {

    let class_path = "-Djava.class.path=../java/build/libs/tinkerpop.jar:";

    let jvm_options = [
        class_path,
        //"-verbose:class",
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        let jvm = Jvm::new(&jvm_options);

        // Get a reference to the Java wrapper class.
        let class = jvm.get_class("TinkerpopWrapper").expect("Could not find `TinkerpopWrapper`");

        // Get references to Java methods in that Java class.
        let tinkergraph_new = jvm.get_static_method(&class, "tinkergraph_new", "()Lorg/apache/tinkerpop/gremlin/structure/Graph;").expect("Could not find `tinkergraph_new()`");
        let main = jvm.get_static_method(&class, "main", "([Ljava/lang/String;)V").expect("Could not find `main()`");
        let print_graph = jvm.get_static_method(&class, "print_graph", "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)V").expect("Could not find `main()`");
        let println = jvm.get_static_method(&class, "println", "(Ljava/lang/Object;)V").expect("Could not find `println()`");

        // Instantiate a Java `TinkerGraph`.
        let graph = jvm.call_static_object_method(&class, &tinkergraph_new, null());

        // Print the `TinkerGraph` via Java's `System.out.println()`.
        let args = vec![jvalue_from_jobject(graph)];
        jvm.call_static_void_method(&class, &println, args.as_ptr());
    }
}
