fn main() {
    let runtime = kioto::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    runtime.block_on(async {
        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(10);
        let now = time::Instant::now();

        thread::sleep(ten_millis);

        println!("Done => {:?}", now.elapsed());
    });

    println!("Hello, world!");
}
