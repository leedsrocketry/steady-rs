pub mod steady;
pub use steady::packets::fluctus_packet::FlightStatus;
pub use steady::packets::fluctus_packet::FluctusPacket;
pub use steady::packets::fluctus_packet::RollingMessage;

pub use steady::commands::command::Command;
pub use steady::commands::start::StartCommand;
pub use steady::responses::response::SteadyReply;
pub use steady::transport::serial::SerialTransport;

// All test cases taken from the documentation
// http://silicdyne.net//resources/docs/fluctus_sgs_interface_protocol_1_7b.pdf
#[cfg(test)]
mod tests {
    use crate::steady::commands::start::Band;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_provided_packet() {
        let packet: &'static str = "FB3E00070100BEDD01000000000000006C00AA89109CFF00650000000000000000000E53000000|Grssi-65/Gsnr6";
        let parsed_packet = FluctusPacket::from_str(packet);
        assert!(
            parsed_packet.is_ok(),
            "Failed to parse packet: {:?}",
            parsed_packet.err()
        );
        let packet = parsed_packet.unwrap();

        // Check provided values
        assert!(
            packet.uid == 62,
            "UID does not match documentation value (62)"
        );
        assert!(
            packet.fw == 263,
            "FW does not match documentation value (263)"
        );
        assert!(
            packet.altitude == 0,
            "Altitude does not match documentation value (0)"
        );
        assert!(
            packet.status == FlightStatus::Idle,
            "Status does not match documentation value (0) (IDLE)"
        );
        // I think this is the correct value, documentation is wrong
        assert!(packet.time_mpu == 122302, "Expected time_mpu to be 122302");
    }

    #[test]
    fn test_create_start_command() {
        let band: Band = Band::from_str("US").unwrap();
        let chan = 1;
        let device = "Fluctus".to_string();

        let start_command = StartCommand::new(band, chan, device.clone()).unwrap();
        assert_eq!(start_command.band, band);
        assert_eq!(start_command.chan, chan);
        assert_eq!(start_command.device, device);
    }

    #[test]
    fn test_read_start_command() {
        let command_str = "start003Fluctus\n";
        let start_command = StartCommand::from_string(command_str).unwrap();
        assert_eq!(start_command.band, Band::US);
        assert_eq!(start_command.chan, 3);
        assert_eq!(start_command.device, "Fluctus");
    }

    #[test]
    fn test_read_invalid_start_command() {
        let command_str = "start032Fluctus\n";
        let start_command = StartCommand::from_string(command_str);
        assert!(
            start_command.is_err(),
            "Expected error for invalid command format"
        );
    }

    #[test]
    fn test_steady_response() {
        let response_str = "Gstartok123";
        let steady_reply = SteadyReply::from_str(response_str);
        assert!(
            steady_reply.is_ok(),
            "Failed to parse response: {:?}",
            steady_reply.err()
        );
        let reply: SteadyReply = steady_reply.unwrap();
        assert_eq!(
            reply.firmware_id, 123,
            "Firmware ID does not match expected value"
        );
    }
}
