
struct Tree<'a> {
    root: Option<Node<'a>>,
}

struct Node<'a> {
    data: i32,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    parent:Option<&'a Node<'a>>,
}

fn main() {
    println!("Hello, world!");
    
    let mut parent = Node{data: 33_i32, parent:None, left:None, right:None};
    let child = Node{data: 44_i32, parent:Some(&parent), left:None, right:None};
    parent.left = Some(Box::new(child));
}

// Now the lifetime is explicit: we tell the compiler that the lifetime of 
// the parent reference is the same as the Node itself. This struct definition
//  will compile, but writing actual code manipulating it will very quickly
//  get into an altercation with the borrow checker. 
 
//  Consider the code that would insert a new child node into the current node; 
//  to mutate the current node, a mutable reference to it has to be in scope.
//   At the same time, the new child's parent link is a reference to the node.
//    The borrow checker won't let us create a reference to an object which 
//    already has a live mutable reference to it; it also won't let us mutate 
//    an object while any other reference to it is alive.

//父节点需要可变，这样才可以设置子节点，然后子节点也需要指向父节点，所以需要指向父节点的一个
//引用，但是因为父节点已经可变，禁止借用，所以本质上永远不可能用这种方式去实现这样的相互引用的
//数据结构，这个在其它语言很正常。