import org.apache.tinkerpop.gremlin.structure.Edge;
import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.structure.Vertex;
import org.apache.tinkerpop.gremlin.tinkergraph.structure.TinkerGraph;

/**
 * A wrapper class for Apache TinkerPop to simplify access from Rust code.
 *
 * The Java method signatures can be seen using the Java command-line tool `javap`:
 *
 *     javap -s build/classes/main/TinkerPopWrapper.class
 */
public class TinkerPopWrapper {

    /**
     * Adds a vertex to the graph
     *
     * @param graph
     * @return a new vertex
     */
    public static Vertex add_vertex_to_graph(Graph graph) {
        return graph.addVertex();
    }

    /**
     * A wrapper for `System.out.println()` that is easier to use from Rust.
     * @param o
     */
    public static void println(Object o) {
        System.out.println(o);
    }

    /**
     * Creates and returns a `TinkerGraph`.
     *
     * @see <http://tinkerpop.apache.org/javadocs/3.2.3/full/org/apache/tinkerpop/gremlin/tinkergraph/structure/TinkerGraph.html>
     * @return a new `TinkerGraph` instance
     */
    public static Graph new_tinkergraph() {
        return TinkerGraph.open();
    }

    /**
     * Adds an edge between two vertices.
     *
     * @param vertex_out
     * @param label
     * @param vertex_in
     * @return
     */
    public static Edge add_edge_between_vertices(Vertex vertex_out, String label, Vertex vertex_in) {
        return vertex_out.addEdge(label, vertex_in);
    }

    public static void main(String[] args) {
        System.out.println(new_tinkergraph());
    }
}
