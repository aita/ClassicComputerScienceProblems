use std::fs::File;

use classic_computer_science_problems::neural_network::*;

use ndarray::{s, Array2, ArrayView1};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    class: String,
}
fn main() -> std::io::Result<()> {
    let mut iris_parameters = Vec::new();
    let mut iris_classifications = Vec::new();
    let mut iris_species = Vec::new();

    let file = File::open("iris.csv")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut records = reader.deserialize().collect::<Result<Vec<Record>, _>>()?;
    records.shuffle(&mut thread_rng());
    for record in records {
        iris_parameters.push([
            record.sepal_length,
            record.sepal_width,
            record.petal_length,
            record.petal_width,
        ]);
        match record.class.as_str() {
            "Iris-setosa" => iris_classifications.push([1.0, 0.0, 0.0]),
            "Iris-versicolor" => iris_classifications.push([0.0, 1.0, 0.0]),
            "Iris-virginica" => iris_classifications.push([0.0, 0.0, 1.0]),
            _ => panic!("Unknown iris class"),
        }
        iris_species.push(record.class);
    }
    let mut iris_parameters = Array2::from(iris_parameters);
    let iris_classifications = Array2::from(iris_classifications);

    normalize_by_feature_scaling(&mut iris_parameters);

    let mut iris_network: Network<Sigmoid> = Network::new(&[4, 6, 3], 0.3);

    let iris_interpret_output = |output: ArrayView1<f64>| {
        let max = output.fold(f64::MIN, |acc, &x| acc.max(x));
        if max == output[0] {
            "Iris-setosa".to_string()
        } else if max == output[1] {
            "Iris-versicolor".to_string()
        } else if max == output[2] {
            "Iris-virginica".to_string()
        } else {
            println!("output: {:?}", output);
            panic!("Unknown iris class")
        }
    };

    let iris_trainers = iris_parameters.slice(s![..140_usize, ..]);
    let iris_trainers_corrects = iris_classifications.slice(s![..140_usize, ..]);
    for _ in 0..50 {
        iris_network.train(iris_trainers, iris_trainers_corrects);
    }

    println!("Iris Network Results:");
    let iris_testers = iris_parameters.slice(s![140_usize..150, ..]);
    let iris_testers_corrects = &iris_species[140..150];
    let iris_result =
        iris_network.validate(iris_testers, iris_testers_corrects, iris_interpret_output);

    println!(
        "{} correct of {} = {}%",
        iris_result.0,
        iris_result.1,
        (iris_result.2 * 100.0).round()
    );

    Ok(())
}
