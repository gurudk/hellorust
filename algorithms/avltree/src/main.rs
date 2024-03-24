pub mod asciidraw;

use crate::asciidraw::AsciiDraw;

fn main() {
    println!("Hello, world!");

    let mut avl = AvlTree::new(5);
    avl.insert(3);
    avl.insert(10);
    avl.insert(6);
    avl.insert(13);
    avl.insert(1);
    avl.insert(4);
    avl.insert(18);
    avl.insert(20);

    println!("tree:{:?}", &avl);

    avl.levelorder();

    // if let Some(rcnode) = &avl.root {
    //     let root = rcnode.borrow();
    //     println!("left key:{:?}", &root.left_key());
    // }

    println!("size:{}", avl.size());
    println!("height:{}", avl.height());

    let n20 = avl.get(20);
    println!("n20:{:?}", n20);

    let n6 = avl.get(3);
    println!("n6:{:?}", n6);

    avl.calculate_level_position();
    println!("==================================================");

    println!("avl:{:?}", avl);
    let mut b = 1;
    b <<= 1;
    b += 1;
    println!("{}", b);
    let mut v: Vec<Option<usize>> = vec![None; 10];
    v[0] = Some(1);
    v[9] = Some(12);
    println!("{:?}", v);
    // for i in 0..vec.len(){
    //     println!("{:?}", vec[i]);
    // }

    let mut asc = AsciiDraw::new(100, 100, ' ');
    asc.line(20, 20, 20, 40, '-')
        .line(40, 40, 20, 40, '-')
        .circle(40, 40, 10, 'd')
        .circle(40, 40, 8, '.')
        .line(0, 0, 20, 20, '\\')
        .line(40, 0, 20, 20, '/')
        .line(0, 0, 15, 20, '.')
        .line(10, 80, 100, 10, '=')
        .draw_box(50, 70, 2, String::from("abcd"))
        .draw_circle(50, 80, 3, '.', 5, String::from("3"))
        .draw_box(60, 70, 1, String::from("4"))
        .draw_box_center(20, 20, 1, String::from("8"))
        // .draw_circle(20, 20, 3, '#', 5, String::from("3"))
        .render();

    let nodes = avl.level_values(0);
    let node = nodes[0];
    println!("{:?}", node);
    println!("{:?}", avl.level_values(4));
    avl.draw_list_horizontal(&mut asc, 5, 90, 10, *avl.level_values(2));
}

use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct AvlTree<T>
where
    T: Copy + Ord + ToString,
{
    root: Option<Node<T>>,
}

#[derive(Debug)]
struct AvlNode<T> {
    key: Option<T>,
    left: Option<Node<T>>,
    right: Option<Node<T>>,
    parent: Option<WeakNode<T>>,
    factor: i32,
    level: usize,
    level_position: usize,
}

type Node<T> = Rc<RefCell<AvlNode<T>>>;
type WeakNode<T> = Weak<RefCell<AvlNode<T>>>;

impl<T: Ord + Copy + Debug + ToString> AvlTree<T> {
    fn new(key: T) -> Self {
        Self {
            root: Some(AvlNode::new_node(key)),
        }
    }

    fn insert(&mut self, key: T) {
        if let Some(root) = &self.root {
            let (is_left, deepened) = self.insert_at(key, root);
        } else {
            self.root = Some(AvlNode::new_node(key));
        }
    }

    fn calculate_level_position(&self) {
        if let Some(root) = &self.root {
            self._cal_position(root);
        }
    }

    fn _cal_position(&self, node: &Node<T>) {
        let mut avlnode = node.borrow_mut();
        match &avlnode.left {
            Some(l) => {
                l.borrow_mut().level_position = avlnode.level_position << 1;
                self._cal_position(l);
            }
            None => (),
        }

        match &avlnode.right {
            Some(r) => {
                let mut rt = avlnode.level_position;
                rt = rt << 1;
                r.borrow_mut().level_position = rt + 1;
                self._cal_position(r);
            }
            None => (),
        }
    }

    fn insert_at(&self, key: T, atnode: &Node<T>) -> (bool, bool) {
        let mut node = atnode.borrow_mut();
        let mut ret = (false, false);
        if let Some(k) = node.key {
            if key == k {
                return ret;
            }

            if key < k {
                match &node.left {
                    None => {
                        //new left node
                        let new_node = AvlNode::new_node_with_parent(key, atnode);
                        new_node.borrow_mut().level = node.level + 1;
                        node.left = Some(new_node);
                        node.factor += 1;

                        if node.right.is_none() {
                            ret = (true, true);
                        } else {
                            ret = (true, false);
                        }
                    }
                    Some(lnode) => {
                        ret = self.insert_at(key, lnode);
                        let (is_left, deepened) = ret;
                        if deepened {
                            node.factor += 1;
                        }
                    }
                }
            } else {
                match &node.right {
                    None => {
                        let new_node = AvlNode::new_node_with_parent(key, atnode);
                        new_node.borrow_mut().level = node.level + 1;
                        node.right = Some(new_node);
                        node.factor -= 1;

                        if node.left.is_none() {
                            ret = (false, true);
                        } else {
                            ret = (false, false);
                        }
                    }
                    Some(rnode) => {
                        ret = self.insert_at(key, rnode);

                        let (is_left, deepened) = ret;
                        if deepened {
                            node.factor -= 1;
                        }
                    }
                }
            }
        } else {
            node.key = Some(key);
        }

        ret
    }

