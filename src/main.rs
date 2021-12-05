mod sonar;
fn day1() {
    println!("==Day One==");
    let s = sonar::Scanner::new("data/input1".to_string());
    println!("part one: {}", s.scan(1));
    println!("part two: {}", s.scan(3));
}
fn main() {
    //day1();
}
