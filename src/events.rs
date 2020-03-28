// TODO: Remove this, all this code will be used by clients eventually.
#![allow(dead_code)]

use std::io::{Error, ErrorKind};

use crate::enums::*;

pub enum Event {
    AdvertisementPacket(AdvertisementPacket),
    CreateConnectionChannelResponse(CreateConnectionChannelResponse),
    ConnectionStatusChanged(ConnectionStatusChanged),
    ConnectionChannelRemoved(ConnectionChannelRemoved),
    ButtonEvent(ButtonEvent),
    NewVerifiedButton(NewVerifiedButton),
    GetInfoResponse(GetInfoResponse),
    NoSpaceForNewConnection(NoSpaceForNewConnection),
    GotSpaceForNewConnection(GotSpaceForNewConnection),
    BluetoothControllerStateChange(BluetoothControllerStateChange),
    PingResponse(PingResponse),
    GetButtonInfoResponse(GetButtonInfoResponse),
    ScanWizardFoundPrivateButton(ScanWizardFoundPrivateButton),
    ScanWizardFoundPublicButton(ScanWizardFoundPublicButton),
    ScanWizardButtonConnected(ScanWizardButtonConnected),
    ScanWizardCompleted(ScanWizardCompleted),
    ButtonDeleted(ButtonDeleted),
    BatteryStatus(BatteryStatus),
}

// For each scanner the client has created, this packet will be sent for each bluetooth
// advertisement packet arriving that comes from a Flic button. Usually the Flic button sends out
// many advertisement packets, with higher frequency if it was lately pressed.
// Opcode: 0
pub struct AdvertisementPacket {
    scan_id: u32, // The scan id corresponding to the scanner which this advertisement packet belongs to.
    bd_addr: [u8; 6], // The bluetooth address of this Flic button. Use it to establish a connection chnanel.
    name_length: u8,  // The length in bytes of the name following.
    name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
    rssi: i8, // Signal strength in dBm, between -126 and 20, where -127 is weakest and 20 is strongest. -127 means not available.
    is_private: bool, // The Flic button is currently in private mode and won't accept connections from unbonded clients. Hold it down for 7 seconds while not attempting to connect to it to make it public. First then you may connect.
    already_verified: bool, // If the server has the bonding key for this Flic button, this value is true. That means you should be able to connect to it.
    already_connected_to_this_device: bool, // This Flic 2 button is already connected to this device.
    already_connected_to_other_device: bool, // This Flic 2 button is already connected to another device.
}

