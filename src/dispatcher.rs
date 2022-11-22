/// This is an implementation for the Vehicle APIs with respect to the rust_powered_lego crate, 
/// which is an async crate. This is the reason why it must be seperated into different threads.
/// The parallel thread will run tokio runtime and control the car. 
/// The current thread will recieve orders from the feedback engine.
/// See: Velocirustor.png

use rust_powered_lego::lego::consts::{
    EndState,
    Profile,
};

#[allow(unused)]
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use anyhow::{Result, Ok, bail};

use crate::VehicleSteeringAPI;

// To save space and troubles, each argument is stored in its own vector.
// Order is important.
#[derive(Debug)]
pub struct Command {
    pub command_id: u8,
    pub args_u8:    Option<Vec<u8>>,
    pub args_i8:    Option<Vec<i8>>,
    pub args_i32:   Option<Vec<i32>>,
    pub end_state:  Option<EndState>,
}

pub struct VehicleAPI {
    event_tx: UnboundedSender<Command>,
}

impl VehicleAPI {
    pub fn new(event_tx: UnboundedSender<Command>) -> Self {
        Self {
            event_tx,
        }
    }
}

impl VehicleSteeringAPI for VehicleAPI {
    type MotorProfile   = Profile;
    type SteerEndState  = EndState;

    // fn update_steering_motor_profile(profile_id: u8, profile: Self::MotorProfile) -> Result<()> {unimplemented!()}
    // fn steer_by_degree(degrees: i32, power: i8) -> Result<()> {unimplemented!()}
    // fn steer_by_pos(abs_pos: i32, power: i8) -> Result<()> {unimplemented!()}
    
    fn steer_until_stopped(&self, direction: i8, power: i8) -> Result<()> {
        let res = self.event_tx.send(
            Command {
                command_id: VehicleSpecificCommands::SteerUntilStopped as u8, 
                args_u8:    None, 
                args_i8:    Some(vec![direction, power]), 
                args_i32:   None,
                end_state:  None,
            }
        );
        if res.is_err() {
            bail!("Error in steer_until_stopped: {:?}", res)
        }
        Ok(())
    }
    
    fn stop_steer(&self, end_state: Self::SteerEndState) -> Result<()> {
        let res = self.event_tx.send(
            Command {
                command_id: VehicleSpecificCommands::StopSteer as u8, 
                args_u8:    None, 
                args_i8:    None, 
                args_i32:   None,
                end_state:  Some(end_state),
            }
        );
        if res.is_err() {
            bail!("Error in steer_until_stopped: {:?}", res)
        }
        Ok(())
    }
}


#[repr(u8)]
enum VehicleSpecificCommands {
    SteerUntilStopped   = 0x00,
    StopSteer           = 0x01,
}