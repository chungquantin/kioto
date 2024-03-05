fn main() {
    tokio_xv6::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    println!("Hello, world!");
}
