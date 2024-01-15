// Import the Debug trait, which allows us to use the {:?} formatter.
use std::fmt::Debug;

// Define an enum for the various types of events that can occur in an elevator system.
#[derive(Debug)]
enum ElevatorEvent {
    CallButtonPressed { floor: u32 },
    ElevatorArrived { floor: u32 },
    EmergencyButtonPressed,
    MaintenanceModeEnabled,
    MaintenanceModeDisabled,
}

// Implement a function to create a new call button press event.
fn new_call_button_pressed_event(floor: u32) -> ElevatorEvent {
    ElevatorEvent::CallButtonPressed { floor }
}

// Implement a function to create a new elevator arrived event.
fn new_elevator_arrived_event(floor: u32) -> ElevatorEvent {
    ElevatorEvent::ElevatorArrived { floor }
}

fn main() {
    // Create an event where the call button is pressed on the 3rd floor.
    let call_event = new_call_button_pressed_event(3);

    // Create an event where the elevator arrives at the 5th floor.
    let arrival_event = new_elevator_arrived_event(5);

    // Print the events to the console.
    println!("Call Event: {:?}", call_event);
    println!("Arrival Event: {:?}", arrival_event);
}
