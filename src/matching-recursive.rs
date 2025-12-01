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


    println!("{}",l);
    println!("{}",k);
    
    l = append(l,k);
    println!("{}",l);
    l = reverse(l);
    println!("{}",l);
    let end = tail(&l).unwrap();
    println!("{}",end);

    let total = foldl(&l,0,sum);
    println!("{}",total);
    l = map(&l, |x: &u8| -> u8 {x + 1});
    println!("{}",foldr(&l,0,&sum));
    
    println!("{}",l);

    println!("car: {}",car(&l).unwrap());
    println!("cdr: {}",cdr(&l).unwrap());
    println!("length: {}",length(&l));
    println!("{}",l);
    println!("finding 10: {}",find(&l,10u8));

    let z : List<u8> = make_list(20usize, 67u8);
    println!("{}",z);
}

