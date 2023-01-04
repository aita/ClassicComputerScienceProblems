use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use ndarray::{Array, Array1, ArrayView1, ArrayView2, Axis};
use ndarray_rand::{rand_distr::Uniform, RandomExt};

struct Neuron<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    weights: Option<Array1<f64>>,
    activation_function: Rc<RefCell<A>>,
    deactivation_function: Rc<RefCell<D>>,
    learning_rate: f64,
    output_cache: Cell<f64>,
    delta: f64,
}

impl<A, D> Neuron<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    fn new(
        weights: Option<Array1<f64>>,
        learning_rate: f64,
        activation_function: Rc<RefCell<A>>,
        deactivation_function: Rc<RefCell<D>>,
    ) -> Self {
        Self {
            weights,
            learning_rate,
            activation_function,
            deactivation_function,
            output_cache: Cell::new(0.0),
            delta: 0.0,
        }
    }

    fn output(&self, inputs: ArrayView1<f64>) -> f64 {
        let output = if let Some(weights) = &self.weights {
            inputs.dot(weights)
        } else {
            0.0
        };
        self.output_cache.set(output);
        output
    }

    fn output_cache(&self) -> f64 {
        self.output_cache.get()
    }

    fn activation_function(&self, val: f64) -> f64 {
        (self.activation_function.borrow())(val)
    }

    fn deactivation_function(&self, val: f64) -> f64 {
        (self.deactivation_function.borrow())(val)
    }
}

struct Layer<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    layer_index: usize,
    neurons: Vec<Neuron<A, D>>,
    output_cache: RefCell<Array1<f64>>,
}

impl<A, D> Layer<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    fn new(
        previous_layer: Option<&Layer<A, D>>,
        layer_index: usize,
        num_neurons: usize,
        learning_rate: f64,
        activation_function: Rc<RefCell<A>>,
        deactivation_function: Rc<RefCell<D>>,
    ) -> Self {
        let mut neurons = Vec::with_capacity(num_neurons);
        for _ in 0..num_neurons {
            let random_weights = if let Some(previous_layer) = previous_layer.as_ref() {
                Some(Array::random(
                    previous_layer.neurons.len(),
                    Uniform::new(0., 1.),
                ))
            } else {
                None
            };
            neurons.push(Neuron::new(
                random_weights,
                learning_rate,
                activation_function.clone(),
                deactivation_function.clone(),
            ));
        }
        Self {
            layer_index,
            neurons,
            output_cache: RefCell::new(Array1::zeros(num_neurons)),
        }
    }

    fn outputs(&self, inputs: ArrayView1<f64>) -> Array1<f64> {
        let output = if self.layer_index > 0 {
            Array1::from_iter(
                self.neurons
                    .iter()
                    .map(|neuron| neuron.activation_function(neuron.output(inputs))),
            )
        } else {
            inputs.to_owned()
        };
        self.output_cache.replace(output.clone());
        output
    }

    fn output_cache(&self) -> Array1<f64> {
        self.output_cache.borrow().clone()
    }

    fn calulate_deltas_for_output_layer(&mut self, expected: ArrayView1<f64>) {
        for (neuron, expected) in self.neurons.iter_mut().zip(expected.iter()) {
            neuron.delta = neuron.deactivation_function(neuron.output_cache())
                * (expected - neuron.output_cache());
        }
    }

    fn calulate_deltas_for_hidden_layer(&mut self, next_layer: &Layer<A, D>) {
        for (index, neuron) in self.neurons.iter_mut().enumerate() {
            let next_weights = Array1::from_iter(
                next_layer
                    .neurons
                    .iter()
                    .map(|neuron| neuron.weights.as_ref().unwrap()[index]),
            );
            let next_deltas =
                Array1::from_iter(next_layer.neurons.iter().map(|neuron| neuron.delta));
            let sum_weights_and_deltas = next_weights.dot(&next_deltas);
            neuron.delta =
                neuron.deactivation_function(neuron.output_cache()) * sum_weights_and_deltas;
        }
    }
}

pub struct Network<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    layers: Vec<Layer<A, D>>,
}

impl<A, D> Network<A, D>
where
    A: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    pub fn new(
        layer_structure: &[usize],
        learning_rate: f64,
        activation_function: A,
        deactivation_function: D,
    ) -> Self {
        if layer_structure.len() < 3 {
            panic!("Network should have at least 3 layers (1 input, 1 hidden, 1 output)");
        }
        let activation_function = Rc::from(RefCell::from(activation_function));
        let deactivation_function = Rc::from(RefCell::from(deactivation_function));
        let layers = Vec::with_capacity(layer_structure.len());
        let mut network = Self { layers };
        let input_layer = Layer::new(
            None,
            0,
            layer_structure[0],
            learning_rate,
            activation_function.clone(),
            deactivation_function.clone(),
        );
        network.layers.push(input_layer);
        for (previous, num_neurons) in layer_structure[1..].iter().enumerate() {
            let layer = Layer::new(
                Some(&network.layers[previous]),
                previous + 1,
                *num_neurons,
                learning_rate,
                activation_function.clone(),
                deactivation_function.clone(),
            );
            network.layers.push(layer);
        }
        network
    }

    pub fn outputs(&self, inputs: ArrayView1<f64>) -> Array1<f64> {
        self.layers.iter().fold(inputs.to_owned(), |inputs, layer| {
            layer.outputs(inputs.view())
        })
    }

    fn backpropagate(&mut self, expected: ArrayView1<f64>) {
        let last_layer = self.layers.len() - 1;
        self.layers[last_layer].calulate_deltas_for_output_layer(expected);
        for i in (0..last_layer - 1).rev() {
            let (current_layer, next_layer) = self.layers.split_at_mut(i + 1);
            current_layer[i].calulate_deltas_for_hidden_layer(&next_layer[0]);
        }
    }

    fn update_weights(&mut self) {
        for i in 0..self.layers.len() - 1 {
            let (front, back) = self.layers.split_at_mut(i);
            let previous_layer = &front[i];
            let layer = &mut back[0];
            for neuron in layer.neurons.iter_mut() {
                let new_weights = neuron.weights.as_ref().unwrap()
                    + neuron.learning_rate * previous_layer.output_cache() * neuron.delta;
                neuron.weights.replace(new_weights);
            }
        }
    }

    pub fn train(&mut self, inputs: ArrayView2<f64>, expected: ArrayView2<f64>) {
        for (inputs, expected) in inputs.outer_iter().zip(expected.outer_iter()) {
            self.outputs(inputs);
            self.backpropagate(expected);
            self.update_weights();
        }
    }

    pub fn validate<T, F>(
        &self,
        inputs: ArrayView2<f64>,
        expected: Vec<T>,
        interpret_output: F,
    ) -> (usize, usize, f64)
    where
        T: PartialEq,
        F: Fn(ArrayView1<f64>) -> T,
    {
        let mut collect = 0;
        for (inputs, expected) in inputs.outer_iter().zip(expected.iter()) {
            let result = interpret_output(self.outputs(inputs).view());
            if result == *expected {
                collect += 1;
            }
        }
        let percentage = collect as f64 / inputs.len_of(Axis(0)) as f64;
        (collect, inputs.len_of(Axis(0)), percentage)
    }
}
