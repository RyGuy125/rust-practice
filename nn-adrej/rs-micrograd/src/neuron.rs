use crate::node;
use crate::node::NodeRef;
use std::iter::repeat_with;
use rand;
use std::rc::Rc;
use std::fmt;

pub type NeuronRef = Rc<Neuron>;

pub struct Neuron {
    weight: Vec<NodeRef>,
    bias: NodeRef
}

impl Neuron {
    pub fn new(nin: usize) -> NeuronRef {
        Rc::new(Self {
            weight: repeat_with(|| node::Node::new(rand::random_range(-1.0..=1.0))).take(nin).collect(),
            bias: node::Node::new(rand::random_range(-1.0..=1.0))
        })
    }
    pub fn call(&self, x: &[f32]) -> NodeRef {
        let act = &self.bias;
        let mut sum = node::Node::new(0.0);

        let it = self.weight.iter().zip(x.iter());
        for (_, (wi,xi)) in it.enumerate() {
            sum = node::Node::add(&sum,&node::Node::mul(&wi,&node::Node::new(*xi)));
        }
        let out = node::Node::tanh(&node::Node::add(&act,&sum));
        out
    }
    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut p = self.weight.clone();
        p.append(&mut vec![self.bias.clone()]);
        p
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(weights: {}\nbias: {})", self.weight.clone().into_iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" | "), self.bias)
    }
}