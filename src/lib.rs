use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{Ordering, min};

#[derive(PartialEq, Eq, Debug)]
struct Chunk<'a> {
    chars: &'a str,
    are_digits: bool,
}

impl<'a> Chunk<'a> {
    pub fn new(chars: &'a str, are_digits: bool) -> Self {
        Chunk { chars, are_digits }
    }
}

fn natural_chunk(a: &str) -> Chunk<'_> {
    let is_digit = match a.as_bytes().first() {
        None => return Chunk::new(a, false),
        Some(c) => c.is_ascii_digit(),
    };
    for (i, c) in a.bytes().enumerate() {
        if c.is_ascii_digit() != is_digit {
            return Chunk::new(&a[..i], is_digit);
        }
    }
    Chunk::new(a, is_digit)
}

pub fn natural_cmp_old(mut a: &str, mut b: &str) -> Ordering {
    while !a.is_empty() && !b.is_empty() {
        let chunk_a = natural_chunk(a);
        let chunk_b = natural_chunk(b);

        if chunk_a.are_digits && chunk_b.are_digits {
            match chunk_a.chars.len().cmp(&chunk_b.chars.len()) {
                Ordering::Equal => (),
                v => return v,
            }
        }

        match chunk_a.chars.cmp(chunk_b.chars) {
            Ordering::Equal => (),
            v => return v,
        }

        a = &a[chunk_a.chars.len()..];
        b = &b[chunk_b.chars.len()..];
    }

    a.len().cmp(&b.len())
}

pub fn natural_cmp(left: &str, right: &str) -> Ordering {
    let l = min(left.len(), right.len());

    // Slice to the loop iteration range to enable bound check
    // elimination in the compiler
    let lhs = &left.as_bytes()[..l];
    let rhs = &right.as_bytes()[..l];

    let mut i = 0;
    let mut lc;
    let mut rc;

    // skip the equal characters
    let res = loop {
        if i == l {
            // in this case the longer sequence wins (even if both ended in a number)
            return left.len().cmp(&right.len());
        }
        lc = lhs[i];
        rc = rhs[i];
        let res = lc.cmp(&rc);
        if !matches!(res, Equal) {
            break res;
        }
        i += 1;
    };

    // special case: last char could have been a number
    let l_digit = lc.is_ascii_digit();
    let r_digit = rc.is_ascii_digit();
    if !l_digit || !r_digit {
        if 0 < i {
            if r_digit {
                if rhs[i - 1].is_ascii_digit() {
                    return Less;
                }
            } else if l_digit && lhs[i - 1].is_ascii_digit() {
                return Greater;
            }
        }
        return res;
    }

    // if the numbers turn out to have the same length, res is the result != Equal
    // current situation:
    // ...1
    // ...2
    //     ^
    loop {
        i += 1;
        if i == l {
            // in this case we don't know yet who wins
            break;
        }

        let lc = lhs[i];
        let rc = rhs[i];

        let l_digit = lc.is_ascii_digit();
        let r_digit = rc.is_ascii_digit();

        if !l_digit && !r_digit {
            // no digit => same length
            return res;
        } else if !l_digit {
            return Less;
        } else if !r_digit {
            return Greater;
        }
    }

    // current situation: (one sequence ended)
    // ...1...?
    // ...2...?
    // find the longer number (that sequence wins)
    // or number_ordering_same_length since we already found a distinguishing character
    if left.len() != l {
        if left.as_bytes()[l].is_ascii_digit() {
            return Greater;
        }
    } else if right.len() != l && right.as_bytes()[l].is_ascii_digit() {
        return Less;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_cmp_works() {
        assert_eq!(Ordering::Equal, natural_cmp("", ""));
        assert_eq!(Ordering::Equal, natural_cmp("1", "1"));
        assert_eq!(Ordering::Equal, natural_cmp("a", "a"));

        assert_eq!(Ordering::Less, natural_cmp("ab", "ac"));
        assert_eq!(Ordering::Greater, natural_cmp("ac", "ab"));

        assert_eq!(Ordering::Less, natural_cmp("test1", "test12"));
        assert_eq!(Ordering::Greater, natural_cmp("test12", "test1"));

        assert_eq!(Ordering::Less, natural_cmp("1", "2"));
        assert_eq!(Ordering::Greater, natural_cmp("2", "1"));

        assert_eq!(Ordering::Less, natural_cmp("12", "13"));
        assert_eq!(Ordering::Greater, natural_cmp("13", "12"));

        assert_eq!(Ordering::Less, natural_cmp("1a", "12"));
        assert_eq!(Ordering::Greater, natural_cmp("12", "1a"));

        assert_eq!(Ordering::Greater, natural_cmp("aa", "a2"));
        assert_eq!(Ordering::Less, natural_cmp("a2", "aa"));

        assert_eq!(Ordering::Greater, natural_cmp("a", "1"));
        assert_eq!(Ordering::Less, natural_cmp("1", "a"));

        assert_eq!(Ordering::Less, natural_cmp("2", "12"));
        assert_eq!(Ordering::Greater, natural_cmp("12", "2"));

        assert_eq!(Ordering::Less, natural_cmp("12a", "22b"));
        assert_eq!(Ordering::Greater, natural_cmp("22b", "12a"));

        assert_eq!(Ordering::Less, natural_cmp("12a", "221"));
        assert_eq!(Ordering::Greater, natural_cmp("221", "12a"));
    }

    use rand::Rng;
    use rand::distr::{Distribution, Uniform};
    use rand_chacha::ChaCha20Rng;
    use rand_chacha::rand_core::SeedableRng;

    const CHARACTERS: &str = "ABCDEF123456";

    fn generate_string<R: Rng + ?Sized>(rng: &mut R) -> String {
        let count_distribution: Uniform<u32> = Uniform::new(1, 10).unwrap();
        let character_distribution: Uniform<u32> =
            Uniform::new(1, CHARACTERS.len() as u32).unwrap();
        let mut res = String::new();
        let count = count_distribution.sample(rng);
        res.reserve(count as usize);
        for _ in 0..count {
            res.push(char::from(
                CHARACTERS.as_bytes()[character_distribution.sample(rng) as usize],
            ));
        }
        res
    }

    #[test]
    fn natural_cmp_same_as_old_fuzz() {
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        for _ in 0..10000 {
            let a = generate_string(&mut rng);
            let b = generate_string(&mut rng);
            assert_eq!(
                natural_cmp_old(a.as_str(), b.as_str()),
                natural_cmp(a.as_str(), b.as_str())
            );
        }
    }
}
