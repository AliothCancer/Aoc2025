#![allow(unused)]

pub(crate) fn solve_p1() {
    let expr = RangeInc::new(3, 5);

    // a diventa di tipo: And<RangeInc, RangeInc>
    let a = expr.and(RangeInc::new(10, 20));

    // b diventa di tipo: Or<And<RangeInc, RangeInc>, RangeInc>
    let b = a.or(RangeInc::new(12, 23));

    let mut is_fresh = true;
    for i in 0..10 {
        let k = b.and(RangeInc::new(i, i + 10));
    }
    let is_fresh = b.eval(4);

    println!("{}", is_fresh);
}

struct FreshId(u32);
struct Id(u32);

struct RangeInc {
    lower: u32,
    upper: u32,
}

impl RangeInc {
    fn new(lower: u32, upper: u32) -> Self {
        Self { lower, upper }
    }
}

// 1. Le struct ora tengono traccia sia del ramo Sinistro (L) che Destro (R)
struct And<L, R>(L, R);
struct Or<L, R>(L, R);

// 2. Il trait non ha generici. È solo un "marcatore" di comportamento.
trait Expression: Sized {
    // 3. I metodi accettano un generico R, a patto che R implementi Expression.
    fn and<R: Expression>(self, rhs: R) -> And<Self, R> {
        And(self, rhs)
    }

    fn or<R: Expression>(self, rhs: R) -> Or<Self, R> {
        Or(self, rhs)
    }
}

// 4. Le implementazioni ora sono semplicissime
impl Expression for RangeInc {}

// Anche i nodi combinati sono a loro volta delle Expression!
impl<L: Expression, R: Expression> Expression for And<L, R> {}
impl<L: Expression, R: Expression> Expression for Or<L, R> {}

// Il trait che deferisce l'esecuzione
trait Defer {
    fn eval(&self, target: u32) -> bool;
}
impl Defer for RangeInc {
    fn eval(&self, target: u32) -> bool {
        target >= self.lower && target <= self.upper
    }
}

impl<L, R> Defer for And<L, R>
where
    L: Defer,
    R: Defer,
{
    fn eval(&self, target: u32) -> bool {
        self.0.eval(target) && self.1.eval(target)
    }
}

impl<L, R> Defer for Or<L, R>
where
    L: Defer,
    R: Defer,
{
    fn eval(&self, target: u32) -> bool {
        self.0.eval(target) || self.1.eval(target)
    }
}
