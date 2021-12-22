use aklete::Aklete;
use std::thread;
use std::time::Duration;
fn main() {
    let mut aklete = Aklete::new();
    aklete.spawn(async {
        thread::sleep(Duration::new(5, 0));
        println!("Hello, world!");
        Ok(())
    });

    aklete.run().unwrap();
}
