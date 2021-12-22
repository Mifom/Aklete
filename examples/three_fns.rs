use aklete::Aklete;
use async_std::task;
use std::time::Duration;

async fn first_to_call() {
    task::sleep(Duration::new(5, 0)).await;
    println!("Second line!");
}

async fn second_to_call() {
    println!("Third line!");
}

async fn parallel_call() {
    println!("First line!");
}

fn main() {
    let mut aklete = Aklete::new();
    aklete.spawn(async {
        first_to_call().await;
        second_to_call().await;
        Ok(())
    });

    aklete.spawn(async {
        parallel_call().await;
        Ok(())
    });
    aklete.run().unwrap();
}
