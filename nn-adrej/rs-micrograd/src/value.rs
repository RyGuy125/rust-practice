use std::collections::HashSet;
use std::hash::{Hash, Hasher};
// use std::borrow::Borrow;
use std::cell::Cell;
use std::rc::{Rc, Weak};
use std::fmt;

/* 
    Problem Statement
        I want to implement backpropagation in Rust, inspired from micro-
        grad in Python.

        I will have a Struct Value with the following members:
        - data
        - grad (gradient)
        - _backward (a closure)
        - _prev (children Value instances)

        A Value must be shared and mutable
        - Multiple parents can point to the same child
        - grad can be mutated long after forward pass creates the Value
        
        I cannot borrow Values because of lifetimes
        - recursive lifetimes (lifetime of "a" depends on "b" depends on...)
        - Need pointer like wrappers over references

        Backpropagation requires:
        - Reading data (thus data never changes)
        - Accumulating grad (grad always changes)
        - Reading _prev (which never changes)

        _backwards closures capture ownership
        - It should not borrow a Value, and should own whatever it needs
        
        Building a DAG isn't clear to Rust
        - Children point to parents and parents are shared so compiler 
            may think cycle
        - _prev does not need Strong Ownership
        - _prev needs non-owning handles

    Core Concepts:
        data is immutable
        grad is mutable
        _backward needs mutable Values through closures that capture
            ownership (?)
        _prev should use weak pointers
        Value will be implemented with Rc<Value>
*/

struct ValueRef {
    weak: Weak<Value>
}

impl PartialEq for ValueRef {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.weak) == Weak::as_ptr(&other.weak)
    }
}
impl Eq for ValueRef {}

impl Hash for ValueRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Weak::as_ptr(&self.weak).hash(state);
    }
}

pub struct Value {
    data: f32,
    grad: Cell<f32>,
    _backward: fn(&Rc<Self>),
    _prev: HashSet<ValueRef>,
    _differentiable: Cell<bool>,
}

fn from_op(data: f32, inputs: &[&Rc<Value>], backward: fn(&Rc<Value>)) -> Rc<Value> {
    let mut prev = HashSet::new();
    for v in inputs {
        prev.insert(ValueRef { weak: Rc::downgrade(v) });
    }

    Rc::new(Value {
        data,
        _prev: prev,
        _backward: backward,
        ..Default::default()
    })
}


// Specify how to print Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data={} : grad={})",self.data,self.grad.get())
    }
}

// Default values so I don't need to provide everything
impl Default for Value {
    fn default() -> Value {
        Value {
            data: 0f32,
            grad: Cell::new(0f32),
            _backward: |_a| {},
            _prev: HashSet::new(),
            _differentiable : Cell::new(false),
        }
    }
}

pub trait ValueOps {
    fn add(&self, rhs: &Rc<Value>) -> Rc<Value>;
    fn pow(&self, rhs: f32) -> Rc<Value>;
    fn backward(&self);
}

impl ValueOps for Rc<Value> {

    /*
    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')

        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
    */

    fn add(&self, rhs: &Rc<Value>) -> Rc<Value>{
        // let a = self.clone();
        // let b = other.clone();
        let backward = |s: &Rc<Value>| {
            for v in &s._prev {
                let grad = v.weak.upgrade().unwrap().grad.get();
                v.weak.upgrade().unwrap().grad.set(grad + s.grad.get());
            }
        };

        from_op(&self.data + &rhs.data, &[&self, &rhs], backward)
    }

    /*
    def __pow__(self,other):
        out = Value(self.data**other, (self,), f'**{other}')

        def _backward():
            self.grad += other * self.data**(other-1) * out.grad
            self.grad = self.grad + other * self.data**(other-1) * out.grad
        out._backward = _backward

        return out
    */
    fn pow(&self, rhs: f32) -> Rc<Value> {
        let backward = move |other: f32| {
            let back = |s: &Rc<Value>| {
                for v in &s._prev {
                    let child = v.weak.upgrade().unwrap();
                    child.grad.set(child.grad.get() + other * child.data.powf(other-1.0) * s.grad.get());
                }
            };
            back
        };
        from_op(self.data.powf(rhs), &[self], backward(rhs))
    }

    fn backward(&self) {
        let mut topo = Vec::new();
        let mut visited = HashSet::new();
        fn build_topo(v: &ValueRef, visited: &mut HashSet<usize>, topo: &mut Vec<Rc<Value>>) {
            let v_rc = v.weak.upgrade().unwrap();
            let ptr = Rc::as_ptr(&v_rc) as usize;
            if visited.contains(&ptr) {
                return;
            }
            visited.insert(ptr);

            // let children = v_rc._prev;
            for child in v_rc._prev.iter() {
                build_topo(&child, visited, topo);
            }

            topo.push(v_rc);
        }

        build_topo(&ValueRef { weak: Rc::downgrade(self) }, &mut visited, &mut topo);

        self.grad.set(1.0);
        for node in topo.into_iter().rev() {
            (node._backward)(&node);
        }
    }
}

impl Value {
    pub fn new(data: f32) -> Rc<Value> {
        Rc::new(Self {
            data: data,
            ..Default::default()
        })
    }


    /*
    def backward(self):
        topo = [] 
        visited = set()
        def build_topo(v):
            if v not in visited:
                visited.add(v)
                for child in v._prev:
                    build_topo(child)
                topo.append(v)
        build_topo(self)

        self.grad = 1.0
        for node in reversed(topo):
            node._backward()
    */
}

