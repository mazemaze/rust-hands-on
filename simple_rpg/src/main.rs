use std::io;

use rand::Rng;

trait Actions {
    fn attack(&self, opponent: BasePlayer) -> i64;
    fn defense(&self) -> i64;
    fn dudge(&self, opponent: BasePlayer) -> bool;
}

#[derive(Clone, Debug)]
struct BasePlayer {
    name: String,
    hp: i64,
    attack: i64,
    defense: i64,
    agility: i64
}

impl BasePlayer {
    fn build_player(name: String, hp: i64, attack: i64, defense: i64, agility: i64) -> BasePlayer {
        BasePlayer {
            name,
            hp,
            attack,
            defense,
            agility
        }
    }
}

impl Actions for BasePlayer {
    fn attack(&self, opponent: BasePlayer) -> i64 {
        return opponent.defense - self.attack;
    }

    fn defense(&self) -> i64 {
        return (self.defense as f32 * 1.5) as i64
    }

    fn dudge(&self, opponent: BasePlayer) -> bool {
        let mut base = 0;
        let mut rng = rand::thread_rng();
        if self.agility > opponent.agility {
            base = 30;
        } else {
            base = 20
        }
        let mut arry: Vec<i32> = vec![0; base];
        for _ in 0..=base {
            let rnd :i32 = rng.gen_range(0..=100);
            arry.push(rnd);
        }
        let rnd: i32 = rng.gen_range(0..=100);
        for num in arry {
            if num == rnd {
                return true;
            }
        }
        return false;
    }
}


fn main() {
    let player1: BasePlayer = BasePlayer {
        name: String::from("player1"),
        hp: 24,
        attack: 12,
        defense: 10,
        agility: 8,
    };

    let opponent: BasePlayer = BasePlayer {
        name: String::from("slime"),
        hp: 10,
        attack: 4,
        defense: 2,
        agility: 3,
    };

    

    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).expect("failed to read from stdin");

        let trimmed = user_input.trim();
        match trimmed.parse::<i64>() {
            Ok(i) => { 
                println!("Your integer input: {}", i);
                break;
             },
            Err(..) => {
                println!("Input a valid number!");
                continue;
            }
        }
    }
    println!("Battle ended")
}
