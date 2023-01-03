use classic_computer_science_problems::kmeans::*;

fn main() {
    tracing_subscriber::fmt::init();

    let points = vec![
        DataPoint::new(vec![2.0, 1.0, 1.0]),
        DataPoint::new(vec![2.0, 2.0, 5.0]),
        DataPoint::new(vec![3.0, 1.5, 2.5]),
    ];
    let mut kmeans_test = KMeans::new(2, points);
    let test_clusters = kmeans_test.run(100);
    for cluster in test_clusters {
        println!("Cluster: {:?}", cluster);
    }
}
