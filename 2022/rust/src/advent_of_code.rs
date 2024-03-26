pub struct AdventOfCode {
    pub days: Vec<Box<dyn AdventOfCodeDay>>
}

pub trait AdventOfCodeDay {
    fn parse_input(&self, input: String);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}