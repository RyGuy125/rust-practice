use std::collections::{HashMap, HashSet};
use std::ops;
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::fmt;

/*
class Value:
    def __init__(self, data, _children=(), _op='',label=''):
        self.data = data
        self.grad = 0.0
        self._backward = lambda: None
        self._prev = set(_children)
        self._op = _op
        self.label = label

    def __repr__(self):
        return f"Value(data={self.data})"
    
    # += is used over = because one Value being added/multiplied  twice would overwrite each other 
    #       but they should accumulate instead of overwrite
    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self,other), '+')
        def _backward():
            self.grad += 1.0 * out.grad
            other.grad += 1.0 * out.grad
        out._backward = _backward

        return out
    
    def __radd__(self,other): #other + self
        return self + other
    
    def __pow__(self,other):
        out = Value(self.data**other, (self,), f'**{other}')

        def _backward():
            self.grad += other * self.data**(other-1) * out.grad
        out._backward = _backward

        return out

    def __rmul__(self,other): # other * self
        return self * other
    
    def __truediv__(self,other): # self / other
        return self * other**-1
    
    def __neg__(self):
        return self * -1

    def __sub__(self,other): # self - other
        return self + (-other)

    def __mul__(self,other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self,other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward

        return out
    
    def exp(self):
        x = self.data
        out = Value(math.exp(x), (self,), 'exp')

        def _backward():
            self.grad += out.data * out.grad
        out._backward = _backward

        return out

    def tanh(self):
        n = self.data
        t = (math.exp(2*n) - 1)/(math.exp(2*n) + 1)
        out = Value(t,(self,), 'tanh')

        def _backward():
            self.grad += (1 - t**2) * out.grad
        out._backward = _backward

        return out
    
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

type ValueId = Uuid;

struct Value {
    id: ValueId,
    data: f32,
    grad: f32,
    _backward: Box<dyn Fn(&mut Value, &mut Value, &Value)>,
    _prev: HashSet<ValueId>,
    _op: char,
    _label: String,
}

struct Graph {
    values: HashMap<ValueId, Value>
}

// Default values so I don't need to provide everything
impl Default for Value {
    fn default() -> Value {
        Value {
            id: Uuid::new_v4(),
            data: 0f32,
            grad: 0f32,
            _backward: Box::new(|_a,_b,_c| {}),
            _prev: HashSet::new(),
            _op: '_',
            _label: String::from("")
        }
    }
}

// New Value (on the heap?)
impl Value {
    fn new(data: f32,  _children: HashSet<ValueId>, op: char) -> Value {
        Self {
            data:data,
            _prev:_children, 
            _op:op, 
            ..Default::default()
        }
    }
}

// How to compare Values
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for Value {}

// How to hash Value
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// Specify how to print Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data={} : grad={})",self.data,self.grad)
    }
}

// Addition op overload
impl<'a, 'b> ops::Add<&'b Value> for &'a Value {
    type Output = Value;
    
    fn add(self, _rhs: &'b Value) -> Value {
        
                // let backward = |s : &mut Value, other: &mut Value, out: &mut Value| {
        // let backward = |out : &Value| {
        //         out._children
        //         s.grad += 1.0f32 * out.grad;
        //         other.grad += 1.0f32 * out.grad;
        // };
        let mut out = Value::new(
            self.data + _rhs.data,
            HashSet::from([self.id,_rhs.id]),
            '+'
        );
        let backward = |s: &mut Value, other: &mut Value, out: &Value| {
            s.grad += 1.0f32 * out.grad;
            other.grad += 1.0f32 * out.grad;
        };
        out._backward = Box::new(backward);
        out
    }
}


fn main() {


    let mut a = Value{_label:"a".to_string(), data:3.0f32, ..Default::default()}; 
    let mut b = Value{_label:"b".to_string(), data:4.0f32, ..Default::default()};
    let mut c = &a + &b;
    

    println!("{}",a);
    println!("{}",b);
    println!("{}", c);
    println!("{}", &a+&b);

    c.grad = 1f32;
    (c._backward)(&mut a, &mut b, &c);
    println!("{} | {} | {}", a, b, c);
}