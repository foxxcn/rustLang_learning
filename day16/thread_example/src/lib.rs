#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    #[test]
    fn test_thread_incrementation() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..5 {
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

        assert_eq!(*counter.lock().unwrap(), 5);
    }

    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("test");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        assert_eq!(received, "test");
    }
}
