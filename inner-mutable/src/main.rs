use std::cell::{Cell, RefCell};  // 只能单线程使用

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        println!("changed");
    }
}

fn ref_cell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

fn main() {
    let n = Cell::<i32>::new(10);
    f(&n, &n);

    let v = RefCell::<Vec<i32>>::new(vec![]);
    ref_cell(&v);
    println!("{v:?}");
}
