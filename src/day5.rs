use std::fs::read_to_string;

pub(crate) fn solve_p1() {
    let input = read_to_string("inputs/day5.txt").expect("Input reading failed: ");
    //let input = EXAMPLE;
    let mut input = input.trim().split("\n\n");
    let rules_str = input.next().expect("File is empty");
    let rules_length = rules_str.lines().count();
    let values_str = input.next().expect("Missing input values after rules");
    assert!(
        input.next().is_none(),
        "Incorrect format, input must only have 2 parts (rules and values), found at least 3"
    );

    let rules = rules_str
        .lines()
        .enumerate()
        .map(|(rule_line, y)| {
            let mut x = y.split('-');
            let left = x
                .next()
                .expect("Missing left bound")
                .parse()
                .expect("Failed parsing left bound");
            let right = x
                .next()
                .expect("Missing right bound")
                .parse()
                .expect("Failed parsing right bound");
            match Expr::parse_range(left, right) {
                Ok(ex) => ex,
                Err(e) => {
                    panic!("{e}\n\tRule:{y}\n\tat line: {rule_line}")
                }
            }
        })
        .reduce(|acc, b| Expr::Or(Box::new(acc), Box::new(b)))
        .expect("We expect at least one rule");

    let solp1 = values_str
        .lines()
        .enumerate()
        .map(|(values_line, x)| {
            let id = match x.parse::<Number>() {
                Ok(a) => a,
                Err(e) => {
                    // plus 2 because of \n\n between rules and values
                    let values_line = values_line + rules_length + 2;
                    panic!("{e}\n\tAt line: {values_line}\n\tFor value: '{x}'")
                }
            };
            Id::parse(id, &rules)
        })
        .filter(|x| matches!(x, Id::Fresh(_)))
        .count();

    println!("Part 1 solution: {}", solp1);
}

const _EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
type Number = u64;
#[derive(Debug)]
enum Id {
    Fresh(Number),
    NotFresh(Number),
}
impl Id {
    fn parse(id: Number, rules: &Expr) -> Self {
        if rules.eval(id) {
            Id::Fresh(id)
        } else {
            Id::NotFresh(id)
        }
    }
}

// EXPRESSION SOLVER
#[derive(Debug)]
enum Expr {
    Rng(Number, Number),
    FreshId(Number),
    Or(Box<Expr>, Box<Expr>),
    //And(Box<Expr>, Box<Expr>),
    //Literal(bool),
}

impl Expr {
    fn eval(&self, id: Number) -> bool {
        match self {
            Expr::Rng(a, b) => a <= &id && &id <= b,
            Expr::Or(expr, expr1) => expr.eval(id) || expr1.eval(id),
            Expr::FreshId(x) => *x == id,
            //Expr::And(expr, expr1) => expr.eval(id) && expr1.eval(id),
            //Expr::Literal(truth) => *truth,
        }
    }
    fn parse_range(a: Number, b: Number) -> Result<Expr, &'static str> {
        if a == b {
            Ok(Expr::FreshId(a))
        } else if a < b {
            Ok(Expr::Rng(a, b))
        } else {
            Err("left is not less than right")
        }
    }
}
