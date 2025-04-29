use std::fs::File;
use std::error::Error;
use csv::Reader;
use ndarray::{Array2,Array1};
use std::collections::HashMap;
pub fn file_reader(file_name: &str)-> Result<Vec<Pokemon>, Box<dyn Error>>{
    let file = File::open(file_name)?;
    let mut reader = Reader::from_reader(file);
    let mut pokemon_list = Vec::with_capacity(1025);
    for re in reader.records(){
        let r = re?;
        println!("{:<15} {:<10} {:<10} {:<20} {:<20}", &r[1],&r[3], &r[4], &r[8], &r[9]);
        let mut pokemon = Pokemon{name: r[1].to_string(), types: vec![r[3].to_string()], abilities: vec![r[8].to_string()]};
        if !r[4].is_empty(){ pokemon.types.push(r[4].to_string())}
        if !r[9].is_empty(){ pokemon.abilities.push(r[9].to_string())}
        pokemon_list.push(pokemon);
    }
    Ok(pokemon_list)
}
#[derive(Debug)]
pub struct Pokemon {
    name: String,
    types: Vec<String>,
    abilities: Vec<String>,
}
fn abilities( pokemon)

