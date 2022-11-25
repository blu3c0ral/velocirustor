/// This is an implementation for the Vehicle APIs with respect to the rust_powered_lego crate, 
/// which is an async crate. This is the reason why it must be seperated into different threads.
/// The parallel thread (LegoVehicleDispatcher) will run tokio runtime and control the car. 
/// The current thread (VehicleAPI) will recieve orders from the feedback engine.
/// Both combined in LegoVehicleClient
/// See: Velocirustor.png

use std::thread;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use btleplug::api::BDAddr;

use rust_powered_lego::lego::consts::{
    EndState,
    Profile, 
    TechnicHubPorts,
};

use rust_powered_lego::connection_manager::ConnectionManager;
use rust_powered_lego::lego::message_parameters::StartupAndCompletionInfo;
use rust_powered_lego::{
    HubType, 
    MotorType,
    hub::Hub
};

use tokio::runtime::Builder;
use tokio::sync::mpsc::{
    UnboundedSender, 
    UnboundedReceiver,
    self
};

use anyhow::{
    Result, 
    Ok, 
    bail
};

use crate::{
    VehicleSteeringAPI, 
    VehicleMotorAPI
};


#[derive(Debug)]
#[repr(u8)]
pub enum VehicleSpecificCommands {
    SteerByPosition             = 0x00,
    SteerUntilStopped           = 0x01,
    StopSteer                   = 0x02,

    ActivateMotorUntilStopped   = 0x03,
    StopMotor                   = 0x04,
}

// To save space and troubles, each argument is stored in its own vector.
// Order is important.
#[derive(Debug)]
pub struct Command {
    pub command_id: VehicleSpecificCommands,
    pub args_u8:    Option<Vec<u8>>,
    pub args_i8:    Option<Vec<i8>>,
    pub args_i32:   Option<Vec<i32>>,
    pub end_state:  Option<EndState>,
}


/// VehicleAPI is implemantation of the Vehicle<Motor, Steering>API trait to control
/// a vehicle. The vehicle API is not alone - it needs a client that is a vehicle-specific.
/// Here a lego motors are being used.
/// See LegoVehicleDispatcher below for the implemantation for the lego motors specifically.
pub struct VehicleAPI {
    event_tx: UnboundedSender<Command>,
}

impl VehicleAPI {
    pub fn new(event_tx: UnboundedSender<Command>) -> Self {
        Self {
            event_tx,
        }
    }

    pub fn print_event_tx(&self) {
        println!("event_tx: {:?}", self.event_tx);
    }

    fn send_command(
        &self,
        command_id:     VehicleSpecificCommands, 
        args_u8:        Option<Vec<u8>>,
        args_i8:        Option<Vec<i8>>,
        args_i32:       Option<Vec<i32>>,
        end_state:      Option<EndState>,
        command_name:   &str,
    ) -> Result<()> {
        let res = self.event_tx.send(
            Command {
                command_id, 
                args_u8, 
                args_i8, 
                args_i32,
                end_state,
            }
        );
        if res.is_err() {
            bail!("Error in {}: {:?}", command_name, res)
        }
        Ok(())
    }
}

impl VehicleSteeringAPI for VehicleAPI {
    type MotorProfile   = Profile;
    type SteerEndState  = EndState;

    // fn update_steering_motor_profile(profile_id: u8, profile: Self::MotorProfile) -> Result<()> {unimplemented!()}
    // fn steer_by_degree(degrees: i32, power: i8) -> Result<()> {unimplemented!()}
    
    fn steer_by_pos(&self, abs_pos: i32) -> Result<()> {
        self.send_command(
            VehicleSpecificCommands::SteerByPosition, 
            None, 
            None, 
            Some(vec![abs_pos]), 
            None, 
            "steer_by_pos",
        )
    }
    
    fn steer_until_stopped(&self, power: i8) -> Result<()> {
        self.send_command(
            VehicleSpecificCommands::SteerUntilStopped, 
            None, 
            Some(vec![power]),
            None, 
            None, 
            "steer_until_stopped",
        )
    }
    
    fn stop_steer(&self, end_state: Self::SteerEndState) -> Result<()> {
        self.send_command(
            VehicleSpecificCommands::StopSteer, 
            None, 
            None, 
            None, 
            Some(end_state), 
            "stop_steer",
        )
    }
}


