pub mod asciidraw;
pub mod svgdraw;

use crate::asciidraw::AsciiDraw;
use crate::svgdraw::SvgDraw;

use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");

    let mut avl = AvlTree::new(50);

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..200);
    for i in 0..100 {
        avl.insert(uni.sample(&mut rng));
    }

    avl.calculate_level_position();

    // println!("tree:{:?}", &avl);

    // avl.levelorder();

    // if let Some(rcnode) = &avl.root {
    //     let root = rcnode.borrow();
    //     println!("left key:{:?}", &root.left_key());
    // }

    // println!("size:{}", avl.size());
    // println!("height:{}", avl.height());

    // let n20 = avl.get(20);
    // println!("n20:{:?}", n20);

    // let n6 = avl.get(3);
    // println!("n6:{:?}", n6);

    // avl.calculate_level_position();
    // println!("==================================================");

    // println!("avl:{:?}", avl);
    // let mut b = 1;
    // b <<= 1;
    // b += 1;
    // println!("{}", b);
    // let mut v: Vec<Option<usize>> = vec![None; 10];
    // v[0] = Some(1);
    // v[9] = Some(12);
    // println!("{:?}", v);
    // for i in 0..vec.len(){
    //     println!("{:?}", vec[i]);
    // }

    // let mut asc = AsciiDraw::new(100, 110, ' ');
    // asc.line(20, 20, 20, 40, '-')
    //     .line(40, 40, 20, 40, '-')
    //     .circle(40, 40, 10, 'd')
    //     .circle(40, 40, 8, '.')
    //     .line(0, 0, 20, 20, '\\')
    //     .line(40, 0, 20, 20, '/')
    //     .line(0, 0, 15, 20, '.')
    //     .line(10, 80, 100, 10, '=')
    //     .draw_box(50, 70, 2, String::from("abcd"))
    //     .draw_circle(50, 80, 3, '.', 5, String::from("3"))
    //     .draw_box(60, 70, 1, String::from("4"))
    //     .draw_box_center(20, 20, 1, String::from("8"))
    //     // .draw_circle(20, 20, 3, '#', 5, String::from("3"))
    //     .render();

    // let nodes = avl.level_values(0);
    // let node = nodes[0];
    // println!("{:?}", node);
    // println!("{:?}", avl.level_values(4));
    // avl.draw_list_horizontal(&mut asc, 5, 90, 10, *avl.level_values(2));

    println!("size:{}", avl.size());

    let mut svg = SvgDraw::new(800, 500);

    // avl.render(&mut svg);
    avl.draw(&mut svg);
    // println!("{:?}", avl);

    match &avl.root {
        Some(root) => {
            println!("leftmost pos_x:{}", avl.leftmost_pos(&Rc::clone(&root)));
            println!("rightmost pos_x:{}", avl.rightmost_pos(&Rc::clone(&root)));
            println!("rightmost pos_x:{}", avl.upmost_pos(&Rc::clone(&root)));
            println!("rightmost pos_x:{}", avl.downmost_pos(&Rc::clone(&root)));
        }
        None => (),
    }

    avl.to_svg(String::from("test_tree.svg"), &mut svg);

    // println!("{}", 2_usize.pow(0));

    // svg.line_joint_circle(50.0,50.0,200.0,100.0,10.0);

    // println!("{:?}", avl.level_values(0));
    // avl.draw(&mut svg);

    if let Some(root) = &avl.root {
        let md = avl.min_distance(&root);
        println!("min_distance:{:?}", md);

        println!("==================================================");

        // let ow = avl.min_distance(&root);
        println!(
            "overwided:{}, min_distance:{:?}",
            avl.is_overwided(&root),
            avl.min_distance(&root)
        );
    }
}

