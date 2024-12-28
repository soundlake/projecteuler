use std::collections::HashSet;

const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn get_digits() -> HashSet<char, BuildHasher> {
    let mut set = HashSet::with_hasher(BuildHasher);
    for c in DIGITS.iter() {
        set.insert(*c);
    }
    set
}

pub fn filter(
    org: &HashSet<char, BuildHasher>,
    needle: &char
) -> HashSet<char, BuildHasher> {
    let mut new = org.clone();
    new.remove(needle);
    new
}

pub fn examine(
    multiplicand: &[char],
    multiplier: &[char],
    rest: &HashSet<char, BuildHasher>
) -> Option<u32> {
    let multiplicand = String::from_iter(multiplicand).parse::<u32>().unwrap();
    let multiplier = String::from_iter(multiplier).parse::<u32>().unwrap();
    let mut rest = rest.clone();

    let product = multiplicand * multiplier;
    if product < 1000 || 10000 <= product {
        return None;
    }

    for c in product.to_string().chars() {
        if !rest.remove(&c) {
            break;
        }
    }

    if rest.is_empty() {
        println!("{multiplicand}x{multiplier}={product}");
        return Some(product);
    }

    None
}

pub struct Hasher {
    state: u8,
}

impl std::hash::Hasher for Hasher {
    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() > 0 {
            self.state = bytes[0];
        } else {
            self.state = 0;
        }
    }

    fn finish(&self) -> u64 {
        self.state.into()
    }
}

#[derive(Clone)]
pub struct BuildHasher;

impl std::hash::BuildHasher for BuildHasher {
    type Hasher = Hasher;
    fn build_hasher(&self) -> Hasher {
        Hasher { state: 0 }
    }
}
