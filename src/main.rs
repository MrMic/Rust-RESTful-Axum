use std::thread;

mod book;
mod data;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
async fn print_data() {
    thread::spawn(move || {
        let data = data::DATA.lock().unwrap();
        println!("{:?}", data);
    })
    .join()
    .unwrap();
}

#[tokio::main]
async fn main() {
    print_data().await;
}
