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

fn let_the_dogs_out(dog: &Dog) {
    print!("let it dogs out for: {}", dog.nickname)
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


println!("
The let_the_dogs_out(another_dog); without ampersand operator will not compile if we try to read values from the another_dog variable again.
Because the ownership was moved to it's function and after the execution it was released from memory.
So we need to 'borrow' or pass the value as reference with ampersand operator: fn let_the_dogs_out(dog: &Dog), also a.k.a non-owning pointers. 
");
    // let_the_dogs_out(another_dog);
    // another_dog.greeting()

    let_the_dogs_out(&another_dog);
println!("
The let_the_dogs_out function have a Stack Frame that points to another_dog Stack Frame that points to value in the Heap and after it's execution will disappears from Stack, releasing memory.
")
}

