mod sonar;
mod navigation;
fn day1() {
    println!("==Day One: Sonar Sweep==");
    let s = sonar::Scanner::new("data/input1".to_string());
    println!("part one: {}", s.scan(1));
    println!("part two: {}", s.scan(3));
}
fn day2() {
    println!("==Day Two: Dive==");
    let mut h = navigation::Helm::new("data/input2".to_string());
    h.navigate();
    println!("part one: {}", h.location());
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = 0;
    if args.len() > 1 {
        day = match args[1].parse::<i32>() {
            Ok(i) => i,
            Err(err) => panic!("not a valid day: {:?}", err),
        }
    }
    match day {
    1=>day1(),
    2=>day2(),
    _=>println!("invlid day selected"),
    }
}
