use std::fmt::{self, Formatter};

use crate::enums::LatencyMode;

pub trait Command {
    fn marshal(&self) -> Vec<u8>;
    fn opcode(&self) -> u8;
}

impl fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v = hex::encode(self.marshal());
        f.write_fmt(format_args!(
            "Opcode {}, Body {}, Length {}",
            self.opcode(),
            v,
            v.chars().count()
        ))
    }
}

// This command is used to retrieve current state about the server. After this command is sent, an
// EvtGetInfoResponse is sent back.
pub struct GetInfo {}

impl Command for GetInfo {
    fn marshal(&self) -> Vec<u8> {
        vec![]
    }
    fn opcode(&self) -> u8 {
        0
    }
}

// Creates a scanner with the given scan_id. For each advertisement packet received from a Flic
// button by the server, an EvtAdvertisementPacket will be sent with the given scan_id until it is
// removed using CmdRemoveScanner. If there is already an active scanner with this scan_id, this
// does nothing.
pub struct CreateScanner {
    pub scan_id: u32,
}

impl Command for CreateScanner {
    fn marshal(&self) -> Vec<u8> {
        self.scan_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        1
    }
}

// Removes the scanner with the given scan_id. Once this is received by the server, it will no
// longer send out EvtAdvertisementPackets with this scan_id.
pub struct RemoveScanner {
    pub scan_id: u32,
}

impl Command for RemoveScanner {
    fn marshal(&self) -> Vec<u8> {
        self.scan_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        2
    }
}

// Creates a connection channel for a Flic button with the given bluetooth address. You assign a
// unique conn_id for this connection channel that will later be used in commands and events to
// refer to this connection channel. After this command is received by the server, an
// EvtCreateConnectionChannelResponse is sent. If there already exists a connection channel with
// this conn_id, this does nothing.
pub struct CreateConnectionChannel {
    pub conn_id: u32,
    pub bd_addr: [u8; 6],
    pub latency_mode: LatencyMode,
    pub auto_disconnect_time: u16,
}

impl Command for CreateConnectionChannel {
    fn marshal(&self) -> Vec<u8> {
        let lm = self.latency_mode as u8;
        let mut v = self.conn_id.to_le_bytes().to_vec();
        v.append(&mut self.bd_addr.to_vec());
        v.push(lm);
        v.append(&mut self.auto_disconnect_time.to_le_bytes().to_vec());
        v
    }
    fn opcode(&self) -> u8 {
        3
    }
}

// Removes a connection channel previously created with CmdCreateConnectionChannel. After this is
// received by the server, this connection channel is removed and no further events will be sent
// for this channel. If there are no other connection channels active to this Flic button among any
// client, the physical bluetooth connection is disconnected.
pub struct RemoveConnectionChannel {
    pub conn_id: u32,
}

impl Command for RemoveConnectionChannel {
    fn marshal(&self) -> Vec<u8> {
        self.conn_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        4
    }
}

// Removes all connection channels among all clients for the specified Flic button bluetooth
// address.
pub struct ForceDisconnect {
    pub bd_addr: [u8; 6],
}

impl Command for ForceDisconnect {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
    fn opcode(&self) -> u8 {
        5
    }
}

// Changes the accepted latency for this connection channel and the auto disconnect time. The
// latency mode will be applied immediately but the auto disconnect time will be applied the next
// time tme Flic is getting connected.
pub struct ChangeModeParameters {
    pub conn_id: u32,
    pub latency_mode: LatencyMode,
    pub auto_disconnect_time: u16,
}

impl Command for ChangeModeParameters {
    fn marshal(&self) -> Vec<u8> {
        let lm = self.latency_mode as u8;
        let mut v = self.conn_id.to_le_bytes().to_vec();
        v.push(lm);
        v.append(&mut self.auto_disconnect_time.to_le_bytes().to_vec());
        v
    }
    fn opcode(&self) -> u8 {
        6
    }
}

// If you for various reasons would like to ping the server, send this command. An EvtPingResponse
// will be sent back in return with the same ping_id.
pub struct Ping {
    pub ping_id: u32,
}

impl Command for Ping {
    fn marshal(&self) -> Vec<u8> {
        self.ping_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        7
    }
}

// Get info about a verified button. An EvtGetButtonInfoResponse will be sent back immediately in
// return with the bd_addr field set to the same value as in the request.
pub struct GetButtonInfo {
    pub bd_addr: [u8; 6],
}

impl Command for GetButtonInfo {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
    fn opcode(&self) -> u8 {
        8
    }
}

