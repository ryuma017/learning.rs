fn main() {
    let mut car = Car::new();
    car.display_info();
    car.move_to(3, 3);
    car.display_info();
    car.move_to(10, 8);
    car.move_to(5, 5);
}

struct Car {
    fuel: u32,
    position: (i32, i32)
}

impl Car {
    fn new() -> Car {
        Car { fuel: 100, position: (0, 0)}
    }

    fn display_info(&self) {
        println!(
            "fuel: {}, position: {:?}",
            self.fuel, self.position
        )
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let distance = (self.position.0 - x).pow(2) + (self.position.1 - y).pow(2);
        if self.fuel >= distance as u32 {
            self.fuel -= distance as u32;
            self.position = (x, y);

            println!("moved to {:?}", self.position);
        } else {
            println!("unable to move. not enough fuel")
        }
    }
}