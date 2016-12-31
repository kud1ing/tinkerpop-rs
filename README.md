# Apache TinkerPop from Rust

An example showing how to call Apache TinkerPop from Rust.

There are two directories:

* `java` contains a Java project to build a fat JAR `tinkerpop.jar` which contains a wrapper Java class around TinkerPop
  and all dependencies.
* `rust` contains Rust code which uses `tinkerpop.jar`.