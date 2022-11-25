
use winit::{event_loop::{EventLoop, DeviceEventFilter}, event::{Event, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState}, platform::run_return::EventLoopExtRunReturn};



pub fn control_event_loop() {
    let mut event_loop = EventLoop::new();
    event_loop.set_device_event_filter(DeviceEventFilter::Never);

    event_loop.run_return(move |event, _, control_flow| {
        control_flow.set_wait();
        match &event {
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {println!("Got UP Pressed!")}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {println!("Got UP Released!")}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {println!("Got Key!")}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {println!("Got Key!")}
            
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {println!("Got Key!")}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {println!("Got Key!")}

            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Pressed, 
                        ..}), 
                    ..} => {println!("Got Key!")}
            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Released, 
                        ..}), 
                    ..} => {println!("Got Key!")}

            Event::DeviceEvent {event: DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..}), 
                    ..} => return,          
            _ => (),
        }
    });
}


fn main() {
    control_event_loop();
}