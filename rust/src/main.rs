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

        let class = jvm.get_class("TinkerpopWrapper").expect("Could not find `TinkerpopWrapper`");
        let println = jvm.get_static_method(&class, "println", "(Ljava/lang/Object;)V").expect("Could not find `println()`");

        let tinkergraph_new = jvm.get_static_method(&class, "tinkergraph_new", "()Lorg/apache/tinkerpop/gremlin/structure/Graph;").expect("Could not find `tinkergraph_new()`");
        let graph = jvm.call_static_object_method(&class, &tinkergraph_new, null());

        let args = vec![jvalue_from_jobject(graph)];
        jvm.call_static_void_method(&class, &println, args.as_ptr());
    }
}
