/* Modules */
pub mod dispatcher;


/* Modules End */



use anyhow::Result;

#[allow(unused_variables)]
pub trait VehicleSteeringAPI {
    type MotorProfile;
    type SteerEndState;
    
    fn update_steering_motor_profile(&self, profile_id: u8, profile: Self::MotorProfile) -> Result<()> {unimplemented!()}
    fn steer_by_degree(&self, degrees: i32, power: i8) -> Result<()> {unimplemented!()}
    fn steer_by_pos(&self, abs_pos: i32) -> Result<()> {unimplemented!()}
    fn steer_until_stopped(&self, power: i8) -> Result<()> {unimplemented!()}
    fn stop_steer(&self, end_state: Self::SteerEndState) -> Result<()> {unimplemented!()}
}

#[allow(unused_variables)]
pub trait VehicleMotorAPI {
    type MotorProfile;
    type MotorEndState;

    fn update_motor_profile(&self, profile_id: u8, profile: Self::MotorProfile) -> Result<()> {unimplemented!()}
    fn activate_motor_until_stopped(&self, power: i8) -> Result<()> {unimplemented!()}
    fn stop_motor(&self, end_state: Self::MotorEndState) -> Result<()> {unimplemented!()}
}
