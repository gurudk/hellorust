
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let shared_map:Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    {
        let mut map:RefMut<_> = shared_map.borrow_mut();
        map.insert("kew", 1);
        map.insert("shieber", 2);

    }

    let total:i32 = shared_map.borrow().values().sum();
    println!("total:{}", total);
    println!("Ref counts:{}", Rc::strong_count(&shared_map));
}
