fn main() {
    println!("Hello, world!");
    let mut bh = BinaryHeap::new();
    bh.push(10);
    bh.push(11);
    bh.push(14);
    bh.push(6);
    bh.push(3);

    println!("bh:{:?}", bh);

    bh.pop();

    println!("bh:{:?}", bh);
    bh.push(12);

    println!("bh:{:?}", bh);

    while bh.size > 0 {
        println!("{:?}", bh.pop());
    }
}

#[derive(Debug)]
struct BinaryHeap {
    size: usize,
    data: Vec<i32>,
}

impl BinaryHeap {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn push(&mut self, elem: i32) {
        self.data.push(elem);
        self.size += 1;
        if self.size > 1 {
            self.move_up(self.size - 1)
        }
    }

    fn move_up(&mut self, index: usize) {
        let mut child = index;
        let mut parent = child >> 1;
        if self.data[child] < self.data[parent] {
            self.data.swap(child, parent);
            if parent > 0 {
                self.move_up(parent);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        if self.size == 1 {
            return self.data.pop();
        } else {
            self.data.swap(0, self.size - 1);
            let ret = self.data.pop();
            self.size -= 1;
            self.move_down(0);

            return ret;
        }
    }

    fn move_down(&mut self, index: usize) {
        let mut parent = index;
        let mut left = 2 * parent;
        let mut right = 2 * parent + 1;
        let min = |x, y| {
            if self.data[x] < self.data[y] {
                x
            } else {
                y
            }
        };

        if left < self.size {
            let min_child = if right < self.size {
                //double child
                min(left, right)
            } else {
                //only left child
                left
            };
            self.data.swap(parent, min_child);
            self.move_down(min_child);
        }
    }
}
