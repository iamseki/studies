trait Animal {
    fn greeting(&self);
}

#[allow(dead_code)]
enum AnimalType {
    Mammals,
    Birds,
    Unknown
}

struct Dog {
    nickname: String,
    animal_type: AnimalType
}

impl Dog {
    pub fn new(nickname: String, animal_type: AnimalType) -> Dog {
        Dog { nickname, animal_type }
    }

    fn get_animal_type(&self) -> String {
        match self.animal_type {
            AnimalType::Birds => "Birds".to_string(),
            AnimalType::Mammals => "Mammals".to_string(),
            AnimalType::Unknown => "Unknown".to_string()
        }
    }
}

impl Animal for Dog {
    fn greeting(&self) {
        println!("Animal {} of type: {} says hello: BARKING", self.nickname, self.get_animal_type())
    }
}

fn main() {
    let dog = Dog::new(String::from("bob"), AnimalType::Mammals);
    dog.greeting();

    let another_dog = dog;
    another_dog.greeting();
    println!("
'dog' ownership was transfered to another_dog variable, so trying to access anything in 'dog' will cause a compile error.
There's no garbage collection at all and the memory safety atm it's handled at compile time by rust borrower checker.
");
}

