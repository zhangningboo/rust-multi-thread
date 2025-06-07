fn main() {
    
    let numbers = vec![0, 1, 2, 3, 4, 5];
    let result = std::thread::scope(|scope| {

        let t1 = scope.spawn(|| {
            for n in &numbers {
                println!("first scope thread {}", n);
            }
            3
        });

        let t2 = scope.spawn(|| {
            for n in &numbers {
                println!("second scope thread {}", n);
            }
            4
        });
        // 会被自动join，确保scope内生成的线程不会超出scope
        t1.join().unwrap() + t2.join().unwrap()
    });
    // std::thread::scope 返回的不是Result
    println!("{}", result);

    static X: [i32; 3] = [1, 2, 3];
    dbg!(&X);
}
