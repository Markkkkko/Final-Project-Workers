
extern crate ndarray; //create ndarray
use ndarray::{Array2, Array1, s};
use rand::seq::SliceRandom;

pub fn kmeans(features: &[Vec<f64>], k: usize) -> Vec<usize> { //create vec using kmeans
    let n = features.len(); //legnth of n
    let mut clusters = vec![0; n]; //create vector
    let data = Array2::from_shape_vec((n, features[0].len()), features.iter().flat_map(|x| x.iter()).cloned().collect()).unwrap();

    
    let mut rng = rand::thread_rng(); //using random points to create cluster
    let mut centroids = Vec::new(); //create new cluster
    let mut indices: Vec<usize> = (0..n).collect();
    indices.shuffle(&mut rng); //converges after collecting data
    for i in 0..k { //iterate 0 to random k
        centroids.push(data.slice(s![indices[i], ..]).to_owned());
    } 

    let mut prev_centroids = vec![Array1::<f64>::zeros(centroids[0].shape()[0]); k];
    let mut converged = false;

    while !converged { 
        
        for i in 0..n {
            let mut min_distance = f64::MAX;
            let mut cluster = 0;
            for j in 0..k { //assign to nearest cluster
                let distance = euclidean_distance(&data.slice(s![i, ..]).to_owned(), &centroids[j]);
                if distance < min_distance {
                    min_distance = distance;
                    cluster = j;
                }
            }
            clusters[i] = cluster;
        }

        
        prev_centroids.clone_from_slice(&centroids); //create new centroid
        for j in 0..k { //iterate
            let cluster_points = data.indexed_iter()
                .filter(|(i, _)| clusters[i.0] == j)  
                .map(|(_, point)| Array1::from(vec![*point])) //maps to new cluster
                .collect::<Vec<Array1<f64>>>();

            if !cluster_points.is_empty() { //condition and crate new centroid
                let new_centroid = mean(&cluster_points);
                centroids[j] = new_centroid;
            }
        }

        // Check for convergence (centroids not changing)
        converged = centroids.iter().zip(prev_centroids.iter()).all(|(c1, c2)| c1 == c2);
    }

    clusters
}

fn euclidean_distance(a: &Array1<f64>, b: &Array1<f64>) -> f64 { //based off of distance
    a.iter() //distance
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn mean(points: &[Array1<f64>]) -> Array1<f64> { //mean calculation
    let mut sum = Array1::zeros(points[0].len()); //start with 0 array
    for point in points {
        sum = &sum + point;
    }
    sum / points.len() as f64
}

