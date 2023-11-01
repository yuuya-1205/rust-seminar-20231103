use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(vec![1, 1, 2, 3, 5]);

    let mut handles = vec![];
    for i in 0..v.len() {
        let v = Arc::clone(&v);
        let handle = thread::spawn(move || {
            println!("Thread ID: {}, v[{}] = {}", i, i, v[i]);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
