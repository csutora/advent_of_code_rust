// advent of code framework
// made by marton csutora (github.com/csutora)
//
// i've tried to minimise the boilerplate code required for each solution:
// - when solving a new day, add two lines in the mod.rs of the respective year
// - when starting a new year, add a line in solutions.rs
// both are documented in the mentioned files.
//
// when starting out, add your session token in a .env file
// (see read_session_token() in inputs.rs)
//
// good luck with advent of code!

mod inputs;
mod solutions;

mod year_2024;

fn main() {
    let mut args = std::env::args().skip(1);

    let year = args
        .next()
        .and_then(|y| y.parse::<u64>().ok())
        .unwrap_or_else(|| {
            eprintln!("usage: aoc <year> <day> (1-25)");
            std::process::exit(1);
        });

    let day = args
        .next()
        .and_then(|d| d.parse::<u8>().ok())
        .filter(|&n| (1..=25).contains(&n))
        .unwrap_or_else(|| {
            eprintln!("usage: aoc <year> <day> (1-25)");
            std::process::exit(1);
        });

    let solution_functions = solutions::Solutions::new();
    if let Some(solution_fn) = solution_functions.get(year, day) {
        let (part1, part2) = solution_fn(inputs::get_for(year, day));
        println!("part 1: {}\npart 2: {}", part1, part2);
    } else {
        eprintln!(
            "solution for day {} of year {} not implemented yet",
            day, year
        );
        std::process::exit(1);
    }
}
