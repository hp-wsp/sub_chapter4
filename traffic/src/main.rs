
fn main() {
    let r = TrafficLight::Red.shine_time_seconds();
    println!("Red time={}", r);
    let g = TrafficLight::Greed.shine_time_seconds();
    println!("Greed time={}", g);
    let y = TrafficLight::Yellow.shine_time_seconds();
    println!("Yellow time={}", y);

}


trait Shine {

    ///Shine time
    fn shine_time_seconds(&self) -> u32;
}

enum TrafficLight {
    Red,
    Greed,
    Yellow
}

impl Shine for TrafficLight {

    fn shine_time_seconds(&self) -> u32 {
        match self {
            &TrafficLight::Red => 10_u32,
            &TrafficLight::Greed => 15_u32,
            &TrafficLight::Yellow => 5
        }
    }
}