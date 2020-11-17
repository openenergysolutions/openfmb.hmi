fn main() {
    prost_build::compile_protos(&["src/microgrid.proto"], &["src/"]).unwrap();
}
