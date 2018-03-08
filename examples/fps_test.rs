extern crate sawblade;
use sawblade::game::fps::FPSRegulator;

fn main() {
    let mut fps_reg = FPSRegulator::new(30);
    loop {
        fps_reg.start();
        println!("Hello!");
        fps_reg.wait();
    }
}