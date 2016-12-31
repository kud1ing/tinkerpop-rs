import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.tinkergraph.structure.TinkerGraph;

/**
 * A wrapper class for Apache TinkerPop to be used from Rust code.
 */
public class TinkerPopWrapper {

    /**
     * A wrapper for `System.out.println()` that is easier to use from Rust.
     * @param o
     */
    public static void println(Object o) {
        System.out.println(o);
    }

    public static void print_graph(Graph g) {
        System.out.println(g);
    }

    /**
     * Creates and returns a `TinkerGraph`.
     * @return
     */
    public static Graph tinkergraph_new() {
        return TinkerGraph.open();
    }

    public static void main(String[] args) {
        System.out.println(tinkergraph_new());
    }
}
