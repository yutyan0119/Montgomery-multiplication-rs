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
fn montgomery_reduction(a: u64, n: u64, n_prime: u64, r: u64, r_x: u64) -> u64 {
    let mut t: u64 = a * n_prime;
    t &= r - 1; //a*n_prime mod r
    t *= n; //a*n_prime mod r * n
    t += a; // a + a*n_prime mod r * n
    t >>= r_x; // (a + a*n_prime mod r * n) / r
    if t >= n {
        t -= n;
    }
    t
}

///モンゴメリ乗算の実装
fn mod_mul(a: u64, b: u64, n: u64) -> u64 {
    let (r, x) = search_r(n);
    println!("r = {}, x = {}", r, x);
    let n_prime: u64 = search_n_prime(r, n);
    println!("n = {}, n_prime = {}, n*n_prime mod r = {}",n, n_prime, n*n_prime % r);
    let r_2: u64 = (r % n) * (r % n) % n; //r^2 mod n
    let result = montgomery_reduction(
        montgomery_reduction(a * b, n, n_prime, r, x) * r_2,
        n,
        n_prime,
        r,
        x,
    );
    result
}

///実行時引数からa,b,Nを受け取って a*b mod N をモンゴメリ乗算を使って計算
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a: u64 = args[1].parse().unwrap();
    let b: u64 = args[2].parse().unwrap();
    let n: u64 = args[3].parse().unwrap();
    println!("{}", mod_mul(a, b, n));
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!(mod_mul(a, b, n), expected);
        }
    }
}