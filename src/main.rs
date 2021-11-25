extern crate advent;

fn main() {
    println!("Hello, world!");

    let lines = advent::read_input("2018", "1");
    let p1 = advent::year_2018::day_1::part_1(lines);
    println!("{p1}");
}
