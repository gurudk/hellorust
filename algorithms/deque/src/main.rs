#[derive(Debug)]
struct Deque<T> {
    data: Vec<T>,
    cap: usize,
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            cap: cap,
        }
    }

    fn add_rear(&mut self, item: T) {
        self.data.insert(0, item);
    }

    fn add_front(&mut self, item: T) {
        self.data.push(item);
    }

    fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn remove_front(&mut self) -> Option<T> {
        if (self.is_empty()) {
            None
        } else {
            Some(self.data.remove(self.size() - 1))
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item)
        }

        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.data.remove(0))
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut dq = Deque::new(5);
    dq.add_front(1);
    dq.add_front(2);
    dq.add_rear(3);
    println!("{:?}", dq);
    println!("{:?}", dq.as_slice());
    assert_eq!(&[3, 1, 2], dq.as_slice());

    let it = dq.iter();
    for item in it {
        println!("{:?}", item);
    }

    let itmut = dq.iter_mut();
    for item in itmut {
        *item += 2;
    }

    assert_eq!(&[5, 3, 4], dq.as_slice());

    println!("{:?}", dq);
    println!("{:?}", dq.as_slice());

    assert_eq!(false, palindrome_checker("abc"));
    assert_eq!(true, palindrome_checker(""));
    assert_eq!(true, palindrome_checker("a"));
    assert_eq!(true, palindrome_checker("aba"));
    assert_eq!(true, palindrome_checker("bb"));
}

fn palindrome_checker(s: &str) -> bool {
    let mut dq: Deque<char> = Deque::new(10);
    for c in s.chars() {
        let _c = dq.add_front(c);
    }

    let mut is_pal = true;

    while dq.size() > 1 {
        let _f = dq.remove_front();
        let _r = dq.remove_rear();

        if _f != _r {
            is_pal = false;
        }
    }

    is_pal
}
