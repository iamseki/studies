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

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
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
    let store = Inventory {
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
