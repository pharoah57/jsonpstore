pub mod dbAdapter;
use dbAdapter::dbBuilder;

fn main() {
    let mut db = dbBuilder::new("sqlite").finalize();
    let name = db.get_name();
    println!("The db type is {}", name);
    db.open();
}