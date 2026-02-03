use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::fmt;

type NodeRef = Rc<Node>;

pub struct Node {
    data: f32,
    grad: Cell<f32>,
    parents: Vec<Weak<Node>>,
    _backward: RefCell<Option<Box<dyn Fn()>>>, 
}

// Default values so I don't need to provide everything
impl Default for Node {
    fn default() -> Node {
        Node {
            data: 0f32,
            grad: Cell::new(0f32),
            parents: vec![],
            _backward: RefCell::new(None),
        }
    }
}

// Specify how to print Node
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data={} : grad={})",self.data,self.grad.get())
    }
}

impl Node {
    pub fn new(data: f32) -> NodeRef {
        Rc::new(Self {
            data: data,
            ..Default::default()
        })
    }

    pub fn add(a: &NodeRef, b: &NodeRef) -> NodeRef {
        let out = Rc::new(Node {
            data: a.data + b.data,
            grad: Cell::new(0.),
            parents: vec![
                Rc::downgrade(a),
                Rc::downgrade(b)
            ],
            _backward: RefCell::new(None)
        });

        let a_weak = Rc::downgrade(a);
        let b_weak = Rc::downgrade(b);
        let out_weak = Rc::downgrade(&out);

        let backward = move || {
            let out = out_weak.upgrade().unwrap();
            let grad = out.grad.get();

            if let Some(a) = a_weak.upgrade() {
                a.grad.set(a.grad.get() + grad);
            }
            if let Some(b) = b_weak.upgrade() {
                b.grad.set(b.grad.get() + grad);
            }
        };

        *out._backward.borrow_mut() = Some(Box::new(backward));
        out
    }

    pub fn pow(a: &NodeRef, exp: f32) -> NodeRef {
        let out = Rc::new(Node {
            data: a.data.powf(exp),
            grad: Cell::new(0.),
            parents: vec![
                Rc::downgrade(a)
            ],
            _backward: RefCell::new(None)
        });

        let a_weak = Rc::downgrade(a);
        let out_weak = Rc::downgrade(&out);

        let backward = move || {
            let out = out_weak.upgrade().unwrap();
            let grad = out.grad.get();

            if let Some(a) = a_weak.upgrade() {
                a.grad.set(a.grad.get() + exp * a.data.powf(exp-1.) * grad);
            }
        };

        *out._backward.borrow_mut() = Some(Box::new(backward));
        out
    }

    pub fn mul(a: &NodeRef, b: &NodeRef) -> NodeRef {
        let out = Rc::new(Node {
            data: a.data * b.data,
            grad: Cell::new(0.),
            parents: vec![
                Rc::downgrade(a),
                Rc::downgrade(b)
            ],
            _backward: RefCell::new(None)
        });

        let a_weak = Rc::downgrade(a);
        let b_weak = Rc::downgrade(b);
        let out_weak = Rc::downgrade(&out);

        let backward = move || {
            let out = out_weak.upgrade().unwrap();
            let grad = out.grad.get();


            if let (Some(a), Some(b)) = (a_weak.upgrade(), b_weak.upgrade()) {
                a.grad.set(a.grad.get() + b.data * grad);
                b.grad.set(b.grad.get() + a.data * grad);
            }
        };
        *out._backward.borrow_mut() = Some(Box::new(backward));
        out
    }

    pub fn exp(a: &NodeRef) -> NodeRef {
        let out = Rc::new(Node {
            data: a.data.exp(),
            grad: Cell::new(0.),
            parents: vec![
                Rc::downgrade(a)
            ],
            _backward: RefCell::new(None)
        });

        let a_weak = Rc::downgrade(a);
        let out_weak = Rc::downgrade(&out);

        let backward = move || {
            let out = out_weak.upgrade().unwrap();
            let grad = out.grad.get();

            if let Some(a) = a_weak.upgrade() {
                a.grad.set(a.grad.get() + out.data * grad);
            }
        };

        *out._backward.borrow_mut() = Some(Box::new(backward));
        out
    }

    pub fn tanh(a: &NodeRef) -> NodeRef {
        let n = a.data * 2.;
        let t = (n.exp() - 1.) / (n.exp() + 1.);
        let out = Rc::new(Node {
            data: t,
            grad: Cell::new(0.),
            parents: vec![
                Rc::downgrade(a)
            ],
            _backward: RefCell::new(None)
        });

        let a_weak = Rc::downgrade(a);
        let out_weak = Rc::downgrade(&out);

        let backward = move || {
            let out = out_weak.upgrade().unwrap();
            let grad = out.grad.get();
            
            if let Some(a) = a_weak.upgrade() {
                a.grad.set(a.grad.get() + (1. - t.powf(2.)) * grad);
            }
        };

        *out._backward.borrow_mut() = Some(Box::new(backward));
        out
    }

    pub fn neg(a: &NodeRef) -> NodeRef {
        Node::mul(a, &Node::new(-1.))
    }

    pub fn sub(a: &NodeRef, b: &NodeRef) -> NodeRef {
        Node::add(a, &Node::neg(b))
    }

    pub fn div(a: &NodeRef, b: &NodeRef) -> NodeRef {
        Node::mul(a, &Node::pow(b,-1.))
    }

    pub fn backward(self: &NodeRef) {
        let mut visited : HashSet<usize> = HashSet::new();
        let mut topo : Vec<NodeRef> = Vec::new();
        fn build_topo(v: &NodeRef, visited: &mut HashSet<usize>, topo: &mut Vec<NodeRef>) {
            let ptr = Rc::as_ptr(v) as usize;
            if visited.contains(&ptr) { return; }
            visited.insert(ptr);

            for p in &v.parents {
                if let Some(parent) = p.upgrade() {
                    build_topo(&parent, visited, topo);
                }
            }
            
            topo.push(v.clone());
        }
        build_topo(self, &mut visited, &mut topo);

        self.grad.set(1.0);
        for node in topo.into_iter().rev() {
            if let Some(ref backward) = *node._backward.borrow() {
                backward();
            }
        }
    }
}

