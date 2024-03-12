const MAX_MILK: usize = 5;

struct MilkBucket {
    pub milk: usize,
}

impl MilkBucket {
    pub fn new() -> Self {
        MilkBucket { milk: 0 }
    }

    pub fn add_milk(&mut self) -> bool {
        if self.milk >= MAX_MILK {
            false
        } else {
            self.milk += 1;
            true
        }
    }
}

fn main() {
    let mut bucket = MilkBucket::new();

    loop {
        println!("Press `p` to pour milk, `d` to drink, or `q` to quit");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_milk() {
        let mut bucket = MilkBucket::new();
        bucket.milk = MAX_MILK;

        assert!(!bucket.add_milk())
    }
}
