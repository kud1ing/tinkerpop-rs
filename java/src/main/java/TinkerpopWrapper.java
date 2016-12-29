import org.apache.tinkerpop.gremlin.structure.Graph;
import org.apache.tinkerpop.gremlin.tinkergraph.structure.TinkerGraph;

public class TinkerpopWrapper {

  public static void println(Object object) {
    System.out.println(object);
  }

  public static Graph tinkergraph_new() {
    return TinkerGraph.open();
  }

  public static void main(String[] args) {
    System.out.println(tinkergraph_new());
  }
}
