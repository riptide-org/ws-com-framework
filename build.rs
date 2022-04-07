fn main() {
    // Warn use of both server and client at the same time, this is because there are conflicting implemenations if
    // they are both used. At some future date we could be more specific with our feature flags to make this not a problem
    // but this is an acceptable bandaid solution.
    //TODO: add feature called "nowarn" which removes this warning
    #[cfg(all(feature = "client", feature = "server"))]
    println!("cargo:warning=both server and client features enabled, this may cause unexpected behaviour");

    println!("cargo:rerun-if-changed=src/message.proto");
    prost_build::compile_protos(&["src/message.proto"], &["src/"]).unwrap();
}
