mod resources;

use crate::resources::{Resources};

fn main() {
    let resources = Resources::new();
    println!("{:?}", resources);
    let j = serde_json::to_string(&resources).unwrap();
    println!("{}", j);

}
