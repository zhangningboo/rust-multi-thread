use std::{
    collections::VecDeque,
    sync::{Mutex, atomic::{AtomicBool, Ordering}},
    thread
};

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());
    let finish_work = AtomicBool::new(false);  // 原子退出标志

    std::thread::scope(|scope| {
        // 消费者线程 - 使用 move 捕获原子标志
        let t = scope.spawn(|| {
            loop {
                // 检查退出标志
                if finish_work.load(Ordering::Acquire) {
                    break;
                }
                let item = queue.lock().unwrap().pop_front();
                if let Some(item) = item {
                    dbg!(item);
                } else {
                    // 队列为空时挂起
                    thread::park();
                }
            }
            println!("消费者线程正常退出");
        });

        // 生产者线程
        for i in 0..100 {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();  // 唤醒消费者
            thread::sleep(std::time::Duration::from_millis(100));  // 缩短等待时间
        }

        // 设置退出标志并唤醒消费者
        finish_work.store(true, Ordering::Release);
        t.thread().unpark();  // 确保消费者能退出
        println!("生产者已完成，通知消费者退出");
    });
}