fn main() {
    // Tell Cargo to rerun this build script if anything in the migrations/ folder changes
    println!("cargo:rerun-if-changed=migrations");
}
