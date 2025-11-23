use std::fmt;

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Xs(T,Box<Self>)
    // Cons(T,Box<Self>)
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            List::Nil => write!(f, "()"),
            List::Xs(ref x, ref xs) => write!(f, "({} {})", x, xs),  
        }
    }
}

fn reverse<T>(l: List<T>) -> List<T> {
    match l {
        List::Nil => List::Nil,
        List::Xs(x,xs) => append(reverse(*xs),List::Xs(x,Box::new(List::Nil)))
    }
}


fn append<T>(l1: List<T>, l2: List<T>) -> List<T> {
    match l1 {
        List::Nil => l2,
        List::Xs(x,xs) => List::Xs(x,Box::new(append(*xs,l2)))
    }
}


fn tail<T>(l: &List<T>) -> Option<&T> {
    match l {
        List::Nil => None,
        List::Xs(x,xs) if matches!(**xs, List::Nil) => Some(x),
        List::Xs(_,xs) => tail(&*xs),
    }
}


fn map<T, U, F: Fn(&T) -> U>(l: &List<T>, f: F) -> List<U> {
    match l {
        List::Nil => List::Nil,
        List::Xs(x, xs) => List::Xs(f(x), Box::new(map(xs, f)))
    }
}


fn foldl<T, U, F: Fn(&T, &U) -> U>(l: &List<T>, init: U, acc: F) -> U {
    match l {
        List::Nil => init,
        List::Xs(x,xs) => foldl(xs, acc(x,&init), acc)
    }
}

fn foldr<T, U, F: Fn(&T,&U) -> U>(l: &List<T>, init: U, acc: &F) -> U {
    match l {
        List::Nil => init,
        List::Xs(x,xs) => acc(x,&foldr(xs,init,&acc))
    }
}

fn sum(adder: &u8, addend: &u8) -> u8 {
    return adder + addend;
}

fn main() {
    let mut l = List::Xs(1u8,
                Box::new(List::Xs(2u8,
                Box::new(List::Xs(3u8,
                Box::new(List::Xs(4u8,
                Box::new(List::Nil))))))));

    let k = List::Xs(5u8,
                Box::new(List::Xs(6u8,
                Box::new(List::Xs(7u8,
                Box::new(List::Xs(8u8,
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

    println!("{}",l);

}

