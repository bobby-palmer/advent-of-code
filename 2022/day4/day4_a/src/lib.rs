pub struct Range {
    upper: u16,
    lower: u16,
}
impl Range {
    pub fn new(lower: &str, upper:&str) -> Range {
        let lower: u16 = lower.parse().unwrap();
        let upper: u16 = upper.parse().unwrap();
        Range {
            upper,
            lower,
        }
    }
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
    pub fn intersects(&self, other: &Range) -> bool {
        (self.lower <= other.lower && self.upper >= other.lower) ||
        (self.lower <= other.upper && self.upper >= other.upper) ||
        (other.contains(self))
    }
}
pub fn get_ranges(line: &str) -> (Range, Range) {
    let (range1, range2) = line.split_once(',').expect("Invalid String!");
    let (lower1, upper1) = range1.split_once('-').expect("Invalid String!");
    let (lower2, upper2) = range2.split_once('-').expect("Invalid String!");
    (
        Range::new(lower1, upper1),
        Range::new(lower2, upper2),
    ) 
}
pub fn one_contains_other(range1: Range, range2: Range) -> bool {
    range1.contains(&range2) || range2.contains(&range1)
}
