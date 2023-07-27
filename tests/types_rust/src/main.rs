pub mod types_api;
pub mod types_api2;

mod tests;

fn main() {
    let test_data = tests::types_all_data::TypesAll::default();
    let data = test_data.to_bytes();
    test_data.check(&data);

    println!("Done");
}
