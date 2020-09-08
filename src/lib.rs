use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct Chunk<'a> {
    chars: &'a str,
    are_digits: bool,
}

impl<'a> Chunk<'a> {
    pub fn new(chars: &'a str, are_digits: bool) -> Self {
        Chunk { chars, are_digits }
    }
}

pub fn natural_chunk(a: &str) -> Chunk {
    let is_digit = match a.as_bytes().first() {
        None => return Chunk::new(a, false),
        Some(c) => c.is_ascii_digit(),
    };
    for (i, c ) in a.bytes().enumerate() {
        if c.is_ascii_digit() != is_digit {
            return Chunk::new(&a[..i], is_digit);
        }
    }
    Chunk::new(a, is_digit)
}

pub fn natural_cmp(mut a: &str, mut b: &str) -> Ordering {
    while !a.is_empty() && !b.is_empty() {
        let chunk_a = natural_chunk(a);
        let chunk_b = natural_chunk(b);

        if chunk_a.are_digits && chunk_b.are_digits {
            match chunk_a.chars.len().cmp(&chunk_b.chars.len()) {
                Ordering::Equal => (),
                v @ _ => return v,
            }
        }

        match chunk_a.chars.cmp(chunk_b.chars) {
            Ordering::Equal => (),
            v @ _ => return v,
        }

        a = &a[chunk_a.chars.len()..];
        b = &b[chunk_b.chars.len()..];
    }

    return a.len().cmp(&b.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_chunk_works() {
        assert_eq!(Chunk::new("", false), natural_chunk(""));
        assert_eq!(Chunk::new("A", false), natural_chunk("A"));
        assert_eq!(Chunk::new("1", true), natural_chunk("1"));
        assert_eq!(Chunk::new("123", true), natural_chunk("123"));
        assert_eq!(Chunk::new("test", false), natural_chunk("test"));
        assert_eq!(Chunk::new("1", true), natural_chunk("1Test"));
        assert_eq!(Chunk::new("123", true), natural_chunk("123Test"));
        assert_eq!(Chunk::new("Test", false), natural_chunk("Test1"));
        assert_eq!(Chunk::new("Test", false), natural_chunk("Test123"));
    }

    #[test]
    fn natural_cmp_works() {
        assert_eq!(Ordering::Equal, natural_cmp("", ""));

        assert_eq!(Ordering::Equal, natural_cmp("1", "1"));
        assert_eq!(Ordering::Equal, natural_cmp("a", "a"));
        assert_eq!(Ordering::Less, natural_cmp("1", "2"));
        assert_eq!(Ordering::Greater, natural_cmp("2", "1"));

        assert_eq!(Ordering::Less, natural_cmp("1", "12"));
        assert_eq!(Ordering::Less, natural_cmp("2", "12"));
        assert_eq!(Ordering::Greater, natural_cmp("24", "12"));

        assert_eq!(Ordering::Less, natural_cmp("test1", "test12"));
        assert_eq!(Ordering::Less, natural_cmp("test2", "test12"));
        assert_eq!(Ordering::Greater, natural_cmp("test24", "test12"));
    }
}