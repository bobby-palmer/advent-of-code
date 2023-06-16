pub mod math {
    #[derive(Debug)]
    pub enum Operator {
        Mult(Symbol, Symbol),
        Add(Symbol, Symbol),
    }
    impl Operator {
        pub fn eval(&self, old: i64) -> i64 {
            match self {
                Operator::Add(left, right) => left.unwrap(old) + right.unwrap(old),
                Operator::Mult(left, right) => left.unwrap(old) * right.unwrap(old),
            }
        }
        pub fn from_string(eqn: &str) -> Option<Self> {
            let mut iter = eqn.split_whitespace();
            let left = Symbol::from_string(iter.next().unwrap());
            let op = iter.next().unwrap();
            let right = Symbol::from_string(iter.next().unwrap());
            match op {
                "*" => Some(Operator::Mult(left, right)),
                "+" => Some(Operator::Add(left, right)),
                _ => None,
            }
        }
    }
    #[derive(Debug)]
    pub enum Symbol {
        Old,
        Number(i64),
    }
    impl Symbol {
        fn unwrap(&self, value: i64) -> i64 {
            match self {
                Symbol::Old => value,
                Symbol::Number(num) => *num,
            }
        }
        fn from_string(symbol: &str) -> Self {
            match symbol {
                "old" => Self::Old,
                _ => Symbol::Number(symbol.parse::<i64>().unwrap()),
            }
        }
    }
}
