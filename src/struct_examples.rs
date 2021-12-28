mod light;
use light::Light;
use light::traffic_light::TrafficLight;
use light::house_light::HouseLight;

pub fn lights_example() {
    let mut light = TrafficLight::new();
    println!("{}", light);
    println!("{:?}", light);

    println!("State: {:?}", light.get_state());
    light.turn_green();
    println!("State: {:?}", light.get_state());
    light.turn_yellow();
    println!("State: {:?}", light.get_state());
    light.turn_red();
    println!("State: {:?}", light.get_state());

    let traffic_light = TrafficLight::new();
    let house_light = HouseLight::new();

    print_light_state(&traffic_light);
    print_light_state(&house_light);
}

fn print_light_state(light: &impl Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}
