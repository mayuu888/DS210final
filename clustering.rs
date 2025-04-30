/*
does all the clustering and labelling of clusters
and also finds the rank and group of the pokemon
function prompting user and that prints all info is also in here
*/
use crate::pokemon;
use pokemon::normalized_stats;
use linfa::traits::Fit;
use linfa_clustering::KMeans;
use ndarray::{Array2,Array1};
use pokemon::Pokemon;
use linfa::prelude::Predict;
use std::collections::HashMap;
use std::io;
/*
performing k means clustering on the normalized pokemon stats
takes the pokemon and number of clusters for basic and special stats as an argument
returns a tuple of arrays of the cluster number each pokemon is in and 
the centroids of each cluster for both basic stats and special stats
*/
pub fn clusters(pokemon: &[Pokemon], basic_c: usize, special_c: usize)-> 
    (Array1<usize>, Array2<f32>, Array1<usize>, Array2<f32>){
    // first get the normalized stats for basic and special
    let (basic_stats,special_stats) = normalized_stats(pokemon);
    // creating datasets for linfa usage which takes array2 
    // and output a dataset and i store and use
    let basic_data = linfa::Dataset::from(basic_stats);
    let special_data = linfa::Dataset::from(special_stats);
    // running k means, max iteration is 100 and if centroids stop 
    // changing more than 1e-5 it stops, training the model, fitting to the 
    // dataset by linfa and then follows the specific number of clusters
    let basic_model = KMeans::params(basic_c).max_n_iterations(100)
        .tolerance(1e-5).fit(&basic_data).expect("failed");
    let special_model = KMeans::params(special_c).max_n_iterations(100)
        .tolerance(1e-5).fit(&special_data).expect("failed");
    // getting results of cluster number for each pokemon and the centroids
    // using predict to it calculates the euclidean distance for each data point
    // which then assigns each data point to the nearest cluter and then return
    // the array of the corresponding cluster number for each datapoint
    // also returns array2 containing the points of each centroid by using .centroids()
    let basic_num = basic_model.predict(&basic_data);
    let special_num = special_model.predict(&special_data);
    let basic_centroids = basic_model.centroids().to_owned();
    let special_centroids = special_model.centroids().to_owned();
    (basic_num,basic_centroids, special_num, special_centroids)
}
/*
cluster labeling based on centroids, input the centroids for basic and special
then it looks at each of the stats and it labels the overall cluster
based on the stats of the centroid
this way all the pokemon in a cluster are labelled based on the cluster it was placed in
so this way i categorize the pokemon that are similar to each other based on ther
basic and special stats seperately
and then returns the corresponding cluster labels for basic and special stats clusters
*/
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
            // strong attack only
            _ if spatk >= 0.5 && spdef < 0.5 => "strong attack only".to_string(),
            // strong defense only
            _ if spatk < 0.5 && spdef >= 0.4  => "strong defense only".to_string(),
            // weak
            _ => "weak".to_string()
        }
    }).collect())

}
/*
takes a zip of all the pokemon and the corresponding cluster number it was given 
and takes the list of cluster names
returns a hashmap of pokemon names to their corresponding cluster labels
looks up the cluster name by index since they match uo, then it makes the 
name label pair and turns it into a hashmap 
*/
pub fn cluster_map(cluster_num: &[(String, usize)], cluster_names:&[String])-> HashMap<String,String>{
    cluster_num.iter().map(|(name,idx)|{
        let label = &cluster_names[*idx];
        (name.clone(),label.clone())
    }).collect()
}
/*
takes the slice of all pokemon, the name that the user specifie, the cluster type 
which is easir basic or special, and then the centroid labels and cluster number
so that i can make the zip  and call cluster map and then enter the needed parameters
and so here this prints out the cluster tyoe and the correspinding cluster label 
that the pokemon is which is the name parameter
doesnt return anything its for calling cluster_map and presenting the result
*/
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
/*
this function prints out all the info and also does the user input
called from main and takes all the info to make main easier to read 
asks the user for a pokemon name
then calls get cluster which will print out the cluster name it is in, called twice for
basis and special stats
after that i also find the rank of the pokemon based on the total stats category of the pokemon
but i first get all the total stats of all pokemon and store them in a vector with the
corresponding pokemon and then i sort the pokemon and total stat pair by the total stat
and then like cluster map i make a rank map so that i can correlate name with the rank
since its now sorted and so each pokemon name maps to its rank, where smaller number is stronger
and then i also printed the group of the pokemon by iterating through all pokemon to find the matching name
*/
pub fn info(all_pokemon: &[Pokemon], basic_centroid_labels:&Vec<String>,basic_num:&Array1<usize>,
    special_centroid_labels:&Vec<String>,special_num:&Array1<usize>){
    // user input
        println!("enter a pokemon name with first letter capitalized ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim();
    // cluster printing
    get_cluster(all_pokemon,name, "basic stats",basic_centroid_labels,basic_num);
    get_cluster(all_pokemon,name,"special stats",special_centroid_labels,special_num);
    // finding rank out of all pokemon
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
    // also printing what group it is in, most common is ordinary
    // also fossil, legendary, baby, ancient paradox
    let pokemon = all_pokemon.iter().find(|p| p.name == name);
    match pokemon{
        Some(p)=> println!("group: {}", p.group),
        None => println!("not found"),
    }
    
}
