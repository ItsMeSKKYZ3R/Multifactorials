pub struct Multifactorials;

type FactorialResult = f64;

impl Multifactorials {
    /// ### n is the number before the factorial -> n!

    pub fn simple(mut n: f64) -> FactorialResult {
        let mut res = vec![];

        if n < 3.0 {
            if n == 0.0 {
                res.push(1.0);
            } else {
                res.push(n);
            }
        } else {
            while n - 1.0 > 0.0 {
                res.push(n);

                n = n - 1.0;
            }
        }

        res.iter().product::<f64>()
    }


    /// ### n is the number before the factorial -> n!
    /// ### number_of_factorials if the number of factorials -> n with number_of_factorials numbers of !

    pub fn complex(mut n: f64, number_of_factorials: i32) -> FactorialResult {
        let mut res = vec![];

        if n < 3.0 {
            if n == 0.0 {
                res.push(1.0);
            } else {
                res.push(n);
            }
        } else {
            while n - number_of_factorials as f64 > 0.0 {
                res.push(n);

                n -= number_of_factorials as f64;
            }
        }

        res.iter().product::<f64>()
    }
}