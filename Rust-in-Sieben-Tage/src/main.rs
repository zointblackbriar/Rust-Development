use std::ops::AddAssign;
use std::cmp::PartialOrd;


#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoesize: i32,
}

pub struct Stepper<T> {
    curr:T, 
    step:T, 
    stop:T,
}

impl<T> Stepper<T> {
    pub fn new(start:T, stop:T, step:T) -> Self {
        Stepper {
            curr: start, 
            step: step,
            stop: stop
        }
    }
}

impl<T> Iterator for Stepper<T> 
    where T:AddAssign + Copy + PartialOrd
    {
        type Item = T; 
        fn next(&mut self) -> Option<T> {
            if self.curr >= self.stop {
                return None
            }

            let res = self.curr; 
            self.curr += self.step; 
            Some(res)
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut temp_total = 0;
        for n in Stepper::new(2, 10, 2) {
            temp_total += n; 
        }
        assert_eq!(temp_total, 20); 

        let sl = sum_list(Stepper::new());
    }

}

fn sum_list<I, S>(l:I, mut s:S ) -> S 
    where I: Iterator<Item=S>,
          S: AddAssign,
        {
            for n in l {
                s += n
            }
            s
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
    // let tDataStructure = Room::Kitchen(4); 
    // println!("Hello from the {:?}", tDataStructure);
    // match tDataStructure {
    //     Kitchen(n) => println!("The kitchen has {} rooms", n),
    //     d => println!("{:?}", d),
    // }

    // let bedroomT = Bedroom(Bed{size:50, count:2});
    // println!("regarding bedroomT {:?}", bedroomT); 
    // let v match bedroomT {
    //     Kitchen(n) => n,
    //     d => 0,
    // }
}
