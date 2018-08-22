extern crate env_logger;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Hello");

    info!("Bye");
}

#[test]
fn test_range() {
    let val = [0, 1, 2, 3, 4, 5].iter()
        .map(|i| *i)
        .collect::<Vec<i32>>();
    println!("len: {}, data: {:?}", val.len(), val);
    println!("0..=2: {:?}", val[0..=2].iter().map(|x| x).collect::<Vec<&i32>>());
    println!("2..=5: {:?}", val[2..=5].iter().map(|x| x).collect::<Vec<&i32>>());
}
