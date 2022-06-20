use crate::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(n: Vec<Neuron>) -> Layer {
        Layer { neurons: n }
    }

    pub fn sigmoid_outputs(&self, i: &Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|n| n.sigmoid_output(i)).collect()
    }

    pub fn relu_outputs(&self, i: &Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|n| n.relu_output(i)).collect()
    }
}
