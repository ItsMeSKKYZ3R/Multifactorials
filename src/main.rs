struct Multifactorials;

impl Multifactorials {
    fn from(mut n: i128, factorials: i32) -> i128 {
        let mut res = vec![];

        if n < 3 {
            if n == 0 {
                res.push(1)
            } else {
                res.push(n)
            }
        } else {
            while n - factorials as i128 > 0 {
                res.push(n);

                n = n - factorials as i128;

                println!("{}", n)
            }
        }

        let value = res.iter().product::<i128>();

        value
    }
}

fn main() {
    let multifactorial = Multifactorials::from(18, 1);

    println!("18! = {:?}", multifactorial)
}
