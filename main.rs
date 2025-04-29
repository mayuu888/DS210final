mod pokemon;
use pokemon::file_reader;
use pokemon::Pokemon;
fn main() {
    let all_pokemon  = file_reader("pokemon_data.csv");
    for p in all_pokemon.iter(){
        println!("{:?}",p);
    }

}
