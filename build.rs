fn main() {
    println!("cargo::rerun-if-changed=save_server.mod.zip");
    println!("cargo::rerun-if-changed=web");
}
