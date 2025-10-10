fn main() {
    #[cfg(feature="postgres")]
    println!("cargo:rerun-if-changed=migrations/postgres");
}