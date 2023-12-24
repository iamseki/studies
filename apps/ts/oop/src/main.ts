enum AnimalType {
  MAMMALS = "Mammals",
  BIRDS = "Birds",
  UNKNOWN = "Unknown"
}

class Animal {
  nickname: string;

  constructor(nickname: string);
  constructor(nickname: string, type?: AnimalType);
  constructor(nickname: string, readonly type?: AnimalType) {
    this.nickname = nickname;
    this.type = type ?? AnimalType.UNKNOWN;
  }

  greeting(hello: string) {
    console.log(`Animal ${this.nickname} of type: ${this.type} says hello: ${hello}`);
  }
}

class Dog extends Animal {
  constructor(nickname: string) {
    super(nickname, AnimalType.MAMMALS);
  }

  greeting() {
    super.greeting("BARKING")
  }
}

class XPTO { };

const main = () => {
  const dog = new Dog("bob");

  dog.greeting();
  console.log(`typeof: ${typeof dog} -- instanceof Animal? ${dog instanceof Animal} -- instanceof Dog? ${dog instanceof Dog} -- instanceof XPTO? ${dog instanceof XPTO}`);

  const anotherDog = dog;
  console.log(`
In typescript multiple variables can reference the same object
So dog Animal object is the same for "dog" and "anotherDog" variables:
dog => ${dog.nickname} --- anotherDog => ${anotherDog.nickname};`);

  anotherDog.nickname = "Scoob";
  console.log(`
Changing a property in one of them will affects both:
dog => ${dog.nickname} --- anotherDog => ${anotherDog.nickname};
The Garbage Collector will automatically reclaims memory when these objects were no longer ferenced.`);
}

main();