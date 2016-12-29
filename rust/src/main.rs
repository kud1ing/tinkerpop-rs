extern crate rucaja;
extern crate tinkerpop_rs;

use rucaja::Jvm;
use tinkerpop_rs::TinkerGraphClass;


fn main() {

    // The JARs need to be in the correct order: each added JAR should be without any unsatisfied
    // dependencies up to that point.
    let class_path =
"-Djava.class.path=\"\
slf4j-api-1.7.5.jar:\
snakeyaml-1.15.jar:\
commons-lang-2.6.jar:\
commons-logging-1.1.1.jar:\
log4j-1.2.17.jar:\
slf4j-api-1.7.21.jar:\
jcabi-log-0.14.jar:\
hppc-0.7.1.jar:\
jcabi-manifests-1.1.jar:\
commons-configuration-1.10.jar:\
gremlin-shaded-3.2.1.jar:\
javatuples-1.2.jar:\
slf4j-log4j12-1.7.21.jar:\
jcl-over-slf4j-1.7.21.jar:\
commons-lang3-3.3.1.jar:\
gremlin-core-3.2.1.jar:\
tinkergraph-gremlin-3.2.1.jar:\
.\"";

    let jvm_options = [
        class_path,
        //"-verbose:class",
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    unsafe {
        let jvm = Jvm::new(&jvm_options);

        let wrapper_class = jvm.get_class("TinkerpopWrapper").expect("Could not find JVM class");

        #[allow(non_snake_case)]
        let TinkerGraph = TinkerGraphClass::new(&jvm).expect("Could not find `TinkerGraph` class");

    }
}
