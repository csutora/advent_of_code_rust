use std::collections::HashMap;

pub type SolutionFn = fn(Vec<String>) -> (String, String);

pub struct Solutions {
    solutions: HashMap<u64, HashMap<u8, SolutionFn>>,
}

impl Solutions {
    pub fn new() -> Self {
        let mut solutions = Self {
            solutions: HashMap::new(),
        };

        // add a line for each year below
        // these call each year's module to add the contained solutions
        // so that the main function can refer to them
        crate::year_2024::register_solutions(&mut solutions);

        solutions
    }

    pub fn add_solution(&mut self, year: u64, day: u8, solution: SolutionFn) {
        self.solutions
            .entry(year)
            .or_insert_with(HashMap::new)
            .insert(day, solution);
    }

    pub fn get(&self, year: u64, day: u8) -> Option<SolutionFn> {
        self.solutions
            .get(&year)
            .and_then(|year_solutions| year_solutions.get(&day))
            .copied()
    }
}
