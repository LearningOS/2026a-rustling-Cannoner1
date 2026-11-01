use std::sync::{Arc, Mutex}; // 修改导入
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 用 Mutex 包裹
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取锁并修改
            let mut status_guard = status_shared.lock().unwrap();
            status_guard.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        // 填???处：获取锁并读取
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}