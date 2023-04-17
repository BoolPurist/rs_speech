mod error;
use error::Precent;
fn main() {
    let precent = Precent::new(25).unwrap();

    println!("{}", precent.show());

    let invalid = Precent::new(0);

    println!("{:?}", average(&[2, 4]));
    println!("{:?}", average(&[]));

    match invalid {
        Ok(success) => println!("{}", success.show()),
        Err(failure) => println!("{failure}"),
    }
}

fn average(to_calc: &[u32]) -> Option<u32> {
    if to_calc.is_empty() {
        None
    } else {
        Some(to_calc.iter().fold(0, |acc, elem| acc + elem) / to_calc.len() as u32)
    }
}
