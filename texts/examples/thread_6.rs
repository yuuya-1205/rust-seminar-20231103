use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        // moveキーワードでスレッドはcounterの所有権を奪う
        let handle = thread::spawn(move || {
            // lock()はResultを返す。
            // ロックを取得して中の値（i32）への書き込み権限を得る
            let mut num = counter.lock().unwrap();

            *num += 1;
        }); // numがスコープ外に出ると自動でロックを解除する。解除のし忘れ防止！
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
