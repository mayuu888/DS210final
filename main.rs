pub mod pokemon;
pub mod clustering;
use clustering::clusters;
use pokemon::file_reader;
use std::error::Error;
use clustering::labelling;
use clustering::info;
fn main() -> Result<(), Box<dyn Error>>{
    // reading the csv file
    let all_pokemon  = file_reader("pokemon_data.csv")?;
    // getting the array of cluster numbers,centroid points for basic and special stats
    let (basic_num,basic_centroids, special_num, special_centroids) = clusters(&all_pokemon, 4,4);
    //println!("{:?}", basic_centroids);
    //println!("{:?}", special_centroids);
    // assigns labelling for basic and special centroids
    let (basic_centroid_labels, special_centroid_labels) = labelling(&basic_centroids, &special_centroids);
    //print!("{:?}", centroid_labels);
    println!("basic stats:");
    // printing all clusters, showing how many pokemon are in each cluster 
    for (c_num, label) in basic_centroid_labels.iter().enumerate() {
        // for all pokemon in the cluster zips with cluster number, filter by cluster 
        // maps filtered results to just pokemon name, originaly did output all the 
        // pokemon names for each cluster but looks too messy
        // instead only prints out the number of pokemon in each cluster
        let members: Vec<_> = all_pokemon.iter()
            .zip(basic_num.iter())
            .filter(|&(_, &c)| c == c_num)  
            .map(|(p, _)| p.name.as_str())             
            .collect();
        println!(
            "   cluster {} - {} ({} pokemon)",
            c_num + 1,
            label,
            members.len(),
        );
    }
    println!("special stats:");
    // loops for each cluster/ centroid labels which is same number
    for (c_num, label) in special_centroid_labels.iter().enumerate() {
        // for all pokemon in the cluster zips with cluster number, filter by cluster 
        // maps filtered results to just pokemon name, originaly did output all the 
        // pokemon names for each cluster but looks too messy
        // instead only prints out the number of pokemon in each cluster
        let members: Vec<_> = all_pokemon.iter()
            .zip(special_num.iter())
            .filter(|&(_, &c)| c == c_num)  
            .map(|(p, _)| p.name.as_str())             
            .collect();
        println!(
            "   cluster {} - {} ({} pokemon)",
            c_num + 1,
            label,
            members.len(),
        );
    }
    // calls the info method from clustering so that i get print out all the info
    // ask user for info, all the clustering and result finding in this method
    info(&all_pokemon, &basic_centroid_labels,&basic_num,
        &special_centroid_labels,&special_num);
    Ok(())
}
#[cfg(test)]
mod tests{
    // makes sure i can use everything all modules and stuff
    use super::*; 
    use super::pokemon::{Pokemon, file_reader, normalized_stats};
    use super::clustering::{clusters, labelling,cluster_map};
    use ndarray::array;
    // made sime sample pokemon 
    fn create_test_pokemon() -> Vec<Pokemon> {
        vec![
            Pokemon {
                name: "Strong1".to_string(),
                hp: 100.0,
                attack: 120.0,
                defense: 80.0,
                spatk: 90.0,
                spdef: 80.0,
                speed: 110.0,
                total: 580,
                group: "TestGroup".to_string(),
            },
            Pokemon {
                name: "Weak1".to_string(),
                hp: 50.0,
                attack: 40.0,
                defense: 50.0,
                spatk: 40.0,
                spdef: 50.0,
                speed: 40.0,
                total: 270,
                group: "TestGroup".to_string(),
            },
        ]
    }
    
    #[test]
    fn test_normalized_stats() {
        let pokemon = create_test_pokemon();
        let (basic_stats, special_stats) = normalized_stats(&pokemon);
        // make sure that the all stats were kept 
        assert_eq!(basic_stats.shape(), [2, 4]);
        assert_eq!(special_stats.shape(), [2, 2]);
        
        // make sure values were normalized between 0 and 1
        assert!(basic_stats.iter().all(|&x| x >= 0.0 && x <= 1.0));
        assert!(special_stats.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }
    // make sure that correct # of clusters were made and that the centroid have correct dimensions
    #[test]
    fn test_clustering() {
        let pokemon = create_test_pokemon();
        let (basic_num, basic_centroids, special_num, special_centroids) = clusters(&pokemon, 2, 2);
        
        assert_eq!(basic_num.len(), 2);
        assert_eq!(special_num.len(), 2);
        assert_eq!(basic_centroids.shape(), [2, 4]);
        assert_eq!(special_centroids.shape(), [2, 2]);
    }
    // makes sure that centroid stats get labelled correctly according 
    // to the rules that i defined based on the results of the overall stats
    #[test]
    fn test_labelling() {
        let basic_centroids = array![
            [0.8, 0.9, 0.7, 0.8],  // strong
            [0.3, 0.4, 0.3, 0.4],  // weak
        ];
        
        let special_centroids = array![
            [0.7, 0.7],  // strong
            [0.6, 0.3],   // strong attack only
        ];
        
        let (basic_labels, special_labels) = labelling(&basic_centroids, &special_centroids);
        assert_eq!(basic_labels, vec!["strong", "average"]);
        assert_eq!(special_labels, vec!["strong", "strong attack only"]);
    }
    // make sure that the cluster map function maps pokemon with labels correctly
    #[test]
    fn test_cluster_map() {
        let cluster_num = vec![
            ("Strong1".to_string(), 0),
            ("Weak1".to_string(), 1),
        ];
        
        let cluster_names = vec!["strong".to_string(), "weak".to_string()];
        let map = cluster_map(&cluster_num, &cluster_names);
        
        assert_eq!(map["Strong1"], "strong");
        assert_eq!(map["Weak1"], "weak");
    }
    
}
