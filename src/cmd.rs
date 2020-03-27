use std::fmt::{self, Formatter};

pub trait Message {
    fn marshal(&self) -> Vec<u8>;
}

impl fmt::Debug for dyn Message {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let v = self.marshal();
        f.write_str(&hex::encode(&v))
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)] // Fine because it gets serialized, we use it implicitly.
pub enum LatencyMode {
    Normal = 0,
    Low = 1,
    High = 2,
}

#[derive(Debug)]
pub struct Command {
    pub opcode: u8,
    pub message: dyn Message,
}

pub struct GetInfoMsg {}

impl Message for GetInfoMsg {
    fn marshal(&self) -> Vec<u8> {
        vec![]
    }
}

pub struct CreateScannerMsg {
    pub scan_id: u32,
}

impl Message for CreateScannerMsg {
    fn marshal(&self) -> Vec<u8> {
        self.scan_id.to_le_bytes().to_vec()
    }
}

pub struct RemoveScannerMsg {
    pub scan_id: u32,
}

impl Message for RemoveScannerMsg {
    fn marshal(&self) -> Vec<u8> {
        self.scan_id.to_le_bytes().to_vec()
    }
}

pub struct CreateConnectionChannelMsg {
    pub conn_id: u32,
    pub bd_addr: [u8; 6],
    pub latency_mode: LatencyMode,
    pub auto_disconnect_time: u16,
}

impl Message for CreateConnectionChannelMsg {
    fn marshal(&self) -> Vec<u8> {
        let lm = self.latency_mode as u8;
        let mut v = self.conn_id.to_le_bytes().to_vec();
        v.append(&mut self.bd_addr.to_vec());
        v.push(lm);
        v.append(&mut self.auto_disconnect_time.to_le_bytes().to_vec());
        v
    }
}

pub struct RemoveConnectionChannelMsg {
    pub conn_id: u32,
}

impl Message for RemoveConnectionChannelMsg {
    fn marshal(&self) -> Vec<u8> {
        self.conn_id.to_le_bytes().to_vec()
    }
}

pub struct ForceDisconnectMsg {
    pub bd_addr: [u8; 6],
}

impl Message for ForceDisconnectMsg {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
}

pub struct ChangeModeParametersMsg {
    pub conn_id: u32,
    pub latency_mode: LatencyMode,
    pub auto_disconnect_time: u16,
}

impl Message for ChangeModeParametersMsg {
    fn marshal(&self) -> Vec<u8> {
        let lm = self.latency_mode as u8;
        let mut v = self.conn_id.to_le_bytes().to_vec();
        v.push(lm);
        v.append(&mut self.auto_disconnect_time.to_le_bytes().to_vec());
        v
    }
}

pub struct PingMsg {
    pub ping_id: u32,
}

impl Message for PingMsg {
    fn marshal(&self) -> Vec<u8> {
        self.ping_id.to_le_bytes().to_vec()
    }
}

pub struct GetButtonInfoMsg {
    pub bd_addr: [u8; 6],
}

impl Message for GetButtonInfoMsg {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
}

pub struct CreateScanWizardMsg {
    pub scan_wizard_id: u32,
}

impl Message for CreateScanWizardMsg {
    fn marshal(&self) -> Vec<u8> {
        self.scan_wizard_id.to_le_bytes().to_vec()
    }
}

pub struct CancelScanWizardMsg {
    pub scan_wizard_id: u32,
}

impl Message for CancelScanWizardMsg {
    fn marshal(&self) -> Vec<u8> {
        self.scan_wizard_id.to_le_bytes().to_vec()
    }
}

pub struct DeleteButtonMsg {
    pub bd_addr: [u8; 6],
}

impl Message for DeleteButtonMsg {
    fn marshal(&self) -> Vec<u8> {
        self.bd_addr.to_vec()
    }
}

pub struct CreateBatteryStatusListenerMsg {
    pub listener_id: u32,
    pub bd_addr: [u8; 6],
}

impl Message for CreateBatteryStatusListenerMsg {
    fn marshal(&self) -> Vec<u8> {
        let mut v = self.listener_id.to_le_bytes().to_vec();
        v.append(&mut self.bd_addr.to_vec());
        v
    }
}

pub struct RemoveBatteryStatusListenerMsg {
    pub listener_id: u32,
}

impl Message for RemoveBatteryStatusListenerMsg {
    fn marshal(&self) -> Vec<u8> {
        self.listener_id.to_le_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_info_msg_marshals_to_empty_vec() {
        let msg = GetInfoMsg {};
        assert_eq!(msg.marshal(), vec![]);
    }

    #[test]
    fn create_scanner_msg_marshal() {
        let msg = CreateScannerMsg {
            scan_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn remove_scanner_msg_marshal() {
        let msg = RemoveScannerMsg {
            scan_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn create_connection_channel_msg_marshal() {
        let msg = CreateConnectionChannelMsg {
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
    fn remove_connection_channel_msg_marshal() {
        let msg = RemoveConnectionChannelMsg {
            conn_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn force_disconnect_msg_marshal() {
        let msg = ForceDisconnectMsg {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn change_mode_parameters_msg_marshal() {
        let msg = ChangeModeParametersMsg {
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
        let msg = PingMsg {
            ping_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn get_button_info_msg_marshal() {
        let msg = GetButtonInfoMsg {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn create_scan_wizard_msg_marshal() {
        let msg = CreateScanWizardMsg {
            scan_wizard_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn cancel_scan_wizard_msg_marshal() {
        let msg = CancelScanWizardMsg {
            scan_wizard_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn delete_button_msg_marshal() {
        let msg = DeleteButtonMsg {
            bd_addr: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        };
        assert_eq!(msg.marshal(), vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn create_battery_status_listener_msg_marshal() {
        let msg = CreateBatteryStatusListenerMsg {
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
    fn remove_battery_status_listener_msg_marshal() {
        let msg = RemoveBatteryStatusListenerMsg {
            listener_id: 0x12345678,
        };
        assert_eq!(msg.marshal(), vec![0x78, 0x56, 0x34, 0x12]);
    }
}
