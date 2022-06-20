use core::f32::consts::E;

pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
    inputs: Vec<usize>
}

impl Neuron {
    pub fn new(i: Vec<usize>) -> Neuron {
        Neuron {
            weights: (0..i.len()).map(|x| rand::random::<f32>()).collect(),
            bias: rand::random::<f32>(),
            inputs: i
        }
    }

    pub fn sigmoid_output(&self, i: &Vec<f32>) -> f32 {
        1f32 / (1f32 + E.powf(-self.iw(i)))
    }

    pub fn relu_output(&self, i: &Vec<f32>) -> f32 {
        let out = self.iw(i);
        if out > 0f32 {
            out
        } else {
            0f32
        }
    }

    fn iw(&self, i: &Vec<f32>) -> f32 {
        i.iter()
        .enumerate()
        .filter_map(|e| {
            if self.inputs.contains(&e.0) {
                Some(*e.1)
            } else {
                None
            }
        })
        .enumerate()
        .map(|e| e.1 * self.weights[e.0])
        .sum::<f32>()
        + self.bias
    }
}
