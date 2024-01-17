use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        print!(
            "
giveaway method as an example of closure: user_preference.unwrap_or_else(|| self.most_stocked())
the || self.most_stocked() => it's a closure without any arguments...
"
        );
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        let add_one_closure = |mut x| {
            x += 1;
        };

        for color in &self.shirts {
            match color {
                ShirtColor::Red => add_one_closure(num_red),
                ShirtColor::Blue => add_one_closure(num_blue),
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn give_away_scenario() {
    let mut store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("shirts before defining closure: {:?}", store.shirts);
    let only_borrows = || println!("From closure: {:?}", store.shirts);
    println!("shirts before calling closure: {:?}", store.shirts);
    only_borrows();
    println!("shirts after calling closure: {:?}", store.shirts);

    let mut borrows_mutably = || store.shirts.push(ShirtColor::Blue);
    borrows_mutably();
    println!(
        "shirts after calling borrows_mutably() closure: {:?}",
        store.shirts
    );

    thread::spawn(move || println!("From thread: {:?}", store.shirts))
        .join()
        .unwrap();

    // can't compile cause move ownership to another thread!!!
    // println!("shirts after calling thread.spawn closure: {:?}", store.shirts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_giveaway_user_prefered() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
        };

        let user_preference = Some(ShirtColor::Red);
        let giveaway = store.giveaway(user_preference);
        assert_eq!(giveaway, user_preference.unwrap());
    }

    #[test]
    fn should_giveaway_more_stocked() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
        };

        let user_preference = None;
        let giveaway = store.giveaway(user_preference);
        assert_eq!(giveaway, ShirtColor::Blue);
    }
}
