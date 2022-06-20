use crate::layer::Layer;

struct Network {
    layers: Vec<Layer>,
}

impl Network {
    fn sigmoid_eval_layers(&self, i: &Vec<f32>) -> Vec<f32> {
        let mut out = vec![];
        let mut 
        let rec = |j: &Vec<f32>, | {
            
        };


        let layer = l.unwrap_or(0);
        if layer == self.layers.len() {

        } else {
            self.sigmoid_eval_layers(self.layers[layer].sigmoid_outputs(i), layer+1)
        }
    }
}
