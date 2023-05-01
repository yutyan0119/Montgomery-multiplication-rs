pub struct MontgomeryReducer {
    n: u64,
    n_prime: u64,
    r: u64,
    r_x: u64,
}

impl MontgomeryReducer {
    pub fn new(n: u64) -> Self {
        let (r, r_x) = Self::search_r(n);
        let n_prime = Self::search_n_prime(r, n);
        // println!(
        //     "n = {}, r = {}, n_prime = {}, n*n_prime mod r = {}",
        //     n,
        //     r,
        //     n_prime,
        //     n * n_prime % r
        // );
        Self { n, n_prime, r, r_x }
    }
    ///n<rとなる2の冪乗のrを探索
    fn search_r(n: u64) -> (u64, u64) {
        let mut r: u64 = 1; //rの値
        let mut x: u64 = 0; //rが2の何乗かを表す
        while r <= n {
            r *= 2;
            x += 1;
        }
        (r, x)
    }

    ///n*n_prime = -1 mod rとなるn_primeを探索
    fn search_n_prime(mut r: u64, n: u64) -> u64 {
        let mut n_prime: u64 = 0; //返却する値
        let mut tmp: u64 = 0; //一時的なr*n_primeの値を注目すべき桁が最下位になるようにしたもの
        let mut i: u64 = 1; //tmpの下位1bitが0だったときにn_primeに足すべき値
        while r > 1 {
            if tmp % 2 == 0 {
                tmp += n;
                n_prime += i;
            }
            tmp /= 2;
            r /= 2;
            i *= 2;
        }
        n_prime
    }

    ///モンゴメリリダクションの実装
    /// t <= (a + (a * n_prime mod r) * n) / r
    fn montgomery_reduction(&self, a: u64) -> u64 {
        let mut t: u64 = a * self.n_prime;
        t &= self.r - 1; //a*n_prime mod r
        t *= self.n; //a*n_prime mod r * n
        t += a; // a + a*n_prime mod r * n
        t >>= self.r_x; // (a + a*n_prime mod r * n) / r
        if t >= self.n {
            t -= self.n;
        }
        // println!("t = {}", t);
        t
    }

    ///モンゴメリ乗算の実装
    pub fn mod_mul(&self, a: u64, b: u64) -> u64 {
        let r_2: u64 = (self.r % self.n) * (self.r % self.n) % self.n; //r^2 mod n
        let aa = self.montgomery_reduction(a * r_2);
        let bb = self.montgomery_reduction(b * r_2);
        let ab = self.montgomery_reduction(aa * bb);
        let ans = self.montgomery_reduction(ab);
        ans
        // println!("r^2 mod n = {}", r_2);
        // println!("a*b = {}", a * b);
        //こっちだとでかい値のときにオーバーフローかなにかでおかしくなる
        // self.montgomery_reduction(self.montgomery_reduction(a * b) * r_2);
    }
}

///実行時引数からa,b,Nを受け取って a*b mod N をモンゴメリ乗算を使って計算
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a: u64 = args[1].parse().unwrap();
    let b: u64 = args[2].parse().unwrap();
    let n: u64 = args[3].parse().unwrap();
    let reducer = MontgomeryReducer::new(n);
    println!("{}", reducer.mod_mul(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_mod_mul() {
        let test_cases = vec![
            (2, 5, 7, 3),
            (3, 4, 5, 2),
            (123, 456, 789, 69),
            (11, 18, 169, 29),
            (98765, 43210, 31415, 2145),
            (17, 19, 29, 4),
        ];

        for (a, b, n, expected) in test_cases {
            let reducer: MontgomeryReducer = MontgomeryReducer::new(n);
            assert_eq!(reducer.mod_mul(a, b), expected);
        }

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        for _ in 0..10 {
            let a: u64 = rng.gen_range(1..1_000_000);
            let b: u64 = rng.gen_range(1..1_000_000);
            let mut n: u64 = rng.gen_range(1..1_000_000);
            if n % 2 == 0 {
                n += 1;
            }
            let reducer: MontgomeryReducer = MontgomeryReducer::new(n as u64);
            let expected: u64 = (a * b) % n;
            println!("{} * {} mod {} = {}", a, b, n, expected);
            assert_eq!(reducer.mod_mul(a, b), expected);
        }
    }
}