// Starts a scan wizard. If there already exists a scan wizard with the same id, this does nothing.
pub struct CreateScanWizard {
    pub scan_wizard_id: u32,
}

impl Command for CreateScanWizard {
    fn marshal(&self) -> Vec<u8> {
        self.scan_wizard_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        9
    }
}

// Cancels a scan wizard that was previously started. If there exists a scan wizard with this id,
// it is cancelled and an EvtScanWizardCompleted is sent with the reason set to
// WizardCancelledByUser.
pub struct CancelScanWizard {
    pub scan_wizard_id: u32,
}

impl Command for CancelScanWizard {
    fn marshal(&self) -> Vec<u8> {
        self.scan_wizard_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        10
    }
}

// Deletes a button. If the button exists in the list of verified buttons, all connection channels
// will be removed for all clients for this button. After that the EvtButtonDeleted event will be
// triggered for all clients. If the button does not exist in the list of verified buttons, the
// request has no effects but an EvtButtonDeleted will be triggered anyway for this client with the
// same address as in the request.
pub struct DeleteButton {
    pub bd_addr: [u8; 6],
}

impl Command for DeleteButton {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
    fn opcode(&self) -> u8 {
        11
    }
}

// Creates a battery status listener for a specific button. If the given listener_id already exists
// for this client, this does nothing. Once created, an EvtBatteryStatus will always immediately be
// sent with the current battery status. Every time the battery status later updates, an
// EvtBatteryStatus will be sent. This will usually happen not more often than every three hours.
// Note that by just having a battery status listener doesn't mean flicd will automatically connect
// to a Flic button in order to get updates. At least one client needs a connection channel for the
// particular button to be able to get new updates.
pub struct CreateBatteryStatusListener {
    pub listener_id: u32,
    pub bd_addr: [u8; 6],
}

impl Command for CreateBatteryStatusListener {
    fn marshal(&self) -> Vec<u8> {
        let mut v = self.listener_id.to_le_bytes().to_vec();
        v.append(&mut self.bd_addr.to_vec());
        v
    }
    fn opcode(&self) -> u8 {
        12
    }
}

// Removes a battery status listener.
pub struct RemoveBatteryStatusListener {
    pub listener_id: u32,
}

impl Command for RemoveBatteryStatusListener {
    fn marshal(&self) -> Vec<u8> {
        self.listener_id.to_le_bytes().to_vec()
    }
    fn opcode(&self) -> u8 {
        13
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_info_marshals_to_empty_vec() {
        let msg = GetInfo {};
        assert_eq!(msg.marshal(), vec![]);
    }

    #[test]
    fn create_scanner_marshal() {
        let msg = CreateScanner {
            scan_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn remove_scanner_marshal() {
        let msg = RemoveScanner {
            scan_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn create_connection_channel_marshal() {
        let msg = CreateConnectionChannel {
            conn_id: 0x12345678,
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
            latency_mode: LatencyMode::Normal,
            auto_disconnect_time: 0x4455,
        };
        assert_eq!(
            msg.marshal(),
            vec![
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
                0x00, // latency_mode
                0x55, 0x44, // auto_disconnect_time
            ]
        );
    }

    #[test]
    fn remove_connection_channel_marshal() {
        let msg = RemoveConnectionChannel {
            conn_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn force_disconnect_marshal() {
        let msg = ForceDisconnect {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn change_mode_parameters_marshal() {
        let msg = ChangeModeParameters {
            conn_id: 0x12345678,
            latency_mode: LatencyMode::Low,
            auto_disconnect_time: 0x4455,
        };
        assert_eq!(
            msg.marshal(),
            vec![
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x01, // latency_mode
                0x55, 0x44, // auto_disconnect_time
            ]
        );
    }

    #[test]
    fn cmd_ping_marshal() {
        let msg = Ping {
            ping_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn get_button_info_marshal() {
        let msg = GetButtonInfo {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn create_scan_wizard_marshal() {
        let msg = CreateScanWizard {
            scan_wizard_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn cancel_scan_wizard_marshal() {
        let msg = CancelScanWizard {
            scan_wizard_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn delete_button_marshal() {
        let msg = DeleteButton {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn create_battery_status_listener_marshal() {
        let msg = CreateBatteryStatusListener {
            listener_id: 0x12345678,
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(
            msg.marshal(),
            vec![
                0x78, 0x56, 0x34, 0x12, // listener_id
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            ]
        );
    }

    #[test]
    fn remove_battery_status_listener_marshal() {
        let msg = RemoveBatteryStatusListener {
            listener_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }
}
