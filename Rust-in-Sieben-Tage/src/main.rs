
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoesize: i32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32), 
    Bedroom(Bed),
    Room,
    Lounge
}

#[derive(Debug)]
pub struct Bed {
    size: i32, // size of the Bed
    count: u32, // count of the Bed
}

impl User {
    fn sample_string(&self ) -> String{
        return format!("{} - {} - {}cm - shoe:{}", self.name, self.age, self.height, self.shoesize);
    }

    fn grow_behavior(&mut self, height:i32) {
        self.height += height; 
    }

    fn terminate(self) {
        println!("Terminate {}", self.sample_string());
    }
}

fn main() {
    println!("Hello, main method!");
    let mut user = User {
        name: "Joe".to_string(),
        age:30,
        height:180,
        shoesize: 45,
    }; 

    println!("User is: {:?}", user);
    user.grow_behavior(20);
    println!("User is: {:?}", user);
    user.terminate();
    // user.terminate(); //you cannot terminate two times
    use self::Room::*; //you don't need to write as below: Room::Kitchen(n)
    let tDataStructure = Room::Kitchen(4); 
    println!("Hello from the {:?}", tDataStructure);
    match tDataStructure {
        Kitchen(n) => println!("The kitchen has {} rooms", n),
        d => println!("{:?}", d),
    }

    let bedroomT = Bedroom(Bed{size:50, count:2});
    println!("regarding bedroomT {:?}", bedroomT); 
    let v match bedroomT {
        Kitchen(n) => n,
        d => 0,
    }
}
