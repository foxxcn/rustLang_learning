use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Debug)]
struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }

    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

#[derive(Debug)]
struct SharedData {
    data: RwLock<String>,
}

impl SharedData {
    fn new() -> Self {
        SharedData {
            data: RwLock::new(String::new()),
        }
    }

    fn write_data(&self, new_data: &str) {
        let mut data = self.data.write().unwrap();
        *data = new_data.to_string();
    }

    fn read_data(&self) -> String {
        let data = self.data.read().unwrap();
        data.clone()
    }
}

fn main() {
    // 使用Arc共享Counter
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get_value());

    // 使用Arc共享SharedData
    let shared_data = Arc::new(SharedData::new());
    let shared_data_clone = Arc::clone(&shared_data);

    let writer = thread::spawn(move || {
        shared_data_clone.write_data("Hello, world!");
    });

    writer.join().unwrap();
    let reader = thread::spawn(move || {
        println!("Shared data: {}", shared_data.read_data());
    });

    reader.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_value(), 2);
    }

    #[test]
    fn test_shared_data() {
        let shared_data = SharedData::new();
        shared_data.write_data("Testing data");
        assert_eq!(shared_data.read_data(), "Testing data");
    }

    #[test]
    fn test_concurrent_counter() {
        let counter = Arc::new(Counter::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                counter.increment();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get_value(), 10);
    }

    #[test]
    fn test_concurrent_shared_data() {
        let shared_data = Arc::new(SharedData::new());
        let shared_data_clone = Arc::clone(&shared_data);

        let writer = thread::spawn(move || {
            shared_data_clone.write_data("Concurrent data");
        });

        writer.join().unwrap();

        let reader = thread::spawn(move || {
            assert_eq!(shared_data.read_data(), "Concurrent data");
        });

        reader.join().unwrap();
    }
}
