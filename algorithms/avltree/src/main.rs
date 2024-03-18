fn main() {
    println!("Hello, world!");
    let mut avt = AvlTree::new(5);
    avt.insert(12);
    avt.insert(13);
    avt.insert(16);
    avt.insert(18);

    println!("{:?}", avt);
}

#[derive(Debug)]
struct AvlTree<T>
where
    T: Ord + Copy,
{
    data: Option<T>,
    left: Option<Box<AvlTree<T>>>,
    right: Option<Box<AvlTree<T>>>,
    factor: i16,
}

impl<T: Ord + Copy> AvlTree<T> {
    fn new(t: T) -> Self {
        AvlTree {
            data: Some(t),
            left: None,
            right: None,
            factor: 0,
        }
    }

    fn insert(&mut self, value: T) -> bool {
        let mut ret = false;
        match &self.data {
            Some(d) => {
                if value == *d {
                    self.data = Some(value);
                    ret = false;
                } else if value < *d {
                    match &mut self.left {
                        Some(l) => {
                            ret = l.insert(value);
                        }
                        //left is none
                        None => {
                            //左右节点都为空，插入节点作为当前左节点，当前因子加1
                            if self.right.is_none() {
                                ret = true;
                            } else {
                                ret = false;
                            }

                            self.factor += 1;
                            self.left = Some(Box::new(AvlTree::new(value)));
                        }
                    }
                } else {
                    match &mut self.right {
                        Some(r) => {
                            ret = r.insert(value);
                        }
                        None => {
                            if self.left.is_none() {
                                ret = true;
                            }else{
                                ret = false;
                            }

                            self.factor -= 1;
                            self.right = Some(Box::new(AvlTree::new(value)));
                        }
                    }
                }
            }
            //this node is none
            None => self.data = Some(value),
        }
    }
}
