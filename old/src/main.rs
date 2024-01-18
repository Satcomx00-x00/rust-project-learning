// Import the Debug trait, which allows us to use the {:?} formatter.
use std::fmt::Debug;

#[derive(Debug)]
enum ElevatorEvent {
    CallButtonPressed { floor: u32 },
    ElevatorArrived { floor: u32 },
    EmergencyButtonPressed,
    MaintenanceModeEnabled,
    MaintenanceModeDisabled,
}

fn new_call_button_pressed_event(floor: u32) -> ElevatorEvent {
    ElevatorEvent::CallButtonPressed { floor }
}

fn new_elevator_arrived_event(floor: u32) -> ElevatorEvent {
    ElevatorEvent::ElevatorArrived { floor }
}

fn main() {
    let call_event = new_call_button_pressed_event(3);
    let arrival_event = new_elevator_arrived_event(5);
    println!("Call Event: {:?}", call_event);
    println!("Arrival Event: {:?}", arrival_event);
}
