use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个线程，并使用 join 来等待线程完成
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("线程 1 正在运行: {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("主线程正在运行: {}", i);
        thread::sleep(std::time::Duration::from_millis(500));
    }

    // 等待线程完成
    handle.join().unwrap();

    // 使用 mpsc 来在线程之间传递消息
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        println!("线程 2 已发送消息");
    });

    let received = rx.recv().unwrap();
    println!("主线程接收到消息: {}", received);

    // 使用 Arc 和 Mutex 来在线程之间共享数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成后计数器的值: {}", *counter.lock().unwrap());
}
