use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    {
        let r = data.borrow();
        println!("data: {}", *r);
    }

    {
        let mut w = data.borrow_mut();
        *w += 1;
    }

    {
        let r = data.borrow();
        println!("data: {}", *r);
    }
}

