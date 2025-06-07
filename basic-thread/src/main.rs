use std::thread;

fn f() {
    for i in 1..10 {
        println!("Hello {} from thread {:?}", i, thread::current().id());
    }
}

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from main thread");

    let numbers = vec![1, 2, 3];
    let t3 = thread::spawn(move || {
        for  n in numbers.clone() {
            println!("{}", n);
        }
        let number_sum: i32 = numbers.iter().sum();
        return number_sum;
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let res = match t3.join() {
        Ok(res) => res,
        Err(_) => panic!("Error joining thread"),
    };
    println!("Sum of numbers: {}", res);

    let _t4 = std::thread::Builder::new()
        .name("my thread".to_string())
        .spawn(f)
        .unwrap()
        .join()
        .unwrap();
}