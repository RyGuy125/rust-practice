use crate::node::NodeRef;
use crate::neuron;
use crate::neuron::NeuronRef;
use std::rc::Rc;

pub type LayerRef = Rc<Layer>;

pub struct Layer {
    neurons: Vec<NeuronRef>
}

impl Layer {
    pub fn new(nin: usize, nout: usize) -> LayerRef {
        let mut n_vec = Vec::new();
        for _ in 0..nout {
            n_vec.push(neuron::Neuron::new(nin));
        }
        Rc::new(Self {
            neurons: n_vec
        })
    }
    pub fn call(&self, x: &[f32]) -> Vec<NodeRef> {
        let mut out_vec = Vec::new();
        for neuron in &self.neurons {
            out_vec.push(neuron.call(x));
        }
        out_vec
    }
    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut p_vec = Vec::new();
        for neuron in self.neurons.clone() {
            for p in neuron.parameters() {
                p_vec.push(p.clone());
            }
        }
        p_vec
    }
}