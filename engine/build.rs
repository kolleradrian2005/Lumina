fn main() {
    println!("cargo:rerun-if-changed=src/engine");
    println!("cargo:rerun-if-changed=src/game");
    println!("cargo:rerun-if-changed=assets");
}
