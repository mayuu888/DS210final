mod pokemon;
mod clustering;
use clustering::clusters;
use pokemon::file_reader;
use std::error::Error;
use clustering::labelling;
use clustering::info;
fn main() -> Result<(), Box<dyn Error>>{
    let all_pokemon  = file_reader("pokemon_data.csv")?;
    let (basic_num,basic_centroids, special_num, special_centroids) = clusters(&all_pokemon, 4,4);
    //println!("{:?}", basic_centroids);
    //println!("{:?}", special_centroids);
    let (basic_centroid_labels, special_centroid_labels) = labelling(&basic_centroids, &special_centroids);
    //print!("{:?}", centroid_labels);
    // printing clusters, what pokemon is in what cluster and how many are in the cluster
    println!("basic stats:");
    for (c_num, label) in basic_centroid_labels.iter().enumerate() {
        // for all pokemon in the cluster, filter by cluster and get name
        let members: Vec<_> = all_pokemon.iter()
            .zip(basic_num.iter())
            .filter(|&(_, &c)| c == c_num)  
            .map(|(p, _)| p.name.as_str())             
            .collect();
        println!(
            "   cluster {} - {} ({} members)",
            c_num + 1,
            label,
            members.len(),
        );
    }
    println!("special stats:");
    for (c_num, label) in special_centroid_labels.iter().enumerate() {
        // for all pokemon in the cluster, filter by cluster and get name
        let members: Vec<_> = all_pokemon.iter()
            .zip(special_num.iter())
            .filter(|&(_, &c)| c == c_num)  
            .map(|(p, _)| p.name.as_str())             
            .collect();
        println!(
            "   cluster {} - {} ({} members)",
            c_num + 1,
            label,
            members.len(),
        );
    }
    info(&all_pokemon, &basic_centroid_labels,&basic_num,
        &special_centroid_labels,&special_num);
    Ok(())
    
}
