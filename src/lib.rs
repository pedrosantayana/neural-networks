mod neuron;
mod layer;
mod network;

#[cfg(test)]
mod tests {
    use crate::neuron::Neuron;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn neuron() {
        //let n = Neuron::new(32usize);
    }
}
