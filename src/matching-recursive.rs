use std::fmt;

// #[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T,Box<Self>)
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List::Nil => write!(f, "()"),
            List::Cons(ref x, ref xs) => write!(f, "({} {})", x, xs),  
        }
    }
}

fn car<T>(l: &List<T>) -> Option<&T> {
    match l {
        List::Nil => None,
        List::Cons(x,_) => Some(x)
    }
}

fn cdr<T>(l: &List<T>) -> Option<&List<T>> {
    match l {
        List::Nil => None,
        List::Cons(_,xs) => Some(&*xs)
    }
}

fn length<T>(l: &List<T>) -> usize {
    match l {
        List::Nil => 0,
        List::Cons(_,xs) => 1usize + length(xs)
    }
}


fn make_list<T: Copy>(size: usize, elem: T) -> List<T> {
    match size {
        0 => List::Nil,
        _ => List::Cons(elem, Box::new(make_list(size-1,elem)))
    }
}

fn reverse<T>(l: List<T>) -> List<T> {
    match l {
        List::Nil => List::Nil,
        List::Cons(x,xs) => append(reverse(*xs),List::Cons(x,Box::new(List::Nil)))
    }
}


fn append<T>(l1: List<T>, l2: List<T>) -> List<T> {
    match l1 {
        List::Nil => l2,
        List::Cons(x,xs) => List::Cons(x,Box::new(append(*xs,l2)))
    }
}


fn tail<T>(l: &List<T>) -> Option<&T> {
    match l {
        List::Nil => None,
        List::Cons(x,xs) if matches!(**xs, List::Nil) => Some(x),
        List::Cons(_,xs) => tail(&*xs),
    }
}


fn map<T, U, F: Fn(&T) -> U>(l: &List<T>, f: F) -> List<U> {
    match l {
        List::Nil => List::Nil,
        List::Cons(x, xs) => List::Cons(f(x), Box::new(map(xs, f)))
    }
}


fn foldl<T, U, F: Fn(&T, &U) -> U>(l: &List<T>, init: U, acc: F) -> U {
    match l {
        List::Nil => init,
        List::Cons(x,xs) => foldl(xs, acc(x,&init), acc)
    }
}

fn foldr<T, U, F: Fn(&T,&U) -> U>(l: &List<T>, init: U, acc: &F) -> U {
    match l {
        List::Nil => init,
        List::Cons(x,xs) => acc(x,&foldr(xs,init,acc))
    }
}

fn find<T, F: Fn(&T,&T)-> bool>(l : &List<T>, elem: &T, comp: &F) -> usize {
    match l {
        List::Nil => usize::MAX,
        List::Cons(x,_) if comp(x,elem) => 0usize,
        List::Cons(_,xs) => 1usize + find(xs,elem,comp)
    }
}

fn index<T>(l: &List<T>, idx: usize) -> Option<&T> {
    match idx {
        0 => car(&l),
        _ => match l {
            List::Nil => None,
            List::Cons(_,xs) => index(xs,idx-1)
        }
    }
    
}

fn equals(x: &u8, y: &u8) -> bool {
    return x == y;
}


fn sum(adder: &u8, addend: &u8) -> u8 {
    return adder + addend;
}

fn main() {
    let mut l = List::Cons(1u8,
                Box::new(List::Cons(2u8,
                Box::new(List::Cons(3u8,
                Box::new(List::Cons(4u8,
                Box::new(List::Nil))))))));

    let k = List::Cons(5u8,
                Box::new(List::Cons(6u8,
                Box::new(List::Cons(7u8,
                Box::new(List::Cons(8u8,
                Box::new(List::Nil))))))));


    println!("l: {}",l);
    println!("k: {}",k);
    
    l = append(l,k);
    println!("l appended with k: {}",l);
    l = reverse(l);
    println!("l reversed: {}",l);
    let end = tail(&l).unwrap();
    println!("last element of l: {}",end);

    let total = foldl(&l,0,sum);
    println!("sum of l: {}",total);
    l = map(&l, |x: &u8| -> u8 {x + 1});
    println!("sum of l after +1 mapping: {}",foldr(&l,0,&sum));
    
    println!("l: {}",l);

    println!("car: {}",car(&l).unwrap());
    println!("cdr: {}",cdr(&l).unwrap());
    println!("length: {}",length(&l));
    println!("l: {}",l);

    let z : List<u8> = make_list(20usize, 67u8);
    println!("Made list:\n{}",z);

    println!("Finding 2u8: {}",find(&l,&2u8,&equals));

    println!("Retrieving 8 from its index: {}",index(&l,find(&l,&8u8,&equals)).unwrap());

    let long_list: List<u32> = make_list(10000usize,u32::MAX);
    // let long_list: List<u32> = make_list(100000usize,u32::MAX); // crashes -> stack overflow
    println!("long list:\n{}",long_list);
}

