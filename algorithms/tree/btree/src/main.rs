
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
struct BTree{
    count:usize,
    root:Rc<RefCell<BNode>>,
}


#[derive(Debug)]
struct BNode{
    data:i32,
    left:Option<Rc<RefCell<BNode>>>,
    right:Option<Rc<RefCell<BNode>>>,
    parent:Option<Weak<RefCell<BNode>>>,
}

impl BNode{
    pub fn new(data:i32)->Rc<RefCell<BNode>>{
        Rc::new(RefCell::new(BNode{
            data:data,
            left:None,
            right:None,
            parent:None,
        }))
    }

    pub fn new_with_parent(data:i32, parent:Rc<RefCell<BNode>>)->Rc<RefCell<BNode>>{
        let rnode = Rc::new(RefCell::new(BNode{
            data:data,
            left:None,
            right:None,
            parent:Some(Rc::downgrade(&parent)),
        }));
        (*parent).borrow_mut().right = Some(Rc::clone(&rnode));
        rnode
    }
}


fn main() {
    println!("Hello, world!");
    let root = Rc::new(RefCell::new(BNode{
        data:-1_i32,
        left:None,
        right:None,
        parent:None,
    }));

    let left1 = Rc::new(RefCell::new(BNode{
        data:222,
        left:None,
        right:None,
        parent:Some(Rc::downgrade(&root)),
    }));

    let right1 = BNode::new_with_parent(444, Rc::clone(&root));

    (*root).borrow_mut().left = Some(Rc::clone(&left1));

    let tree = BTree{count:2, root:Rc::clone(&root)};

    println!("{:?}", tree);

}
