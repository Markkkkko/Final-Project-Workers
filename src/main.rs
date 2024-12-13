

mod employee;
mod clustering;
mod graph;
mod utils;

use employee::Employee;
use clustering::{kmeans};
use utils::load_employee_data;
use std::collections::HashMap;

fn main() { //load data
   
    let graph = load_employee_data("/opt/app-root/src/final_project/final_proj/Employe_Performance_dataset.csv");

    // Extract features for clustering 
    let features: Vec<Vec<f64>> = graph.employees.iter()
        .map(|e| vec![e.age as f64, e.experience as f64])
        .collect();


    let standardized_features = standardize(&features); //standarize features

    
    let clusters = create_ordered_clusters_with_priority(&standardized_features, 10); //create manual clusters

   
    print_clusters(&graph.employees, &clusters);


    print_cluster_means(&graph.employees, &clusters); //show mean of cluster
}


fn standardize(features: &Vec<Vec<f64>>) -> Vec<Vec<f64>> { //standarize features
    let num_features = features[0].len();
    let mut means = vec![0.0; num_features];
    let mut stddevs = vec![0.0; num_features];


    for feature_idx in 0..num_features { //create mean using iteraiton, mean of each feature
        let sum: f64 = features.iter().map(|f| f[feature_idx]).sum();
        means[feature_idx] = sum / features.len() as f64;
    }

    // Compute the standard deviation of each feature
    for feature_idx in 0..num_features {
        let variance: f64 = features.iter().map(|f| (f[feature_idx] - means[feature_idx]).powi(2)).sum();
        stddevs[feature_idx] = (variance / features.len() as f64).sqrt();
    }

    // Normalize the features
    features.iter().map(|f| {
        f.iter().enumerate().map(|(i, &value)| {
            (value - means[i]) / stddevs[i] // Standardize each feature
        }).collect()
    }).collect()
}


fn create_ordered_clusters_with_priority(features: &Vec<Vec<f64>>, k: usize) -> Vec<usize> { //prioritize experience
    let mut indexed_features: Vec<(Vec<f64>, usize)> = features.iter().enumerate().map(|(i, f)| (f.clone(), i)).collect();
     //prioritize experience, used algorithm
    
    indexed_features.sort_by(|a, b| {
        let weighted_a = (a.0[1] * 2.0 + a.0[0]); // Experience (index 1) is weighted more heavily
        let weighted_b = (b.0[1] * 2.0 + b.0[0]);
        weighted_a.partial_cmp(&weighted_b).unwrap()
    });

    // Create a cluster vector where each entry will belong to a cluster
    let mut clusters = vec![0; features.len()];

    // Distribute data points into clusters
    for (i, (_, original_index)) in indexed_features.iter().enumerate() {
        let cluster_id = i * k / features.len(); // Divide the sorted data into `k` clusters
        clusters[*original_index] = cluster_id;
    }

    clusters
}

fn print_clusters(employees: &[Employee], clusters: &[usize]) { //show clusters
    for (i, employee) in employees.iter().enumerate() {
        let cluster = clusters[i];
        println!("Employee {} (ID: {}): Age = {}, Experience = {}, Cluster {}", 
                 employee.name, employee.id, 
                 employee.age,
                 employee.experience, cluster);
    }
}

// Print mean of features for each cluster
fn print_cluster_means(employees: &[Employee], clusters: &[usize]) {
    let mut cluster_data: HashMap<usize, Vec<(f64, f64)>> = HashMap::new();

    // Group employees by their cluster and collect their features
    for (i, employee) in employees.iter().enumerate() {
        let cluster = clusters[i];
        let features = (employee.age as f64, employee.experience as f64);

        // Insert the features 
        cluster_data.entry(cluster).or_insert(Vec::new()).push(features);
    }

    for (cluster, data) in cluster_data { //calculate mean age and experience
        let total = data.len() as f64; //calculat mean
        let (sum_age, sum_experience): (f64, f64) = data.iter().fold((0.0, 0.0), |(sum_age, sum_experience), &(age, experience)| {
            (sum_age + age, sum_experience + experience)
        });

        let mean_age = sum_age / total;
        let mean_experience = sum_experience / total;

        println!("Cluster {}: Mean Age = {:.2}, Mean Experience = {:.2}", cluster, mean_age, mean_experience);
    }
}


