    fn levelorder(&self) {
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_front(Rc::clone(&root));
        }

        while !queue.is_empty() {
            if let Some(node) = &queue.pop_back() {
                let avlnode = node.borrow();
                println!(
                    "key={:?}, factor={:?}, level={:?}",
                    avlnode.key, avlnode.factor, avlnode.level
                );
                if let Some(lnode) = &avlnode.left {
                    queue.push_front(Rc::clone(&lnode));
                }

                if let Some(rnode) = &avlnode.right {
                    queue.push_front(Rc::clone(&rnode));
                }
            }
        }
    }

    fn get(&self, key: T) -> Option<Node<T>> {
        match &self.root {
            Some(root) => {
                return self._get(key, root);
            }
            None => {
                return None;
            }
        }
    }

    fn _get(&self, key: T, node: &Node<T>) -> Option<Node<T>> {
        let avlnode = node.borrow();
        if avlnode.key == Some(key) {
            return Some(Rc::clone(node));
        }
        let mut ret = None;
        if let Some(k) = avlnode.key {
            if key < k {
                match &avlnode.left {
                    Some(l) => {
                        ret = self._get(key, l);
                    }
                    None => (),
                }
            } else {
                match &avlnode.right {
                    Some(r) => {
                        ret = self._get(key, r);
                    }
                    None => (),
                }
            }
        }

        ret
    }

    fn size(&self) -> usize {
        match &self.root {
            Some(root) => {
                return self._size(root);
            }
            None => {
                return 0;
            }
        }
    }

    fn height(&self) -> usize {
        match &self.root {
            Some(root) => {
                return self._height(root);
            }
            None => {
                return 0;
            }
        }
    }

    fn _height(&self, node: &Node<T>) -> usize {
        let avlnode = node.borrow();
        let mut left_height = 0;
        let mut right_height = 0;
        match &avlnode.left {
            Some(l) => {
                left_height = self._height(l);
            }
            None => (),
        }

        match &avlnode.right {
            Some(r) => {
                right_height = self._height(r);
            }
            None => (),
        }

        1 + max(left_height, right_height)
    }

    fn _size(&self, node: &Node<T>) -> usize {
        let avlnode = node.borrow();

        let mut left_size = 0;
        let mut right_size = 0;
        match &avlnode.left {
            Some(l) => {
                left_size += self._size(l);
            }
            None => (),
        }

        match &avlnode.right {
            Some(r) => {
                right_size += self._size(r);
            }
            None => (),
        }

        left_size + right_size + 1
    }

    fn level_values(&self, height: usize) -> Box<Vec<Option<T>>> {
        let count = 2_usize.pow(height as u32);
        let mut vec: Vec<Option<T>> = vec![None; count];

        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_front(Rc::clone(&root));
        }

        while !queue.is_empty() {
            if let Some(node) = &queue.pop_back() {
                let avlnode = node.borrow();
                // println!(
                //     "key={:?}, factor={:?}, level={:?}",
                //     avlnode.key, avlnode.factor, avlnode.level
                // );
                if height == avlnode.level {
                    vec[avlnode.level_position] = Some(avlnode.key.unwrap().clone());
                }

                if let Some(lnode) = &avlnode.left {
                    queue.push_front(Rc::clone(&lnode));
                }

                if let Some(rnode) = &avlnode.right {
                    queue.push_front(Rc::clone(&rnode));
                }
            }
        }

        Box::new(vec)
    }

    fn render(&self) {}

    fn draw_list_horizontal(
        &self,
        pen: &mut AsciiDraw,
        x0: usize,
        y0: usize,
        interval: usize,
        list: Vec<Option<T>>,
    ) {
        for i in 0..list.len() {
            if !list[i].is_none() {
                let s = list[i].unwrap().to_string();
                let len = s.chars().count();
                pen.draw_box(x0 + i * interval, y0, len, s);
                println!("x,y={}{}", x0 + i * interval, y0);
            }
        }

        pen.render();
    }
}

impl<T: Ord + Copy> AvlNode<T> {
    fn new(key: T) -> Self {
        Self {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
            factor: 0,
            level: 0,
            level_position: 0,
        }
    }

    fn new_node(key: T) -> Node<T> {
        Rc::new(RefCell::new(Self {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
            factor: 0,
            level: 0,
            level_position: 0,
        }))
    }

    fn new_node_with_parent(key: T, p: &Node<T>) -> Node<T> {
        Rc::new(RefCell::new(Self {
            key: Some(key),
            left: None,
            right: None,
            parent: Some(Rc::downgrade(p)),
            factor: 0,
            level: 0,
            level_position: 0,
        }))
    }

    fn new_with_parent(key: T, p: Node<T>) -> Self {
        Self {
            key: Some(key),
            left: None,
            right: None,
            parent: Some(Rc::downgrade(&p)),
            factor: 0,
            level: 0,
            level_position: 0,
        }
    }

    fn left_key(&self) -> Option<T> {
        if let Some(l) = &self.left {
            l.borrow().key
        } else {
            None
        }
    }

    fn right_key(&self) -> Option<T> {
        if let Some(r) = &self.right {
            r.borrow().key
        } else {
            None
        }
    }
}
