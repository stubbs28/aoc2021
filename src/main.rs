mod day1;
fn day1() {
    println!("==Day One: Sonar Sweep==");
    let s = day1::Scanner::new("data/input1".to_string());
    println!("part one: {}", s.scan(1));
    println!("part two: {}", s.scan(3));
}
mod day2;
fn day2() {
    println!("==Day Two: Dive==");
    let mut h = day2::Helm::new("data/input2".to_string());
    println!("part one: {}", h.navigate(false));
    println!("part two: {}", h.navigate(true));
}
mod day3;
fn day3() {
    println!("==Day Three: Binary Diagnostic==");
    let p = day3::Power::new("data/input3".to_string());
    println!("part one: {}", p.consumption());
    println!("part two: {}", p.life_support());
}
mod day4;
fn day4() {
    println!("==Day Four: Giant Squid==");
    let b = day4::Bingo::new("data/input4".to_string());
    println!("part one: {}", b.winning_score());
    println!("part one: {}", b.losing_score());
}
mod day5;
fn day5() {
    println!("==Day Five: Hydrothermal Venture==");
    let mut v = day5::VentMap::new("data/input5".to_string());
    v.map_horz_vert();
    println!("part one: {}", v.danger_score());
    v.map_diag();
    println!("part two: {}", v.danger_score());
}
mod day6;
fn day6() {
    println!("==Day Six: Lanternfish==");
    let mut s = day6::School::new("data/input6".to_string());
    println!("part one: {}", s.reproduce(80));
    println!("part two: {}", s.reproduce(256 - 80));
}
mod day7;
fn day7() {
    println!("==Day Seven: The Treachery of Whales==");
    let c = day7::Crabs::new("data/input7".to_string());
    println!("part one: {}", c.human_align_cost());
    println!("part two: {}", c.crab_align_cost());
}
mod day8;
fn day8() {
    println!("==Day Eight: Seven Segment Search==");
    let mut n = day8::Notes::new("data/input8");
    println!("part one: {}", n.known_count());
    n.map_signals();
    println!("part two: {}", n.output_sum());
}
mod day9;
fn day9() {
    println!("==Day Nine: Smoke Basin==");
    let mut h = day9::HeightMap::new("data/input9", 100, 100);
    println!("part one: {}", h.danger());
    println!("part one: {}", h.basins());
}
mod day10;
fn day10() {
    println!("==Day Ten: Syntax Scoring==");
    let n = day10::Navigation::new("data/input10");
    println!("part one: {}", n.corrupted_score());
    println!("part two: {}", n.fixed_score());
}
mod day11;
fn day11() {
    println!("==Day Eleven: Dumbo Octopus==");
    let mut o = day11::OctoModel::new("data/input11");
    println!("part one: {}", o.multi_step(100));
    println!("part two: {}", o.get_sync());
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = 11;
    if args.len() > 1 {
        day = match args[1].parse::<i32>() {
            Ok(i) => i,
            Err(err) => panic!("not a valid day: {:?}", err),
        }
    }
    match day {
        1 => day1(),
        2 => day2(),
        3 => day3(),
        4 => day4(),
        5 => day5(),
        6 => day6(),
        7 => day7(),
        8 => day8(),
        9 => day9(),
        10 => day10(),
        11 => day11(),
        _ => println!("invlid day selected"),
    }
}
