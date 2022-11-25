use rust_powered_lego::lego::consts::{EndState, TechnicHubPorts};
use winit::{event_loop::{EventLoop, DeviceEventFilter}, event::{Event, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState}, platform::run_return::EventLoopExtRunReturn};

use velocirustor::{dispatcher::{VehicleAPI, LegoVehicleClient}, VehicleMotorAPI, VehicleSteeringAPI};

pub fn control_event_loop(client: &VehicleAPI) {
    let mut event_loop = EventLoop::new();
    event_loop.set_device_event_filter(DeviceEventFilter::Never);

    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;

    event_loop.run_return(move |event, _, control_flow| {
        control_flow.set_wait();
        match &event {
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {if !up_pressed {println!("Got UP!"); up_pressed = true; _ = client.activate_motor_until_stopped(100);}}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {up_pressed = false; _ = client.stop_motor(EndState::FLOAT);}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {if !down_pressed {down_pressed = true; _ = client.activate_motor_until_stopped(-100);}}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {down_pressed = false; _ = client.stop_motor(EndState::FLOAT);}
            
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {if !left_pressed {left_pressed = true; _ = client.steer_until_stopped(-25);}}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {left_pressed = false; _ = client.stop_steer(EndState::FLOAT);}

            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {if !right_pressed {right_pressed = true; _ = client.steer_until_stopped(25);}}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {right_pressed = false; _ = client.stop_steer(EndState::FLOAT);}

            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..}), 
                    ..} => return,          
            _ => (),
        }
    });
}


fn main() {
    let hub_address = "90:84:2b:4e:5b:96";
    let motor_port = TechnicHubPorts::A;
    let steer_motor_port = TechnicHubPorts::B;
    let lvc = LegoVehicleClient::new(hub_address, steer_motor_port, motor_port);
    println!("Before control_event_loop");
    control_event_loop(lvc.get_client());
}