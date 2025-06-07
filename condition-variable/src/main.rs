use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering}, 
        Condvar, 
        Mutex, Arc
    }
};

fn main() {
    // 使用 Arc 包装共享状态，保证线程安全
    struct SharedState {
        queue: VecDeque<i32>,
        finish_work: AtomicBool,
    }
    
    let shared = Arc::new((
        Mutex::new(SharedState {
            queue: VecDeque::new(),
            finish_work: AtomicBool::new(false),
        }),
        Condvar::new()
    ));

    std::thread::scope(|scope| {
        let shared_consumer = shared.clone();
        // 消费者线程
        scope.spawn(move || {
            loop {
                let (lock, cvar) = &*shared_consumer;
                let mut data = lock.lock().unwrap();
                
                // 等待条件：队列非空或生产者已结束
                while data.queue.is_empty() {
                    // 检查退出标志
                    if data.finish_work.load(Ordering::Acquire) {
                        println!("消费者退出");
                        return;
                    }
                    // 使用条件变量安全等待[1,3](@ref)
                    data = cvar.wait(data).unwrap();
                }
                
                // 处理队列数据
                if let Some(item) = data.queue.pop_front() {
                    drop(data); // 提前释放锁
                    dbg!(item);
                } else if data.finish_work.load(Ordering::Acquire) {
                    println!("消费者退出");
                    return;
                }
            }
        });

        // 生产者线程
        let (lock, cvar) = &*shared;
        for i in 0..10 {
            {
                let mut data = lock.lock().unwrap();
                data.queue.push_back(i);
                cvar.notify_one(); // 通知消费者[1](@ref)
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        // 设置退出标志并唤醒消费者
        {
            let data = lock.lock().unwrap();
            data.finish_work.store(true, Ordering::Release);
            cvar.notify_one(); // 关键唤醒[1,3](@ref)
            println!("生产者已完成，通知消费者退出");
        }
    });
}