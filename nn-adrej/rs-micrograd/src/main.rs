mod node;
use crate::node::Node;
// mod neuron;
// mod layer;
// mod mlp;

#[test]
fn test_node() {
    let x1 = Node::new(2.0);
    let x2 = Node::new(0.0);
    let w1 = Node::new(-3.0);
    let w2 = Node::new(1.0);
    

    let b = Node::new(6.8813735870195432);

    // x1*w1 + x2*w2 + b
    let x1w1 = Node::mul(&x1, &w1);
    let x2w2 = Node::mul(&x2, &w2);
    let x1w1x2w2 = Node::add(&x1w1, &x2w2);
    let n = Node::add(&x1w1x2w2, &b);
    let o = Node::tanh(&n);

    println!("1: {} | {}\n2: {} | {}\nb: {}\nx1*w1 + x2*w2 + b: {}\nfinal: {}",x1,w1,x2,w2,b,n,o);
    o.backward();
    println!("\n\n1: {} | {}\n2: {} | {}\nb: {}\nx1*w1 + x2*w2 + b: {}\nfinal: {}",x1,w1,x2,w2,b,n,o);

    assert_eq!(w1.grad.get().round(),1.0);
}

#[test]
fn test_add() {
    let a = node::Node::new(2.0);
    let b = node::Node::new(3.0);
    let c = node::Node::add(&a, &b);

    println!("{} | {} | {}",a,b,c);
    c.backward();
    println!("{} | {} | {}",a,b,c);

    assert_eq!(a.grad.get().round(),1.0);
    assert_eq!(b.grad.get().round(),1.0);
}

#[test]
fn test_pow() {
    let d = node::Node::new(4.0);
    let e = node::Node::pow(&d, 2.0);
    println!("{} | {}",d,e);
    e.backward();
    println!("{} | {}",d,e);

    assert_eq!(d.grad.get().round(),8.0);
}

#[test]
fn test_mul() {
    let f = node::Node::new(3.0);
    let g = node::Node::new(1.5);
    let h = node::Node::mul(&f,&g);
    println!("{} | {} | {}",f,g,h);
    h.backward();
    println!("{} | {} | {}",f,g,h);

    assert_eq!(f.grad.get(),1.5);
    assert_eq!(g.grad.get(),3.0);

}

#[test]
fn test_exp() {
    let i = node::Node::new(1.0);
    let j = node::Node::exp(&i);
    println!("{} | {}",i,j);
    j.backward();
    println!("{} | {}",i,j);

    assert_eq!(i.grad.get(),2.7182817);
}

#[test]
fn test_tanh() {
    let k = node::Node::new(0.549306144);
    let l = node::Node::tanh(&k);
    println!("{} | {}",k,l);
    l.backward();
    println!("{} | {}",k,l);

    assert_eq!(k.grad.get(),0.75);
}

#[test]
fn test_sub() {
    let m = node::Node::new(6.0);
    let n = node::Node::new(7.0);
    let o = node::Node::sub(&m,&n);
    println!("{} | {} | {}",m,n,o);
    o.backward();
    println!("{} | {} | {}",m,n,o);

    assert_eq!(m.grad.get(),1.0);
    assert_eq!(n.grad.get(),0.0);
}

#[test]
fn test_div() {
    let p = node::Node::new(10.0);
    let q = node::Node::new(5.0);
    let r = node::Node::div(&p,&q);
    println!("{} | {} | {}",p,q,r);
    r.backward();
    println!("{} | {} | {}",p,q,r);

    assert_eq!(p.grad.get(),0.2);
    assert_eq!(q.grad.get(),-0.4);
}

fn main() {

    
    /*

    // let x = neuron::Neuron::new(3);
    // println!("{}", x);
    // let act = x.call(&[2.0,3.0,1.0]);
    // println!("{}", act);

    // let z = layer::Layer::new(2,3);
    // let ns = z.call(5.0);

    // let x = [2.0,3.0,-1.0];
    // let mlp = mlp::MLP::new(3,&[4,4,1]);
    // let res = mlp.call(&x);
    // for i in res {
    //     println!("{}",i);
    // }

    // let xs= vec![
    //     vec![2.0, 3.0, -1.0],
    //     vec![3.0, -1.0, 0.5],
    //     vec![0.5, 1.0, 1.0],
    //     vec![1.0, 1.0, -1.0],
    // ];
    // let ys = vec![
    //     node::Node::new(1.0), 
    //     node::Node::new(-1.0), 
    //     node::Node::new(-1.0), 
    //     node::Node::new(1.0)
    // ];
    // let mut loss = node::Node::new(0.0); 
    // mlp.learn(xs,ys,20,&mut loss);
    */
}