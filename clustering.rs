use crate::pokemon;
use pokemon::normalized_stats;
use linfa::traits::Fit;
use linfa_clustering::KMeans;
use ndarray::{Array2,Array1};
use pokemon::Pokemon;
use linfa::prelude::Predict;
use std::collections::HashMap;
use std::io;
// performing k means clustering on the normalized pokemon stats
// takes the pokemon and number of clusters as an argument
// returns an array of the cluster number each pokemon is in and the centroids of each cluster
pub fn clusters(pokemon: &[Pokemon], basic_c: usize, special_c: usize)-> 
    (Array1<usize>, Array2<f32>, Array1<usize>, Array2<f32>){
    let (basic_stats,special_stats) = normalized_stats(pokemon);
    // dataset for linfa usage
    let basic_data = linfa::Dataset::from(basic_stats);
    let special_data = linfa::Dataset::from(special_stats);
    // running k means, max iteration is 100 and if centroids stop changing more than 1e-5 it stops, training the model
    let basic_model = KMeans::params(basic_c).max_n_iterations(100)
        .tolerance(1e-5).fit(&basic_data).expect("failed");
    let special_model = KMeans::params(special_c).max_n_iterations(100)
        .tolerance(1e-5).fit(&special_data).expect("failed");
    // getting results of cluster number for each pokemon and the centroids
    let basic_num = basic_model.predict(&basic_data);
    let special_num = special_model.predict(&special_data);
    let basic_centroids = basic_model.centroids().to_owned();
    let special_centroids = special_model.centroids().to_owned();
    (basic_num,basic_centroids, special_num, special_centroids)
}
// cluster labeling based on centroids 
pub fn labelling(basic_centroids: &Array2<f32>, 
    special_centroids: &Array2<f32>) -> (Vec<String>, Vec<String>){
    (basic_centroids.rows().into_iter().map(|cluster| {
        // taking out the specific stats from the centroid
        let stats = cluster.to_vec();
        let hp = stats[0];
        let attack = stats[1];
        let defense = stats[2];
        let speed = stats[3];   
        // determing cluster type based on centroid stat combinations
        match () {
            // strong
            _ if attack > 0.6 && speed > 0.4 => "strong".to_string(),
            // decent
            _ if attack > 0.4 && speed > 0.4 => "decent".to_string(),
            // average
            _ if attack > 0.2 && speed > 0.2 && hp > 0.2 && defense >0.2 => "average".to_string(),
            // weak
            _ => "weak".to_string()
        }
    }).collect(),
    special_centroids.rows().into_iter().map(|cluster| {
        // taking out the specific stats from the centroid
        let stats = cluster.to_vec();
        let spatk = stats[0];
        let spdef = stats[1];  
        // determing cluster type based on centroid stat combinations
        match () {
            // strong
            _ if spatk >= 0.4 && spdef >= 0.4 => "strong".to_string(),
            // decent
            _ if spatk >= 0.5 && spdef < 0.5 => "strong attack only".to_string(),
            // average
            _ if spatk < 0.5 && spdef >= 0.4  => "strong defense only".to_string(),
            // weak
            _ => "weak".to_string()
        }
    }).collect())

}
pub fn cluster_map(cluster_num: &[(String, usize)], cluster_names:&[String])-> HashMap<String,String>{
    cluster_num.iter().map(|(name,idx)|{
        let label = &cluster_names[*idx];
        (name.clone(),label.clone())
    }).collect()
}
pub fn info(all_pokemon: &[Pokemon], basic_centroid_labels:&Vec<String>,basic_num:&Array1<usize>,
    special_centroid_labels:&Vec<String>,special_num:&Array1<usize>){
    println!("enter a pokemon name with first letter capitalized ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim();
    get_cluster(all_pokemon,name, "basic stats",basic_centroid_labels,basic_num);
    get_cluster(all_pokemon,name,"special stats",special_centroid_labels,special_num);
    let mut ranked: Vec<(&Pokemon,i32)> = all_pokemon.iter().map(|p| (p,p.total)).collect();
    ranked.sort_by(|a,b| b.1.cmp(&a.1));
    let rank_map: HashMap<String,usize> = ranked.iter().enumerate().map(|(rank,(p,_))| 
        (p.name.clone(),rank+1)).collect();
    match rank_map.get(name){
        Some(rank)=> {
            println!("rank #: {}/{}",rank,all_pokemon.len());
        },
        _ => println!("error: '{}' not found in dataset", name),
    }
    let pokemon = all_pokemon.iter().find(|p| p.name == name);
    match pokemon{
        Some(p)=> println!("group: {}", p.group),
        None => println!("not found"),
    }
    
}
pub fn get_cluster(all_pokemon: &[Pokemon], name: &str, cluster_type: &str, 
    centroid_labels:&Vec<String>,cluster_num:&Array1<usize>){
    let pokemon_num: Vec<(String, usize)> = all_pokemon.iter()
    .zip(cluster_num.iter())
    .map(|(pokemon, &cluster)| (pokemon.name.clone(), cluster))
    .collect();
    let cluster_map = cluster_map(&pokemon_num,&centroid_labels);
    match cluster_map.get(name){
        Some(cluster)=> {
            println!("{}: {}",cluster_type, cluster);
        },
        _ => println!("error: '{}' not found in dataset", name),
    }
}