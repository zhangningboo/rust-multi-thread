use std::sync::Mutex;

fn main() {
    let n = Mutex::<i32>::new(0);
    std::thread::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // drop(guard)  会自动释放
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}
