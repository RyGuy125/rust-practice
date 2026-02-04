use crate::node;
use crate::node::NodeRef;
use crate::layer;
use crate::layer::LayerRef;
use std::rc::Rc;

type MLPRef = Rc<MLP>;

pub struct MLP {
    layers: Vec<LayerRef>
}

impl MLP {
    pub fn new(nin: usize, nouts: &[usize]) -> MLPRef {
        let mut sz = vec![nin];
        sz.extend(nouts);

        let mut l_vec = Vec::new();
        for i in 0..nouts.len() {
            l_vec.push(layer::Layer::new(sz[i], sz[i+1]));
        }
        Rc::new(Self {
            layers: l_vec
        })
    }
    pub fn call(&self, x : &[f32]) -> Vec<NodeRef> {
        let mut y = Vec::new();
        for layer in &self.layers {
            y = layer.call(x);
        }
        y
    }
    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut p_vec = Vec::new();
        for layer in self.layers.clone() {
            for p in layer.parameters() {
                p_vec.push(p.clone());
            }
        }
        p_vec
    }
    pub fn learn(self: &MLPRef, xs: Vec<Vec<f32>>, ys: Vec<NodeRef>, intervals: u32, loss: &mut NodeRef) {
        for _ in 0..intervals {
            let ypred = xs.iter().fold(Vec::new(), |mut call, row| {
                    call.append(&mut self.call(row));
                    call
                });
            let it = ys.iter().zip(ypred.iter());
            for (_, (ygt,yout)) in it.enumerate() {
                let pow = node::Node::pow(&node::Node::sub(&yout, &ygt),2.0);
                *loss = node::Node::add(&loss, &pow);
            }
            println!("loss: {}",loss);
            loss.backward();
            for p in self.parameters() {
                p.adjust_weight(-0.01);
            }
            for pred in ypred {
                print!("{} ",pred);
            }
        }
    }
}