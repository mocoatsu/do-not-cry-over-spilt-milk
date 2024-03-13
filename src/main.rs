use std::io;

const MAX_MILK: usize = 5;

struct MilkBucket {
    pub milk: usize,
}

impl MilkBucket {
    pub fn new() -> Self {
        MilkBucket { milk: 0 }
    }

    pub fn pour_milk(&mut self) -> bool {
        if self.milk >= MAX_MILK {
            return false;
        } else {
            self.milk += 1;
            return true;
        }
    }

    pub fn dring_milk(&mut self) -> bool {
        if self.milk == 0 {
            false
        } else {
            self.milk -= 1;
            true
        }
    }

    pub fn display_bucket(&mut self) {
        let total_lines = 5;

        for line in (1..=total_lines).rev() {
            if self.milk == line {
                println!("|~~|");
            } else {
                println!("|  |");
            }
        }

        if self.milk == MAX_MILK {
            println!("|__| __");
        } else {
            println!("|__|");
        }
    }
}

fn main() {
    let mut bucket = MilkBucket::new();

    loop {
        bucket.display_bucket();

        if bucket.milk == MAX_MILK {
            println!("Don't cry over split milk!! ༼ ༎ຶ ෴ ༎ຶ༽");
            break;
        }

        println!("Press `p` to pour milk, `d` to drink, or `q` to quit");

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "p" => {
                bucket.pour_milk();
            }
            "d" => {
                if !bucket.dring_milk() {
                    println!("No milk left to drink");
                }
            }
            "q" => break,
            _ => println!("Invalid input. Please try again."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pour_milk() {
        let mut bucket = MilkBucket::new();
        bucket.milk = MAX_MILK - 1;

        assert!(bucket.pour_milk());
        assert!(!bucket.pour_milk())
    }

    #[test]
    fn test_drink_milk() {
        let mut bucket = MilkBucket::new();
        bucket.milk += 1;

        assert!(bucket.dring_milk());
        assert!(!bucket.dring_milk());
    }
}