impl VehicleMotorAPI for VehicleAPI {
    type MotorProfile   = Profile;
    type MotorEndState  = EndState;

    // fn update_motor_profile(&self, profile_id: u8, profile: Self::MotorProfile) -> Result<()> {unimplemented!()}
    
    fn activate_motor_until_stopped(&self, power: i8) -> Result<()> {
        self.send_command(
            VehicleSpecificCommands::ActivateMotorUntilStopped, 
            None, 
            Some(vec![power]), 
            None, 
            None, 
            "activate_motor_until_stopped",
        )
    }

    fn stop_motor(&self, end_state: Self::MotorEndState) -> Result<()> {
        self.send_command(
            VehicleSpecificCommands::StopMotor, 
            None, 
            None, 
            None, 
            Some(end_state), 
            "stop_motor",
        )
    }
}




/// LegoVehicleDispatcher is capable of holding a connection to lego hub and controling the motors connected to it.
/// This is the "other" thread that is running tokio rt.
/// run_dispatcher is the command event loop that is at the other end of VehicleAPI::send_command

pub async fn run_dispatcher(mut event_rx: UnboundedReceiver<Command>, hub: impl HubType, steer_motor_port: TechnicHubPorts, motor_port: TechnicHubPorts) {
    let motor = hub.get_motor(motor_port as u8).await.unwrap();
    let steer_motor = hub.get_motor(steer_motor_port as u8).await.unwrap();

    loop {
        let maybe_command = event_rx.try_recv();

        if maybe_command.is_err() {continue;}

        let command = maybe_command.unwrap();
        
        match command.command_id {
            VehicleSpecificCommands::SteerByPosition => {
                goto_pos(&steer_motor, command.args_i32.unwrap()[0]).await;
            },
            VehicleSpecificCommands::SteerUntilStopped => {
                activate_motor(&steer_motor, command.args_i8.unwrap()[0]).await;
            },
            VehicleSpecificCommands::StopSteer => {
                stop_motor(&steer_motor, command.end_state.unwrap()).await;
            },
            VehicleSpecificCommands::ActivateMotorUntilStopped => {
                activate_motor(&motor, command.args_i8.unwrap()[0]).await;
            },
            VehicleSpecificCommands::StopMotor => {
                stop_motor(&motor, command.end_state.unwrap()).await;
            },
        }      
    }
}


async fn activate_motor<T>(motor: &T, power: i8)
where
    T: MotorType 
{
    _ = motor.start_power(power, StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction).await;
}

async fn stop_motor<T>(motor: &T, end_state: EndState)
where
    T: MotorType,
{
    _ = motor.stop_motor(
        end_state, 
        Profile::AccDec, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await;
}

async fn goto_pos<T>(motor: &T, abs_pos: i32)
where
    T: MotorType,
{
    _ = motor.set_abs_position(
        abs_pos, 
        StartupAndCompletionInfo::ExecuteImmediatelyAndNoAction
    ).await;
}




pub struct LegoVehicleClient {
    client: VehicleAPI,
}

impl LegoVehicleClient {
    pub fn new(hub_address: &str, steer_motor_port: TechnicHubPorts, motor_port: TechnicHubPorts) -> Self {

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let hub_mac = hub_address.to_owned();

        thread::spawn(move || {
            let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

            runtime.block_on(async {
                let hub = get_hub(&hub_mac).await.unwrap();
                run_dispatcher(event_rx, hub, steer_motor_port, motor_port).await;
                sleep(Duration::from_secs(2));
            });
        });

        Self {
            client: VehicleAPI::new(event_tx),
        }
    }

    pub fn get_client(&self) -> &VehicleAPI {
        &self.client
    }

}

async fn get_hub(address:  &str) -> Result<Hub> {
    
    // Converting the MAC string to btleplug::api::BDAddr type
    let address = BDAddr::from_str(address)?;

    // The ConnectionManager connects stuff - so ask it for the hub...
    let cm = ConnectionManager::new();

    // It is possible to use the name of the hub or its MAC address. That's why it's Option<>
    // Here, only address is implemented
    let hub = cm.get_hub(None, Some(address), 5).await?;

    // Great! Let's get on with this...
    Ok(hub)
}