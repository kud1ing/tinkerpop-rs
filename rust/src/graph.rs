use rucaja::{JvmAttachment, JvmClass, JvmMethod, JvmObject, JvmString, jvalue_from_jobject};
use std::ptr::null;


///
pub struct Graph<'a> {

    // The JVM.
    jvm_attachment: &'a JvmAttachment,

    // The wrapper JVM class from our fat JAR.
    wrapper_class: JvmClass<'a>,

    // Methods from that wrapper class.
    method_add_edge_between_vertices: JvmMethod,
    method_add_vertex_to_graph: JvmMethod,
    method_println: JvmMethod,

    // A `TinkerPop` graph object.
    object_graph: JvmObject<'a>,
}

impl<'a> Graph<'a> {

    ///
    pub fn add_edge(&self, vertex1: &JvmObject, predicate: &JvmString, vertex2: &JvmObject) -> Option<JvmObject> {

        let args = vec![
            jvalue_from_jobject(vertex1.jvm_ptr()),
            jvalue_from_jobject(predicate.jvm_ptr()),
            jvalue_from_jobject(vertex2.jvm_ptr()),
        ];

        JvmMethod::call_static_object_method(
            self.jvm_attachment,
            &self.wrapper_class,
            &self.method_add_edge_between_vertices,
            args.as_ptr()
        )
    }

    ///
    pub fn add_vertex(&self) -> Option<JvmObject> {

        let args = vec![jvalue_from_jobject(self.object_graph.jvm_ptr())];

        JvmMethod::call_static_object_method(
            self.jvm_attachment,
            &self.wrapper_class,
            &self.method_add_vertex_to_graph,
            args.as_ptr()
        )
    }

    ///
    pub fn new(jvm_attachment: &JvmAttachment) -> Option<Graph> {

        // Resolve the Java wrapper class from the fat JAR.
        let wrapper_class = JvmClass::get_class(jvm_attachment, "TinkerPopWrapper").expect("Could not find `TinkerPopWrapper`");

        // Resolve Java methods in that wrapper class.
        let method_add_edge_between_vertices = JvmMethod::get_static_method(
            jvm_attachment,
            &wrapper_class, "add_edge_between_vertices",
            "(Lorg/apache/tinkerpop/gremlin/structure/Vertex;Ljava/lang/String;Lorg/apache/tinkerpop/gremlin/structure/Vertex;)Lorg/apache/tinkerpop/gremlin/structure/Edge;"
        ).expect("Could not find Java method `add_edge_between_vertices()`");

        let method_add_vertex_to_graph = JvmMethod::get_static_method(
            jvm_attachment,
            &wrapper_class, "add_vertex_to_graph",
            "(Lorg/apache/tinkerpop/gremlin/structure/Graph;)Lorg/apache/tinkerpop/gremlin/structure/Vertex;"
        ).expect("Could not find Java method `add_vertex_to_graph()`");

        let method_new_tinkergraph = JvmMethod::get_static_method(
            jvm_attachment,
            &wrapper_class, "new_tinkergraph",
            "()Lorg/apache/tinkerpop/gremlin/structure/Graph;"
        ).expect("Could not find Java method `new_tinkergraph()`");

        let method_println = JvmMethod::get_static_method(
            jvm_attachment,
            &wrapper_class,
            "println",
            "(Ljava/lang/Object;)V"
        ).expect("Could not find Java method `println()`");

        // Create a `TinkerGraph` object.
        let object_graph = JvmMethod::call_static_object_method(
            jvm_attachment,
            &wrapper_class,
            &method_new_tinkergraph,
            null()
        ).unwrap();

        Some(
            Graph {
                jvm_attachment: jvm_attachment,
                wrapper_class: wrapper_class,
                method_add_edge_between_vertices: method_add_edge_between_vertices,
                method_add_vertex_to_graph: method_add_vertex_to_graph,
                method_println: method_println,
                object_graph: object_graph,
            }
        )
    }

    ///
    pub fn println(&self) {

        let args = vec![jvalue_from_jobject(self.object_graph.jvm_ptr())];

        JvmMethod::call_static_void_method(
            self.jvm_attachment,
            &self.wrapper_class,
            &self.method_println,
            args.as_ptr()
        )
    }
}