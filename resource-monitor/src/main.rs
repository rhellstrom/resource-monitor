mod resources;

use crate::resources::{Resources};

fn main() {
    let resources = Resources::new();
    println!("{}", resources.serialize());

}