pub fn unmarshal_advertisement_packet(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// This event will always be sent when a CmdCreateConnectionChannel is received, containing the
// status of the request.
// Opcode: 1
pub struct CreateConnectionChannelResponse {
    conn_id: u32,                        // Connection channel identifier.
    error: CreateConnectionChannelError, // Whether the request succeeded or not.
    connection_status: ConnectionStatus, // The current connection status to this button. This might be a non-disconnected status if there are already other active connection channels to this button.
}

pub fn unmarshal_create_connection_channel_response(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// This event is sent when the connection status is changed.
// Opcode: 2
pub struct ConnectionStatusChanged {
    conn_id: u32,                        // Connection channel identifier.
    connection_status: ConnectionStatus, // New connection status.
    disconnect_reason: DisconnectReason, // If the connection status is Disconnected, this contains the reason. Otherwise this parameter is considered invalid.
}

pub fn unmarshal_connection_status_changed(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// This event is sent when a connection channel is removed. After this event is sent from the
// server, it will no longer send events corresponding to this connection channel. From this point,
// the conn_id can now be reused when creating new connection channels. Note: If you got an
// EvtCreateConnectionChannelResponse with an error different than NoError, the connection channel
// have never been considered created, and this event will thus never be sent afterwards.
// Opcode: 3
pub struct ConnectionChannelRemoved {
    conn_id: u32,                  // Connection channel identifier.
    removed_reason: RemovedReason, // Reason for this connection channel being removed.
}

pub fn unmarshal_connection_channel_removed(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// There are four types of button events. For each type of event, there is a different set of
// possible ClickTypes. Normally one application would handle one type of events and discard the
// others, depending on how many different triggers you would like the Flic button to be used for.
// The following event types are defined:
// Possible ClickTypes are ButtonUp and ButtonDown. Used to simply know when the button was pressed or released.
// Possible ClickTypes are ButtonClick and ButtonHold. Used if you want to distinguish between click and hold.
// Possible ClickTypes are ButtonSingleClick and ButtonDoubleClick. Used if you want to distinguish between a single click and a double click.
// Possible ClickTypes are ButtonSingleClick, ButtonDoubleClick and ButtonHold. Used if you want to distinguish between a single click, a double click and a hold.
// Opcode: 4, 5, 6 or 7 for the different types of event, in the same order as above.
pub struct ButtonEvent {
    conn_id: u32,          // Connection channel identifier.
    click_type: ClickType, // The click type. For each opcode, there are different possible values.
    was_queued: bool, // If this button event happened during the button was disconnected or not.
    time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

pub fn unmarshal_button_event(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// This is sent to all clients when a button has been successfully verified that was not verified
// before (for the current bluetooth controller bluetooth address). Note: The
// EvtConnectionStatusChanged with connection_status = Ready will be sent just before this event.
// Opcode: 8
pub struct NewVerifiedButton {
    bd_addr: [u8; 6], // The bluetooth address for the verified Flic button.
}

pub fn unmarshal_new_verified_button(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// This is sent as a response to a CmdGetInfo.
// Opcode: 9
pub struct GetInfoResponse {
    bluetooth_controller_state: BluetoothControllerState, // Current state of the HCI connection to the bluetooth controller.
    my_bd_addr: [u8; 6], // Current bluetooth address / identity of this device.
    my_bd_addr_type: BdAddrType, // Current bluetooth address type of this device.
    max_pending_connections: u8, // The max number of Flic buttons that can be monitored at the same time, regardless of having an established connection or not.
    max_concurrently_connected_buttons: i16, // The max number of Flic buttons that can have an established bluetooth connection at the same time. If this amount is reached, no other pending connection will succeed until another one has disconnected. This value will be -1 until the value becomes known. It becomes known first when the maximum number of connections is currently established and there is an attempt to establish yet another connection. Not all bluetooth controllers handle this correctly; some simply hides the fact that the maximum is reached and further connections won't succeed successfully, until a previously established connection is disconnected. Note: For some bluetooth controllers we have tested we have already hardcoded the correct value and this parameter will thus not be -1 but the correct one.
    current_pending_connections: u8, // Current number of Flic buttons that are monitored by the server, among all clients.
    currently_no_space_for_new_connection: bool, // The maximum number of concurrently connected buttons has been reached.
    nb_verified_buttons: u16, // Number of verified buttons for this my_bd_addr/my_bd_addr_type pair.
    bd_addr_of_verified_buttons: Vec<[u8; 6]>, // An array of all the verified buttons.
}

pub fn unmarshal_get_info_response(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent when the maximum number of connections has been reached (immediately after the
// EvtConnectionStatusChanged event). If the maximum number of connections is unknown, it is sent
// when the maximum number of connections are reached and an attempt is made to connect yet another
// button.
// Opcode: 10
pub struct NoSpaceForNewConnection {
    max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

pub fn unmarshal_no_space_for_new_connection(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent when the maximum number of concurrent connections was reached but a button has now
// disconnected, making room for one new connection. Now a new connection attempt will
// automatically be made to devices having a connection channel open but has not yet established a
// connection.
// Opcode: 11
pub struct GotSpaceForNewConnection {
    max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

pub fn unmarshal_got_space_for_new_connection(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// See enum BluetoothControllerStateChange. If the bluetooth controller is detached, the scanners
// and connection channels set up by the user will maintain their state (but obviously no
// advertisement packet / connection state change / button events will be received). When the state
// is changed to Attached, internally all pending connections and scanners will be recreated as
// they were before the bluetooth controller was detached. Note: before sending the Detached state
// to the client, the server will first send EvtConnectionStatusChanged for each connected button
// with connection_status = Disconnected. Note: If the bluetooth controller sends a hardware error
// event, the state will transition directly from Attached to Resetting and if it was able to
// reset, back to Attached.
// Opcode: 12
pub struct BluetoothControllerStateChange {
    state: BluetoothControllerState, // The new state.
}

pub fn unmarshal_bluetooth_controller_state_change(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent in response to a CmdPing
// Opcode: 13
pub struct PingResponse {
    ping_id: u32, // Same ping id as sent in the CmdPing.
}

pub fn unmarshal_ping_response(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent in return to a CmdGetButtonInfo. If the button was not verified, all parameters except
// bd_addr will contain zero-bytes.
// Opcode: 14
pub struct GetButtonInfoResponse {
    bd_addr: [u8; 6],         // The bluetooth device address of the request.
    uuid: [u8; 16], // The uuid of the button. Each button has a unique 128-bit identifier.
    color_length: u8, // The length in bytes of the color following.
    color: [u8; 16], // The first color_length bytes of this array contain the UTF-8 encoding of the color. The other bytes will be zeros. Currently the following strings are defined: black, white, turquoise, green and yellow but more colors may be added later, so don't expect these are the only possible values.
    serial_number_length: u8, // The length in bytes of the serial number following.
    serial_number: [u8; 16], // The serial number of the button, in UTF-8 encoding. Only the first serial_number_length bytes are used. The other bytes will be zeros.
}

pub fn unmarshal_get_button_info_response(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent once if a previously not verified private button is found during the scan. If this is
// received, tell the user to hold the button down for 7 seconds.
// Opcode: 15
pub struct ScanWizardFoundPrivateButton {
    scan_wizard_id: u32, // Scan wizard id.
}

pub fn unmarshal_scan_wizard_found_private_button(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent once if a previously not verified public button is found during scan. Now the scan wizard
// stops scanning internally and instead initiates a connection to this button.
// Opcode: 16
pub struct ScanWizardFoundPublicButton {
    scan_wizard_id: u32, // Scan wizard id.
    bd_addr: [u8; 6],    // The bluetooth address of the Flic button that was found.
    name_length: u8,     // The length in bytes of the name following.
    name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
}

pub fn unmarshal_scan_wizard_found_public_button(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent when the found button connects for the first time. Now the verification and pairing process
// will begin.
// Opcode: 17
pub struct ScanWizardButtonConnected {
    scan_wizard_id: u32, // Scan wizard id.
}

pub fn unmarshal_scan_wizard_button_connected(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent when the scan wizard has completed. See ScanWizardResult documentation for more
// information.
// Opcode: 18
pub struct ScanWizardCompleted {
    scan_wizard_id: u32,      // Scan wizard id.
    result: ScanWizardResult, // Result of the scan wizard.
}

pub fn unmarshal_scan_wizard_completed(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent as a response to CmdDeleteButton or when a verified button has been deleted from the
// database.
// Opcode: 19
pub struct ButtonDeleted {
    bd_addr: [u8; 6],             // The bluetooth device address of the deleted button.
    deleted_by_this_client: bool, // Whether or not the client that initiated the deletion was the current client.
}

pub fn unmarshal_button_deleted(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}

// Sent to a battery status listener created by CmdCreateBatteryStatusListener in order to indicate
// the current battery status.
// Opcode: 20
pub struct BatteryStatus {
    listener_id: u32,       // Listener identifier.
    battery_percentage: i8, // A value between 0 and 100 that indicates the current battery status. The value can also be -1 if unknown.
    timestamp: i64, // UNIX timestamp (time in seconds since 1970-01-01T00:00:00Z, excluding leap seconds).
}

pub fn unmarshal_battery_status(data: &Vec<u8>) -> std::io::Result<Event> {
    Err(Error::new(ErrorKind::NotFound, "not implemented"))
}
