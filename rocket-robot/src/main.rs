#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rppal::gpio::Gpio;
use std::error::Error;
use std::time::Duration;
use std::thread;

const MOTOR_PIN_1: u8 = 13;
const MOTOR_PIN_2: u8 = 21;
const MOTOR_PIN_3: u8 = 17;
const MOTOR_PIN_4: u8 = 27;

fn motor_forward() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_high();
    pin2.set_low();
    pin3.set_high();
    pin4.set_low();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

fn motor_backward() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_low();
    pin2.set_high();
    pin3.set_low();
    pin4.set_high();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

fn motor_left() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_low();
    pin2.set_high();
    pin3.set_high();
    pin4.set_low();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

fn motor_right() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_high();
    pin2.set_low();
    pin3.set_low();
    pin4.set_high();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
    
}

#[get("/forward")]
fn forward() -> Template {
    motor_forward();
    Template::render("index", context! {})
}

#[get("/backward")]
fn backward() -> Template {
    motor_backward();
    Template::render("index", context! {})
}

#[get("/left")]
fn left() -> Template {
    motor_left();
    Template::render("index", context! {})
}

#[get("/right")]
fn right() -> Template {
    motor_right();
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/forward", routes![forward])
        .mount("/backward", routes![backward])
        .mount("/left", routes![left])
        .mount("/right", routes![right])
        .attach(Template::fairing())
}
