mod sonar;
mod navigation;
mod diagnostics;
mod games;
fn day1() {
    println!("==Day One: Sonar Sweep==");
    let s = sonar::Scanner::new("data/input1".to_string());
    println!("part one: {}", s.scan(1));
    println!("part two: {}", s.scan(3));
}
fn day2() {
    println!("==Day Two: Dive==");
    let mut h = navigation::Helm::new("data/input2".to_string());
    println!("part one: {}", h.navigate(false));
    println!("part two: {}", h.navigate(true));
}
fn day3() {
    println!("==Day Three: Binary Diagnostic==");
    let p = diagnostics::Power::new("data/input3".to_string());
    println!("part one: {}", p.consumption());
    println!("part two: {}", p.life_support());
}
fn day4() {
    println!("==Day Four: Giant Squid==");
    let b = games::Bingo::new("data/input4".to_string());
    println!("part one: {}", b.winning_score());
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
    3=>day3(),
    4=>day4(),
    _=>println!("invlid day selected"),
    }
}
