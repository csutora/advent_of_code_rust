use crate::solutions::*;

// export the module of each day here
pub mod day_01;

pub fn register_solutions(solutions: &mut Solutions) {
    // and add a line for each day here
    solutions.add_solution(2024, 1, day_01::solve as SolutionFn);
}
