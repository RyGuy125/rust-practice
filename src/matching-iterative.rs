use std::fmt;

pub enum List<T> {
    Nil,
    Cons(T,Box<Self>)
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut l : &List<T> = self;
        while let List::Cons(x,xs) = l {
            let _ = write!(f,"{} ", x);
            l = &*xs;
        }
        return write!(f,"()");
    }
}

fn make_list<T: Copy>(mut size: usize, elem: T) -> List<T> {
    let mut l : List<T> = List::Nil;
    while size != 0usize {
        l = List::Cons(elem,Box::new(l));
        size-=1;
    }
    l
}

fn length<T>(l: &List<T>) -> usize {
    let mut length: usize = 0usize;
    let mut list : &List<T> = l;
    while let List::Cons(_,xs) = list {
        length+=1;
        list = &*xs;
    }
    length
}

fn car<T>(l: &List<T>) -> Option<&T> {
    if let List::Cons(x,_) = l {
        return Some(x);
    }
    None
}

fn cdr<T>(l: &List<T>) -> Option<&List<T>> {
    if let List::Cons(_,xs) = l {
        return Some(xs);
    }
    None
}

fn append<T>(l1: &mut List<T>, l2: List<T>) -> &mut List<T> {
    let list : &mut List<T> = l1;
    while let &mut List::Cons(ref x,ref xs) = list {
        if matches!(**xs, List::Nil) { 
            *list = List::Cons(*x,value);
        }
        else { 
            *list = **xs;
        }
    }
    l1
}

// fn reverse<T>(l: List<T>) -> List<T> {

// }

fn main() {
    let l = make_list(1_000usize,u8::MAX);
    // println!("list l: \n{}",l);
    println!("length of l: {}",length(&l));
    println!("first element of l: {}",car(&l).unwrap());
    println!("{}",append(&mut make_list(10,u8::MAX),make_list(10,u8::MIN)));
    println!("{}",l);
}