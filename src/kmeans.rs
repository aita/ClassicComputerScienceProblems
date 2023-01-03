use rand::Rng;

fn min(data: &[f64]) -> Option<f64> {
    data.iter().cloned().fold(None, |min, x| match min {
        None => Some(x),
        Some(min) => Some(x.min(min)),
    })
}

fn max(data: &[f64]) -> Option<f64> {
    data.iter().cloned().fold(None, |max, x| match max {
        None => Some(x),
        Some(max) => Some(x.max(max)),
    })
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();
    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn stdev(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - value;
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn zscores(original: &[f64]) -> Option<Vec<f64>> {
    let avg = mean(original)?;
    let std = stdev(original)?;
    if std == 0.0 {
        Some(vec![0.0; original.len()])
    } else {
        Some(original.iter().map(|x| (x - avg) / std).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    originals: Vec<f64>,
    dimensions: Vec<f64>,
}

impl DataPoint {
    pub fn new(initial: Vec<f64>) -> Self {
        Self {
            originals: initial.clone(),
            dimensions: initial,
        }
    }

    pub fn num_dimensions(&self) -> usize {
        self.dimensions.len()
    }

    pub fn distance(&self, other: &DataPoint) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.num_dimensions() {
            let diff = self.dimensions[i] - other.dimensions[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Cluster {
    points: Vec<usize>,
    centroid: DataPoint,
}

impl Cluster {
    pub fn points(&self) -> &[usize] {
        &self.points
    }

    pub fn centroid(&self) -> &DataPoint {
        &self.centroid
    }
}

pub struct KMeans {
    points: Vec<DataPoint>,
    clusters: Vec<Cluster>,
}

impl KMeans {
    pub fn new(k: usize, mut points: Vec<DataPoint>) -> Self {
        if k < 1 {
            panic!("k must be greater than 0");
        }
        let mut kmeans = Self {
            points,
            clusters: Vec::new(),
        };
        kmeans.zscore_normalize();
        for _ in 0..k {
            let random_point = kmeans.random_point();
            kmeans.clusters.push(Cluster {
                points: Vec::new(),
                centroid: random_point,
            });
        }
        kmeans
    }

    fn centroids(&self) -> Vec<&DataPoint> {
        self.clusters.iter().map(|c| &c.centroid).collect()
    }

    fn dimension_slice(&self, dimension: usize) -> Vec<f64> {
        self.points
            .iter()
            .map(|p| p.dimensions[dimension])
            .collect()
    }

    fn zscore_normalize(&mut self) {
        let mut zscored = vec![Vec::new(); self.points.len()];
        for dimension in 0..self.points[0].num_dimensions() {
            let dimension_slice = self.dimension_slice(dimension);
            let zscores = zscores(&dimension_slice).unwrap();
            for (i, zscore) in zscores.iter().enumerate() {
                zscored[i].push(*zscore);
            }
        }

        for (i, dimentions) in zscored.into_iter().enumerate() {
            self.points[i].dimensions = dimentions;
        }
    }

    fn random_point(&self) -> DataPoint {
        let mut rng = rand::thread_rng();
        let mut rand_dimensions = Vec::new();
        for dimension in 0..self.points[0].num_dimensions() {
            let values = self.dimension_slice(dimension);
            let rand_value = rng.gen_range(min(&values).unwrap()..max(&values).unwrap());
            rand_dimensions.push(rand_value);
        }
        DataPoint::new(rand_dimensions)
    }

    fn assign_cluster(&mut self) {
        for (handle, point) in self.points.iter().enumerate() {
            let (idx, _closest) = self
                .centroids()
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    let da = a.distance(point);
                    let db = b.distance(point);
                    da.partial_cmp(&db).unwrap()
                })
                .unwrap();
            let cluster = &mut self.clusters[idx];
            cluster.points.push(handle);
        }
    }

    fn generate_centroids(&mut self) {
        for cluster in self.clusters.iter_mut() {
            if cluster.points.is_empty() {
                continue;
            }
            let mut means = Vec::new();
            for dimension in 0..self.points[0].num_dimensions() {
                let dimension_slice = cluster
                    .points
                    .iter()
                    .map(|i| self.points[*i].dimensions[dimension])
                    .collect::<Vec<f64>>();
                means.push(mean(&dimension_slice).unwrap());
            }
            cluster.centroid = DataPoint::new(means);
        }
    }

    pub fn run(&mut self, max_iterations: usize) -> &[Cluster] {
        for iteration in 0..max_iterations {
            for cluster in self.clusters.iter_mut() {
                cluster.points.clear();
            }
            self.assign_cluster();
            let old_centroids = self.centroids().into_iter().cloned().collect::<Vec<_>>();
            self.generate_centroids();
            let old_centroids = old_centroids.iter().map(|x| x).collect::<Vec<_>>();
            let centroids = self.centroids();
            if old_centroids == centroids {
                tracing::info!("Converged after {} iterations", iteration);
                break;
            }
        }
        &self.clusters
    }
}
