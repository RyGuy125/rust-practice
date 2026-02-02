use crate::value::ValueOps;
mod value;
/*
class Value:
    
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
*/

fn main() {

    let a = value::Value::new(2.0);
    let b = value::Value::new(3.0);
    let c = a.add(&b);
    

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", a.add(&b));

    // c.grad = 1f32;
    c.backward();
    println!("{} | {} | {}", a, b, c);
}