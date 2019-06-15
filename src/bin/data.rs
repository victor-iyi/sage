extern crate sage;

use sage::kg::from_jsonld;

fn main() {
    let path = "resources/schema-org/movie.jsonld";
    println!("{}", from_jsonld(path));
}
