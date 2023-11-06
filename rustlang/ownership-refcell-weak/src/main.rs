use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Car{
    name: String,
    wheels: RefCell<Vec<Weak<Wheel>>>,
}

struct Wheel {
    id: i32,
    car: Rc<Car>,
}
fn main() {
    let car:Rc<Car> = Rc::new(
        Car{
            name: "Tesla".to_string(),
            wheels: RefCell::new(vec![])
        }
    );

    let wl1 = Rc::new(Wheel{id:1,car:Rc::clone(&car)});
    let wl2 = Rc::new(Wheel{id:2, car:Rc::clone(&car)});

    let mut wheels = car.wheels.borrow_mut();
    wheels.push(Rc::downgrade(&wl1));
    wheels.push(Rc::downgrade(&wl2));

    for wheel_weak in wheels.iter(){
        let wl = wheel_weak.upgrade().unwrap();
        println!("wheel {} owned by {}", wl.id, wl.car.name);
    }
}
