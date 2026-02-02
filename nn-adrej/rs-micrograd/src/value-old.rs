use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::fmt;

pub struct Value {
    data: f32,
    grad: f32,
    _backward: Box<dyn Fn(&mut Value, &mut Value, &Value)>,
    _prev: HashSet<Weak<Value>>,
}

// Default values so I don't need to provide everything
impl Default for Value {
    fn default() -> Value {
        Value {
            data: 0f32,
            grad: 0f32,
            _backward: Box::new(|_a,_b,_c| {}),
            _prev: HashSet::new(),
        }
    }
}

// New Value (on the heap?)
impl Value {
    pub fn new(data: f32) -> Rc<Value> {
        Rc::new(Self {
            data: data,
            ..Default::default()
        })
    }
}

fn from_op(data: f32, inputs: &[Rc<Value>]/*, op: char*/) -> Rc<Value> {
    let mut prev = HashSet::new();
    for v in inputs {
        prev.insert(Rc::downgrade(v)); // dedups Value
    }

    Rc::new(Value {
        data,
        _prev: prev,
        // _op: op,
        ..Default::default()
    })
}

trait AsValueRc {
    fn as_rc(&self) -> Option<Rc<Value>>;
}

impl AsValueRc for Rc<Value> {
    fn as_rc(&self) -> Option<Rc<Value>> {
        Some(self.clone())
    }
}

impl AsValueRc for Weak<Value> {
    fn as_rc(&self) -> Option<Rc<Value>> {
        self.upgrade()
    }
}

trait ValueOps {
    fn add<V: AsValueRc>(&self, rhs: &V) -> Rc<Value>;
}

impl ValueOps for Rc<Value> {
    fn add<V: AsValueRc>(&self, rhs: &V) -> Rc<Value> {
        let a = self.clone();
        let b = rhs.as_rc().expect("Dangling Weak<Value>");

        from_op(a.data + b.data, &[a, b])
    } 
}

// Specify how to print Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data={} : grad={})",self.data,self.grad)
    }
}

fn backward(self) {
    let mut topo : Vec<Rc<Value>> = Vec::new();
    let mut visited : HashSet<ValueRef> = HashSet::new();
    fn build_topo(v: ValueRef, visited: &mut HashSet<ValueRef>, topo: &mut Vec<Rc<Value>>) {
        if !visited.contains(&v) {
            visited.insert(v);
            for child in &v.weak.upgrade().unwrap()._prev {
                build_topo(child,visited,topo);
            }
            topo.push(v.weak.upgrade().unwrap());
        }
    } 
    build_topo(
        ValueRef { weak : Rc::downgrade(&self) },
        &visited,
        &topo
    );

    self.grad.set(1.0);
    for node in topo.iter_mut().rev() {
        (node._backward)(node);
    }
}