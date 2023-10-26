#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("===========FnMut trait: once on each item, multiple calls ============");
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    println!("===========FnOnce trait: closure capture and move value out to closure ============");
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations:Vec<String> = vec![];
    let value = String::from("by key called");

    //sort_by_key need FnMut closure
    list.sort_by_key(|r| {
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);

    println!("===========FnOnce trait: once on each item, multiple calls ============");

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        println!("called in closure, num_sort_operations value:{}", num_sort_operations);
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
    println!("called out of closure, num_sort_operations value:{}", num_sort_operations);
}