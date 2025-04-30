mod pokemon;
use pokemon::file_reader;
use std::error::Error;
use pokemon::normalized_stats;
fn main() -> Result<(), Box<dyn Error>>{
    let all_pokemon  = file_reader("pokemon_data.csv")?;
    let stats = normalized_stats(&all_pokemon);
    print!("{:?}", stats);
    Ok(())

}
