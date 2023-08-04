mod calculator;

fn main() {
    let result = calculator::calc("5 / 2").unwrap();

    println!("{result}");
}
