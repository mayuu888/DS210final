use std::fs::File;
use std::error::Error;
use csv::Reader;
use ndarray::{Array2};
use std::str::FromStr;
pub fn file_reader(file_name: &str)-> Result<Vec<Pokemon>, Box<dyn Error>>{
    let file = File::open(file_name)?;
    let mut reader = Reader::from_reader(file);
    let mut pokemon_list = Vec::with_capacity(1025);
    for re in reader.records(){
        let r = re?;
        let mut pokemon = Pokemon{name: r[1].to_string(), hp: f32::from_str(&r[11])?, 
            attack: f32::from_str(&r[12])?, defense: f32::from_str(&r[13])?, 
            spatk:f32::from_str(&r[14])?, spdef: f32::from_str(&r[15])?, 
            speed: f32::from_str(&r[16])?, total: i32::from_str(&r[17])?,
            group: r[28git].to_string()};
        pokemon_list.push(pokemon);
    }
    Ok(pokemon_list)
}
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
// make the stats into a normalized vector between 0-1
// returning a 2D array where rows represent a pokemons stats
pub fn normalized_stats(pokemon:&[Pokemon])-> (Array2<f32>, Array2<f32>){
    // finding max value for each category
    let max_hp = pokemon.iter().map(|p| p.hp).fold(0.0,f32::max);
    let max_attack = pokemon.iter().map(|p| p.attack).fold(0.0,f32::max);
    let max_defense = pokemon.iter().map(|p| p.defense).fold(0.0,f32::max);
    let max_speed = pokemon.iter().map(|p| p.speed).fold(0.0,f32::max);
    let max_spatk = pokemon.iter().map(|p| p.spatk).fold(0.0,f32::max);
    let max_spdef = pokemon.iter().map(|p| p.spdef).fold(0.0,f32::max);
    //println!("hp: {:?}, attack: {:?}, defense: {:?}, speed: {:?}",max_hp,max_attack,max_defense,max_speed);
    // making 2D array and normalizing all the values betwen 0-1 by dividng by max value
    (Array2::from_shape_vec((pokemon.len(),4), pokemon.iter().flat_map(|p|
        vec![p.hp/max_hp, p.attack/max_attack, p.defense/max_defense, p.speed/max_speed])
        .collect()).expect("failed"),
    Array2::from_shape_vec((pokemon.len(),2), pokemon.iter().flat_map(|p|
        vec![p.spatk/max_spatk, p.spdef/max_spdef])
        .collect()).expect("failed"))
}