# Apache TinkerPop from Rust

An example showing how to call [Apache TinkerPop](https://tinkerpop.apache.org) from [Rust](https://www.rust-lang.org) via
[Rucaja](https://github.com/kud1ing/rucaja) (JNI).

This repository contains two directories:

* `java` contains a Java project to build a fat JAR `tinkerpop.jar` which contains a wrapper Java class around TinkerPop
  and all dependencies.
* `rust` contains Rust code which uses `tinkerpop.jar`.
    Run it using `cargo run`.
