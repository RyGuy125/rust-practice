use std::collections::HashSet;
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

pub struct Value {
    id: Uuid, // UUIDv4
    data: f32,
    grad: f32,
    //_backward: Box<dyn FnMut(&Value,&Value,&Value)>,
    _prev: HashSet<Value>,
    _op: char,
    _label: String,
}

impl Default for Value {
    fn default() -> Value {
        Value {
            id: Uuid::new_v4(),
            data: 0f32,
            grad: 0f32,
            //_backward: Box::new(|_a,_b,_c| {}),
            _prev: HashSet::new(),
            _op: '_',
            _label: String::from("")
        }
    }
}

// impl Value {
//     fn new(id: Uuid, data: f32, grad: f32, _backward: fn(Value,Value,Value), _prev: HashSet<Value>, _op: char, label: String) -> Self {
//         Self { id, data, grad, _backward, _prev, _op, label}
//     }
// }

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(data={} : grad={})",self.data,self.grad)
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;
    

    fn add(self, mut _rhs: Value) -> Value {
        
        //let backward = |mut s : &Value, mut other: &Value, out: &Value| {
        //        s.grad += 1.0f32 * out.grad;
        //        other.grad += 1.0f32 * out.grad;
        //};

        let out = Value {
            id: Uuid::new_v4(),
            data: self.data + _rhs.data,
            _prev: HashSet::from([self,_rhs]),
            _op: '+',
            grad: 0.0f32,
            //_backward: backward,
            _label: String::from("")
        };
        out
    }
}


fn main() {
    let a = Value{_label:"a".to_string(), data:3.0f32, ..Default::default()};
    let b = Value{_label:"b".to_string(), data:4.0f32, ..Default::default()};
    let c = a + b;
    // c._backward()

    println!("{}",a);
    println!("{}",b);
    println!("{}", a+b);
}