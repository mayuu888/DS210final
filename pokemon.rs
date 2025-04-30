/*
pokemon module which reads the csv file and makes the struct for pokemon
and also makes the stats into normalized arrays
*/
use std::fs::File;
use std::error::Error;
use csv::Reader;
use ndarray::{Array2};
use std::str::FromStr;
// reads the csv file and then returns the vector of all pokemon 
pub fn file_reader(file_name: &str)-> Result<Vec<Pokemon>, Box<dyn Error>>{
    // reading file and making empty vector to store all ookemin
    let file = File::open(file_name)?;
    let mut reader = Reader::from_reader(file);
    let mut pokemon_list = Vec::with_capacity(1025);
    for re in reader.records(){
        let r = re?;
        // for every line which is a pokemon, it takes the data that i need to make the pokemon
        let mut pokemon = Pokemon{name: r[1].to_string(), hp: f32::from_str(&r[11])?, 
            attack: f32::from_str(&r[12])?, defense: f32::from_str(&r[13])?, 
            spatk:f32::from_str(&r[14])?, spdef: f32::from_str(&r[15])?, 
            speed: f32::from_str(&r[16])?, total: i32::from_str(&r[17])?,
            group: r[28].to_string()};
        // add each pokemon
        pokemon_list.push(pokemon);
    }
    // return the list of pokemon
    Ok(pokemon_list)
}
// this is my pokemon struct which has all the fields that i wanted to use
// and keep track of for my clustering and results that i print out to the user
// each field is labelled and has correspoinding type
#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub spatk: f32,
    pub spdef: f32,
    pub speed: f32,
    pub total: i32,
    pub group: String,
}
/*
make the stats into a normalized vector between 0-1
returning a 2D array where rows represent a pokemons stats
takes the slice of all pokemon 
*/
pub fn normalized_stats(pokemon:&[Pokemon])-> (Array2<f32>, Array2<f32>){
    // finding max value for each stat through iterating, mapping and folding
    // min is just 0 because you cant have negative stat, and just storing it
    let max_hp = pokemon.iter().map(|p| p.hp).fold(0.0,f32::max);
    let max_attack = pokemon.iter().map(|p| p.attack).fold(0.0,f32::max);
    let max_defense = pokemon.iter().map(|p| p.defense).fold(0.0,f32::max);
    let max_speed = pokemon.iter().map(|p| p.speed).fold(0.0,f32::max);
    let max_spatk = pokemon.iter().map(|p| p.spatk).fold(0.0,f32::max);
    let max_spdef = pokemon.iter().map(|p| p.spdef).fold(0.0,f32::max);
    //println!("hp: {:?}, attack: {:?}, defense: {:?}, speed: {:?}",max_hp,max_attack,max_defense,max_speed);
    // making tuple of 2D arrays and normalizing all the values betwen 0-1 by dividng by max value
    // then returning it, each row is pokemon and the columns are the corresponding stats
    // created from raw data and the specified dimensions, the # of pokemon as rows
    // and number of columns is number of stats
    (Array2::from_shape_vec((pokemon.len(),4), pokemon.iter().flat_map(|p|
        vec![p.hp/max_hp, p.attack/max_attack, p.defense/max_defense, p.speed/max_speed])
        .collect()).expect("failed"),
    Array2::from_shape_vec((pokemon.len(),2), pokemon.iter().flat_map(|p|
        vec![p.spatk/max_spatk, p.spdef/max_spdef])
        .collect()).expect("failed"))
}