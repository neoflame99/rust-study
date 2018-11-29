//-- example1
/*
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    let d = match &b {
        List::Cons(V,_) => Some(V),
        List::Nil => None,
    };

    if d != None {
        let dd = d.unwrap();
        *dd.borrow_mut() += 100;
    }

    println!("a after = {:?}",a);
    println!("b after = {:?}",b);
    println!("c after = {:?}",c);
}

*/

//-- example2
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value : i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main (){
    let leaf = Rc::new(Node {
        value: 3,
        parent : RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong={}, weak={}",Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("branch strong={}, weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("branch strong={}, weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong={}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong={}, weak={}",Rc::strong_count(&leaf), Rc::weak_count(&leaf));

}