use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
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
    pos_x: i32,
    pos_y: i32,
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

    fn draw(&self, svg: &mut SvgDraw) {
        let mut x_interval: i32 = 30;
        let y_interval: i32 = 80;
        let height = self.height();
        let margin = 20;
        let leaf_count = 2_usize.pow((height - 1) as u32);
        svg.row = (height - 1) * y_interval as usize + margin * 2;
        svg.col = leaf_count * x_interval as usize + margin * 2;

        self.init_position(svg.row as i32 / 2, margin as i32, x_interval, y_interval);

        if let Some(root) = &self.root {
            svg.col = margin + self.rightmost_pos(&Rc::clone(&root)) as usize;
            svg.row = margin + self.downmost_pos(&Rc::clone(&root)) as usize;
            self.shift_horizonal(&Rc::clone(&root), -100);

            let mut i = 0;
            while self.is_completed(root) {
                self.keep_away_childs(root, margin as i32);
                i += 1;
            }

            let adjust_x = self.leftmost_pos(root);
            self.shift_horizonal(root, -adjust_x + margin as i32);

            let mut wi = 0;
            // while self.is_overwided(root) {
            //narrow childs
            // self.keep_close_childs(root, margin as i32);
            // wi += 1;
            // }

            // if adjust_x < 0 {
            //     println!("Neeeeeeeeeeeeeeeeeeeeeeeeed to adjust");
            //     self.shift_horizonal(root, -adjust_x + margin as i32);
            // } else {
            //     self.shift_horizonal(root, -adjust_x + margin as i32);
            // }

            println!(
                "Adjust this tree {} times, keep closed {} times, overlapped:{}",
                i,
                wi,
                self.is_completed(&Rc::clone(&root))
            );
        }

        if let Some(root) = &self.root {
            svg.col = margin + self.rightmost_pos(&Rc::clone(&root)) as usize;
            svg.row = margin + self.downmost_pos(&Rc::clone(&root)) as usize;
        }

        // println!("new row and col:{},{}", svg.row, svg.col);
    }

    fn is_overwided(&self, node: &Node<T>) -> bool {
        let avlnode = node.borrow();

        if avlnode.left.is_none() && avlnode.right.is_none() {
            //leaf node cannot be overlapped
            return false;
        }

        // let mut leftchild_rightmost = avlnode.pos_x;
        // let mut rightchild_leftmost = avlnode.pos_x;

        // if let Some(lnode) = &avlnode.left {
        //     leftchild_rightmost = self.rightmost_pos(lnode);
        // }

        // if let Some(rnode) = &avlnode.right {
        //     rightchild_leftmost = self.leftmost_pos(rnode);
        // }

        // let diff = rightchild_leftmost - leftchild_rightmost;

        if let Some(diff) = self.min_distance(node) {
            if diff > 30 {
                println!("overwided key={:?},diff={}", avlnode.key, diff);
                return true;
            }
        }

        let mut is_left_overwided = false;
        let mut is_right_overwided = false;

        if let Some(lnode) = &avlnode.left {
            is_left_overwided = self.is_overwided(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            is_right_overwided = self.is_overwided(rnode);
        }

        return is_left_overwided || is_right_overwided;
    }

    fn is_completed(&self, node: &Node<T>) -> bool {
        let avlnode = node.borrow();

        if avlnode.left.is_none() && avlnode.right.is_none() {
            //leaf node cannot be overlapped
            return false;
        }

        // let mut leftchild_rightmost = avlnode.pos_x;
        // let mut rightchild_leftmost = avlnode.pos_x;

        // if let Some(lnode) = &avlnode.left {
        //     leftchild_rightmost = self.rightmost_pos(lnode);
        // }

        // if let Some(rnode) = &avlnode.right {
        //     rightchild_leftmost = self.leftmost_pos(rnode);
        // }

        // let diff = rightchild_leftmost - leftchild_rightmost;

        if let Some(diff) = self.min_distance(node) {
            if diff <= 20 {
                // println!("key={:?},diff={}", avlnode.key, diff);
                return true;
            }
        }

        let mut is_left_overlaped = false;
        let mut is_right_overlaped = false;

        if let Some(lnode) = &avlnode.left {
            is_left_overlaped = self.is_completed(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            is_right_overlaped = self.is_completed(rnode);
        }

        return is_left_overlaped || is_right_overlaped;
    }

    fn keep_away_childs(&self, node: &Node<T>, margin: i32) {
        let avlnode = node.borrow();

        // let mut leftchild_rightmost = avlnode.pos_x;
        // let mut rightchild_leftmost = avlnode.pos_x;

        // if let Some(lnode) = &avlnode.left {
        //     leftchild_rightmost = self.rightmost_pos(lnode);
        // }

        // if let Some(rnode) = &avlnode.right {
        //     rightchild_leftmost = self.leftmost_pos(rnode);
        // }

        // let diff = rightchild_leftmost - leftchild_rightmost;

        if let Some(diff) = self.min_distance(node) {
            if diff <= 20 {
                if let Some(lnode) = &avlnode.left {
                    self.shift_horizonal(lnode, -(diff.abs() + margin) / 2);
                }

                if let Some(rightnode) = &avlnode.right {
                    self.shift_horizonal(rightnode, (diff.abs() + margin) / 2);
                }

                //adjust all parents node
                // self.keep_away_ancester_childs(node, margin);
            }

            // if diff > 30 {
            //     if let Some(lnode) = &avlnode.left {
            //         self.shift_horizonal(lnode, (diff - 30) / 2);
            //     }

            //     if let Some(rightnode) = &avlnode.right {
            //         self.shift_horizonal(rightnode, -(diff - 30) / 2);
            //     }
            // }
        }

        if let Some(lnode) = &avlnode.left {
            self.keep_away_childs(lnode, margin);
        }

        if let Some(rnode) = &avlnode.right {
            self.keep_away_childs(rnode, margin);
        }
    }

    fn keep_close_childs(&self, node: &Node<T>, margin: i32) {
        let avlnode = node.borrow();

        // let mut leftchild_rightmost = avlnode.pos_x;
        // let mut rightchild_leftmost = avlnode.pos_x;

        // if let Some(lnode) = &avlnode.left {
        //     leftchild_rightmost = self.rightmost_pos(lnode);
        // }

        // if let Some(rnode) = &avlnode.right {
        //     rightchild_leftmost = self.leftmost_pos(rnode);
        // }

        // let diff = rightchild_leftmost - leftchild_rightmost;

        if let Some(diff) = self.min_distance(node) {
            // if diff <= 20 {
            //     if let Some(lnode) = &avlnode.left {
            //         self.shift_horizonal(lnode, -(diff.abs() + margin) / 2);
            //     }

            //     if let Some(rightnode) = &avlnode.right {
            //         self.shift_horizonal(rightnode, (diff.abs() + margin) / 2);
            //     }

            //     //adjust all parents node
            //     // self.keep_away_ancester_childs(node, margin);
            // }

            if diff > 30 {
                println!("just keep cloooooooooooooooooose node:{:?}", avlnode.key);
                if let Some(lnode) = &avlnode.left {
                    self.shift_horizonal(lnode, (diff - 30) / 2);
                }

                if let Some(rightnode) = &avlnode.right {
                    self.shift_horizonal(rightnode, -(diff - 30) / 2);
                }
            }
        }

        if let Some(lnode) = &avlnode.left {
            self.keep_away_childs(lnode, margin);
        }

        if let Some(rnode) = &avlnode.right {
            self.keep_away_childs(rnode, margin);
        }
    }

    fn init_position(&self, x0: i32, y0: i32, x_interval: i32, y_interval: i32) {
        match &self.root {
            Some(root) => {
                root.borrow_mut().pos_x = x0;
                root.borrow_mut().pos_y = y0;
                self._init_pos(root, x0, y0, x_interval, y_interval);
            }
            None => (),
        }
    }

    fn _init_pos(&self, node: &Node<T>, x0: i32, y0: i32, x_interval: i32, y_interval: i32) {
        let avlnode = node.borrow();

        match &avlnode.left {
            Some(l) => {
                l.borrow_mut().pos_x = avlnode.pos_x - x_interval / 2;
                l.borrow_mut().pos_y = avlnode.pos_y + y_interval;
                self._init_pos(l, x0, y0, x_interval, y_interval);
            }
            None => (),
        }

        match &avlnode.right {
            Some(r) => {
                r.borrow_mut().pos_x = avlnode.pos_x + x_interval / 2;
                r.borrow_mut().pos_y = avlnode.pos_y + y_interval;
                self._init_pos(r, x0, y0, x_interval, y_interval);
            }
            None => (),
        }
    }

    fn to_svg(&self, file_name: String, svg: &mut SvgDraw) {
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_front(Rc::clone(&root));
        }

        while !queue.is_empty() {
            if let Some(node) = &queue.pop_back() {
                let avlnode = node.borrow();
                // println!(
                //     "key={:?}, pos_x={:?}, pos_y={:?}",
                //     avlnode.key, avlnode.pos_x, avlnode.pos_y
                // );
                //draw node
                let s = avlnode.key.unwrap().to_string();
                svg.circle(avlnode.pos_x, avlnode.pos_y, 10, String::from("black"), 1);
                svg.text(avlnode.pos_x, avlnode.pos_y, 6, 8, s);

                if let Some(lnode) = &avlnode.left {
                    //draw left line
                    svg.line_joint_circle(
                        avlnode.pos_x as f64,
                        avlnode.pos_y as f64,
                        lnode.borrow().pos_x as f64,
                        lnode.borrow().pos_y as f64,
                        10_f64,
                        String::from("black"),
                        1,
                    );
                    queue.push_front(Rc::clone(&lnode));
                }

                if let Some(rnode) = &avlnode.right {
                    //draw right line
                    svg.line_joint_circle(
                        avlnode.pos_x as f64,
                        avlnode.pos_y as f64,
                        rnode.borrow().pos_x as f64,
                        rnode.borrow().pos_y as f64,
                        10_f64,
                        String::from("black"),
                        1,
                    );
                    queue.push_front(Rc::clone(&rnode));
                }
            }
        }

        svg.render(file_name);
    }

    fn shift_horizonal(&self, node: &Node<T>, distance: i32) {
        let mut avlnode = node.borrow_mut();
        avlnode.pos_x += distance;
        if let Some(lnode) = &avlnode.left {
            self.shift_horizonal(lnode, distance);
        }

        if let Some(rnode) = &avlnode.right {
            self.shift_horizonal(rnode, distance);
        }
    }

    fn leftmost_pos(&self, node: &Node<T>) -> i32 {
        let avlnode = node.borrow();
        let mut left_child_pos_x = avlnode.pos_x;
        let mut right_child_pos_x = avlnode.pos_x;
        if let Some(lnode) = &avlnode.left {
            left_child_pos_x = self.leftmost_pos(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            right_child_pos_x = self.leftmost_pos(rnode);
        }

        min(left_child_pos_x, right_child_pos_x)
    }

    fn rightmost_pos(&self, node: &Node<T>) -> i32 {
        let avlnode = node.borrow();
        let mut left_child_pos_x = avlnode.pos_x;
        let mut right_child_pos_x = avlnode.pos_x;
        if let Some(lnode) = &avlnode.left {
            left_child_pos_x = self.rightmost_pos(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            right_child_pos_x = self.rightmost_pos(rnode);
        }

        max(left_child_pos_x, right_child_pos_x)
    }

    fn downmost_pos(&self, node: &Node<T>) -> i32 {
        let avlnode = node.borrow();
        let mut left_child_pos = avlnode.pos_y;
        let mut right_child_pos = avlnode.pos_y;
        if let Some(lnode) = &avlnode.left {
            left_child_pos = self.downmost_pos(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            right_child_pos = self.downmost_pos(rnode);
        }

        max(left_child_pos, right_child_pos)
    }

    fn upmost_pos(&self, node: &Node<T>) -> i32 {
        //may be simpler
        let avlnode = node.borrow();
        let mut left_child_pos = avlnode.pos_y;
        let mut right_child_pos = avlnode.pos_y;
        if let Some(lnode) = &avlnode.left {
            left_child_pos = self.upmost_pos(lnode);
        }

        if let Some(rnode) = &avlnode.right {
            right_child_pos = self.upmost_pos(rnode);
        }

        min(left_child_pos, right_child_pos)
    }

    fn min_distance(&self, node: &Node<T>) -> Option<i32> {
        let avlnode = node.borrow();

        if avlnode.left.is_none() || avlnode.right.is_none() {
            return None;
        } else {
            let mut left_queue = VecDeque::new();
            let mut right_queue = VecDeque::new();
            let mut min_dist = i32::MAX;

            if let Some(lnode) = &avlnode.left {
                left_queue.push_front(Rc::clone(&lnode));
            };

            if let Some(rnode) = &avlnode.right {
                right_queue.push_front(Rc::clone(&rnode));
            };

            let mut left_key = None;
            let mut right_key = None;
            while !left_queue.is_empty() && !right_queue.is_empty() {
                let mut left_rightmost_pos = i32::MIN;
                let mut right_leftmost_pos = i32::MAX;
                for i in 0..left_queue.len() {
                    if left_queue[i].borrow().pos_x > left_rightmost_pos {
                        left_rightmost_pos = left_queue[i].borrow().pos_x;
                        left_key = left_queue[i].borrow().key;
                    }
                }

                for j in 0..right_queue.len() {
                    if right_queue[j].borrow().pos_x < right_leftmost_pos {
                        right_leftmost_pos = right_queue[j].borrow().pos_x;
                        right_key = right_queue[j].borrow().key;
                    }
                }

                let temp_dist = right_leftmost_pos - left_rightmost_pos;
                if temp_dist < min_dist {
                    min_dist = temp_dist;

                    println!(
                        "leftkey={:?},rightkey={:?},min_dist={}",
                        left_key, right_key, min_dist
                    );
                }

                //put next level childs
                let mut left_queue_count = left_queue.len();
                let mut right_queue_count = right_queue.len();

                // 全部出栈，子节点全部入栈
                while left_queue_count > 0 {
                    if let Some(ln) = &left_queue.pop_back() {
                        if let Some(lnode) = &ln.borrow().left {
                            left_queue.push_front(Rc::clone(&lnode));
                        }

                        if let Some(rnode) = &ln.borrow().right {
                            left_queue.push_front(Rc::clone(&rnode));
                        }
                    }

                    left_queue_count -= 1;
                }

                while right_queue_count > 0 {
                    if let Some(rn) = &right_queue.pop_back() {
                        if let Some(lnode) = &rn.borrow().left {
                            right_queue.push_front(Rc::clone(&lnode));
                        }

                        if let Some(rnode) = &rn.borrow().right {
                            right_queue.push_front(Rc::clone(&rnode));
                        }
                    }

                    right_queue_count -= 1;
                }
            }

            Some(min_dist)
        }
    }

    fn render(&self, svg: &mut SvgDraw) {
        let mut x_interval: i32 = 30;
        let y_interval: i32 = 120;
        let height = self.height();
        let margin = 30;
        let leaf_count = 2_usize.pow((height - 1) as u32);
        svg.row = (height - 1) * y_interval as usize + margin * 2;
        svg.col = leaf_count * x_interval as usize + margin * 2;

        // println!("new row and col:{},{}", svg.row, svg.col);

        let x0: i32 = margin as i32;
        let y0: i32 = svg.row as i32 - margin as i32;

        let mut x_level: i32 = x0;
        let mut y_level: i32 = y0;
        for h in (0..height).rev() {
            // println!("{:?}", self.level_values(h));
            let list = self.level_values(h);

            for i in 0..list.len() {
                if !list[i].is_none() {
                    let s = list[i].unwrap().to_string();
                    let len = s.chars().count();
                    let idx = i as i32;
                    // println!("{},{}", x_level + idx * x_interval, y_level);
                    svg.circle(
                        x_level + idx * x_interval,
                        y_level,
                        10,
                        String::from("black"),
                        1,
                    );
                    svg.text(x_level + idx * x_interval, y_level, 6, 8, s);
                    // svg.text();
                    if h > 0 {
                        if i % 2 == 0 {
                            //left child
                            svg.line_joint_circle(
                                (x_level + idx * x_interval) as f64,
                                y_level as f64,
                                (x_level + idx * x_interval + x_interval / 2) as f64,
                                (y_level - y_interval) as f64,
                                10_f64,
                                String::from("black"),
                                1,
                            );
                        } else {
                            //right child
                            svg.line_joint_circle(
                                (x_level + idx * x_interval) as f64,
                                y_level as f64,
                                (x_level + idx * x_interval - x_interval / 2) as f64,
                                (y_level - y_interval) as f64,
                                10_f64,
                                String::from("black"),
                                1,
                            );
                        }
                    }
                }
            }

            x_level = x_level + x_interval / 2;
            y_level = y_level - y_interval;
            x_interval = 2 * x_interval;
        }

        svg.render(String::from("tree_image.svg"));
    }

    fn draw_list_horizontal(
        &self,
        pen: &mut AsciiDraw,
        x0: i32,
        y0: i32,
        interval: i32,
        list: Vec<Option<T>>,
    ) {
        for i in 0..list.len() {
            if !list[i].is_none() {
                let s = list[i].unwrap().to_string();
                let len = s.chars().count() as i32;
                pen.draw_box(x0 + i as i32 * interval, y0, len, s);
                // println!("x,y={}{}", x0 + i as i32 * interval, y0);
            }
        }
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
            pos_x: 0,
            pos_y: 0,
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
            pos_x: 0,
            pos_y: 0,
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
            pos_x: 0,
            pos_y: 0,
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
            pos_x: 0,
            pos_y: 0,
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
