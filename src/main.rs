use rust_gpiozero::Servo;
use std::{thread::sleep, time::Duration};

fn lockdoor(servo: &mut Servo) {
        servo.min();
        sleep(Duration::from_secs(2));
        println!("Door has been locked");
}

fn unlockdoor(servo: &mut Servo) {
        servo.max();
        sleep(Duration::from_secs(2));
        println!("Door has been unlocked");
    
}

fn main() {
    let mut servo = Servo::new(17);

    lockdoor(&mut servo);
    unlockdoor(&mut servo);

   
}