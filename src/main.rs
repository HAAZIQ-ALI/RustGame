use rand::Rng;
use std::fmt;

impl Gems {
  fn from_numbers(num: u8) -> Option<Gems> {
    match num {
        1 => Some(Gems::Diamond),
        2 => Some(Gems::Sapphire),
        3 => Some(Gems::Ruby),
        4 => Some(Gems::Emerald),
        5 => Some(Gems::Onyx),
        6 => Some(Gems::Jade),
        _ => None, 
    }
  }
}

impl fmt::Display for Gems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gems::Diamond => write!(f, "Diamond 💎"),
            Gems::Sapphire => write!(f, "Sappire 🔹"),
            Gems::Ruby => write!(f, "Ruby 🔺"),
            Gems::Emerald => write!(f, "Emerald 🟩"),
            Gems::Onyx => write!(f, "Onyx 🟣"),
            Gems::Jade => write!(f, "Jade 🔸"),
        }
    }
}

enum Gems {
    Diamond = 1,
    Sapphire = 2,
    Ruby,
    Emerald,
    Onyx,
    Jade,
}
fn main() {
    let gems = [
        (Gems::Diamond, 100.00),
        (Gems::Sapphire, 50.00),
        (Gems::Ruby, 35.00),
        (Gems::Emerald, 20.00),
        (Gems::Onyx, 10.00),
        (Gems::Jade, 5.00),
    ];

    for gem in gems {
        println!("Gem : {} , Price : {} ", gem.0, gem.1);
    }

    let mut map = [[0; 5]; 5];

    for i in map {
        println!("{i:?}");
    }

    for _i in 0..map.len() {
        let mut rng = rand::rng();
        let rand1 = rng.random_range(0..5);
        let rand2 = rng.random_range(1..=6);
        
        map[rand1][rand1] = rand2;
    }

     for i in map {
        println!("{i:?}");
    }
}
