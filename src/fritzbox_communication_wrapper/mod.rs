use fritzbox_communication::fritzbox_communication::structs::general::FritzboxCommunication;
use std::process;

/// Wrapper for the fritzbox_communication object API
pub struct FritzboxCommunicationWrapper {
    pub fritzbox_communication: FritzboxCommunication,
}

/// Implementation for the Fritzbox Communication wrapper struct.
impl FritzboxCommunicationWrapper {
    
    /// Get new FritboxCommunicationWrapper object.
    pub async fn new() -> FritzboxCommunicationWrapper  {
        // Get a new FritzboxCommunication object that is going to get wrapper afterwards.
        let fritzbox_communication_result = FritzboxCommunication::new().await;
        let fritzbox_communication;
        match fritzbox_communication_result {
            Ok(val) => {
                fritzbox_communication = val;
            }
            Err(_e) => {
                log::error!("Failed to create the fritzbox_communication object, error: {}", _e);
                process::exit(1);
            },
        }

        // Return the FritzboxCommunicationWrapper object.
        return FritzboxCommunicationWrapper{fritzbox_communication: fritzbox_communication};    
    }

    /// Get internet speed.
    pub async fn get_speed(&mut self) -> String {
        let root_result = self.fritzbox_communication.get_data().await;
        match root_result {
            Ok(val) => {
                return "Up: ".to_owned() + &val.data.internet.up + " and Down: " + &val.data.internet.down;
            },
            Err(err) => {
                return err;
            }
        }
    }
}