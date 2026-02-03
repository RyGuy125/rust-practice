
mod node;

fn main() {

    let a = node::Node::new(2.0);
    let b = node::Node::new(3.0);
    let c = node::Node::add(&a, &b);

    println!("{} | {} | {}",a,b,c);
    c.backward();
    println!("{} | {} | {}",a,b,c);


    let d = node::Node::new(4.0);
    let e = node::Node::pow(&d, -1.0);
    println!("{} | {}",d,e);
    e.backward();
    println!("{} | {}",d,e);

    let f = node::Node::new(3.0);
    let g = node::Node::new(1.5);
    let h = node::Node::mul(&f,&g);
    println!("{} | {} | {}",f,g,h);
    h.backward();
    println!("{} | {} | {}",f,g,h);

    let i = node::Node::new(1.);
    let j = node::Node::exp(&i);
    println!("{} | {}",i,j);
    j.backward();
    println!("{} | {}",i,j);

    let k = node::Node::new(0.549306144);
    let l = node::Node::tanh(&k);
    println!("{} | {}",k,l);
    l.backward();
    println!("{} | {}",k,l);

    let m = node::Node::new(6.0);
    let n = node::Node::new(7.0);
    let o = node::Node::sub(&m,&n);
    println!("{} | {} | {}",m,n,o);
    o.backward();
    println!("{} | {} | {}",m,n,o);

    let p = node::Node::new(7.5);
    let q = node::Node::new(1.5);
    let r = node::Node::div(&p,&q);
    println!("{} | {} | {}",p,q,r);
    r.backward();
    println!("{} | {} | {}",p,q,r);

}