// TODO: Remove this, all this code will be used by clients eventually.
#![allow(unused_variables, dead_code, unused_assignments)]

use crate::enums::*;
use std::error;
use std::fmt;
use std::string;

pub type Result<T> = std::result::Result<T, UnmarshalError>;

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

#[derive(Debug, Clone)]
pub enum UnmarshalError {
    BadLength(usize, usize),
    BadString(string::FromUtf8Error),
    BadEnum(u8, &'static str),
    BadOpcode(u8),
    Unimplemented, // TODO: Implement things and remove this
}

impl fmt::Display for UnmarshalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnmarshalError::BadLength(got_len, want_len) => {
                write!(f, "body length {}, expected {}", got_len, want_len)
            }
            UnmarshalError::BadString(err) => write!(f, "failed to load string: {}", err),
            UnmarshalError::BadEnum(field, val) => {
                write!(f, "enum value {} is invalid for enum {}", field, val)
            }
            UnmarshalError::BadOpcode(opcode) => write!(f, "unknown opcode {}", opcode),
            UnmarshalError::Unimplemented => write!(f, "sorry, this hasn't been implemented yet!"),
        }
    }
}

impl error::Error for UnmarshalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Underlying cause isn't tracked. Note: I mostly just copied this bit from
        // https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
        None
    }
}

fn check_sz(data: &[u8], want_len: usize) -> Result<()> {
    if data.len() == want_len {
        return Ok(());
    }

    Err(UnmarshalError::BadLength(data.len(), want_len))
}

fn load_bool(data: &[u8], o: usize) -> bool {
    data[o] == 1
}

fn load_u32(data: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([data[o], data[o + 1], data[o + 2], data[o + 3]])
}

fn load_i64(data: &[u8], o: usize) -> i64 {
    i64::from_le_bytes([
        data[o],
        data[o + 1],
        data[o + 2],
        data[o + 3],
        data[o + 4],
        data[o + 5],
        data[o + 6],
        data[o + 7],
    ])
}

fn load_string(data: &[u8], o: usize, sz: usize) -> Result<String> {
    let res = String::from_utf8(data[o..=(o + sz)].to_vec());
    match res {
        Ok(s) => Ok(s),
        Err(err) => Err(UnmarshalError::BadString(err)),
    }
}

fn load_bd_addr(data: &[u8], o: usize) -> [u8; 6] {
    [
        data[o],
        data[o + 1],
        data[o + 2],
        data[o + 3],
        data[o + 4],
        data[o + 5],
    ]
}

fn load_uuid(data: &[u8], o: usize) -> [u8; 16] {
    [
        data[o],
        data[o + 1],
        data[o + 2],
        data[o + 3],
        data[o + 4],
        data[o + 5],
        data[o + 6],
        data[o + 7],
        data[o + 8],
        data[o + 9],
        data[o + 10],
        data[o + 11],
        data[o + 12],
        data[o + 13],
        data[o + 14],
        data[o + 15],
    ]
}

// For each scanner the client has created, this packet will be sent for each bluetooth
// advertisement packet arriving that comes from a Flic button. Usually the Flic button sends out
// many advertisement packets, with higher frequency if it was lately pressed.
// Opcode: 0
pub struct AdvertisementPacket {
    scan_id: u32, // The scan id corresponding to the scanner which this advertisement packet belongs to.
    bd_addr: [u8; 6], // The bluetooth address of this Flic button. Use it to establish a connection chnanel.

    // Next two fields aren't copied directly
    // name_length: u8,  // The length in bytes of the name following.
    // name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
    name: String,
    rssi: i8, // Signal strength in dBm, between -126 and 20, where -127 is weakest and 20 is strongest. -127 means not available.
    is_private: bool, // The Flic button is currently in private mode and won't accept connections from unbonded clients. Hold it down for 7 seconds while not attempting to connect to it to make it public. First then you may connect.
    already_verified: bool, // If the server has the bonding key for this Flic button, this value is true. That means you should be able to connect to it.
    already_connected_to_this_device: bool, // This Flic 2 button is already connected to this device.
    already_connected_to_other_device: bool, // This Flic 2 button is already connected to another device.
}

pub fn unmarshal_advertisement_packet(data: &[u8]) -> Result<Event> {
    check_sz(data, 32)?;

    let name_len = data[10] as usize;

    let evt = AdvertisementPacket {
        scan_id: load_u32(data, 0),
        bd_addr: load_bd_addr(data, 4),
        // data[10] is the length of the name field.
        name: load_string(data, 11, name_len)?,
        rssi: data[27] as i8,
        is_private: load_bool(data, 28),
        already_verified: load_bool(data, 29),
        already_connected_to_this_device: load_bool(data, 30),
        already_connected_to_other_device: load_bool(data, 31),
    };

    Ok(Event::AdvertisementPacket(evt))
}

// This event will always be sent when a CmdCreateConnectionChannel is received, containing the
// status of the request.
// Opcode: 1
pub struct CreateConnectionChannelResponse {
    conn_id: u32,                        // Connection channel identifier.
    error: CreateConnectionChannelError, // Whether the request succeeded or not.
    connection_status: ConnectionStatus, // The current connection status to this button. This might be a non-disconnected status if there are already other active connection channels to this button.
}

pub fn unmarshal_create_connection_channel_response(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let error = match num::FromPrimitive::from_u8(data[4]) {
        Some(err) => err,
        None => {
            return Err(UnmarshalError::BadEnum(
                data[4],
                "CreateConnectionChannelError",
            ))
        }
    };

    let connection_status = match num::FromPrimitive::from_u8(data[5]) {
        Some(conn_status) => conn_status,
        None => return Err(UnmarshalError::BadEnum(data[5], "ConnectionStatus")),
    };

    let evt = CreateConnectionChannelResponse {
        conn_id: load_u32(data, 0),
        error,
        connection_status,
    };

    Ok(Event::CreateConnectionChannelResponse(evt))
}

// This event is sent when the connection status is changed.
// Opcode: 2
pub struct ConnectionStatusChanged {
    conn_id: u32,                        // Connection channel identifier.
    connection_status: ConnectionStatus, // New connection status.
    disconnect_reason: DisconnectReason, // If the connection status is Disconnected, this contains the reason. Otherwise this parameter is considered invalid.
}

pub fn unmarshal_connection_status_changed(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let connection_status = match num::FromPrimitive::from_u8(data[4]) {
        Some(conn_status) => conn_status,
        None => return Err(UnmarshalError::BadEnum(data[4], "ConnectionStatus")),
    };

    let disconnect_reason = match num::FromPrimitive::from_u8(data[5]) {
        Some(disconn_reason) => disconn_reason,
        None => return Err(UnmarshalError::BadEnum(data[5], "DisconnectionReason")),
    };

    let evt = ConnectionStatusChanged {
        conn_id: load_u32(data, 0),
        connection_status,
        disconnect_reason,
    };

    Ok(Event::ConnectionStatusChanged(evt))
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

pub fn unmarshal_connection_channel_removed(data: &[u8]) -> Result<Event> {
    check_sz(data, 5)?;

    let removed_reason = match num::FromPrimitive::from_u8(data[5]) {
        Some(rem_reason) => rem_reason,
        None => return Err(UnmarshalError::BadEnum(data[5], "RemoveReason")),
    };

    let evt = ConnectionChannelRemoved {
        conn_id: load_u32(data, 0),
        removed_reason,
    };

    Ok(Event::ConnectionChannelRemoved(evt))
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

pub fn unmarshal_button_event(data: &[u8]) -> Result<Event> {
    check_sz(data, 10)?;

    let click_type = match num::FromPrimitive::from_u8(data[5]) {
        Some(click_type) => click_type,
        None => return Err(UnmarshalError::BadEnum(data[5], "ClickType")),
    };

    let evt = ButtonEvent {
        conn_id: load_u32(data, 0),
        click_type,
        was_queued: load_bool(data, 5),
        time_diff: load_u32(data, 6),
    };

    Ok(Event::ButtonEvent(evt))
}

// This is sent to all clients when a button has been successfully verified that was not verified
// before (for the current bluetooth controller bluetooth address). Note: The
// EvtConnectionStatusChanged with connection_status = Ready will be sent just before this event.
// Opcode: 8
pub struct NewVerifiedButton {
    bd_addr: [u8; 6], // The bluetooth address for the verified Flic button.
}

pub fn unmarshal_new_verified_button(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let evt = NewVerifiedButton {
        bd_addr: load_bd_addr(data, 0),
    };

    Ok(Event::NewVerifiedButton(evt))
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

pub fn unmarshal_get_info_response(data: &[u8]) -> Result<Event> {
    // TODO: Implement!
    Err(UnmarshalError::Unimplemented)
}

// Sent when the maximum number of connections has been reached (immediately after the
// EvtConnectionStatusChanged event). If the maximum number of connections is unknown, it is sent
// when the maximum number of connections are reached and an attempt is made to connect yet another
// button.
// Opcode: 10
pub struct NoSpaceForNewConnection {
    max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

pub fn unmarshal_no_space_for_new_connection(data: &[u8]) -> Result<Event> {
    check_sz(data, 1)?;

    let evt = NoSpaceForNewConnection {
        max_concurrently_connected_buttons: data[0],
    };

    Ok(Event::NoSpaceForNewConnection(evt))
}

// Sent when the maximum number of concurrent connections was reached but a button has now
// disconnected, making room for one new connection. Now a new connection attempt will
// automatically be made to devices having a connection channel open but has not yet established a
// connection.
// Opcode: 11
pub struct GotSpaceForNewConnection {
    max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

pub fn unmarshal_got_space_for_new_connection(data: &[u8]) -> Result<Event> {
    check_sz(data, 1)?;

    let evt = GotSpaceForNewConnection {
        max_concurrently_connected_buttons: data[0],
    };

    Ok(Event::GotSpaceForNewConnection(evt))
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

pub fn unmarshal_bluetooth_controller_state_change(data: &[u8]) -> Result<Event> {
    check_sz(data, 1)?;

    let state = match num::FromPrimitive::from_u8(data[0]) {
        Some(state) => state,
        None => return Err(UnmarshalError::BadEnum(data[0], "BluetoothControllerState")),
    };

    let evt = BluetoothControllerStateChange { state };

    Ok(Event::BluetoothControllerStateChange(evt))
}

// Sent in response to a CmdPing
// Opcode: 13
pub struct PingResponse {
    ping_id: u32, // Same ping id as sent in the CmdPing.
}

pub fn unmarshal_ping_response(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = PingResponse {
        ping_id: load_u32(data, 0),
    };

    Ok(Event::PingResponse(evt))
}

// Sent in return to a CmdGetButtonInfo. If the button was not verified, all parameters except
// bd_addr will contain zero-bytes.
// Opcode: 14
pub struct GetButtonInfoResponse {
    bd_addr: [u8; 6], // The bluetooth device address of the request.
    uuid: [u8; 16],   // The uuid of the button. Each button has a unique 128-bit identifier.

    // Next two fields aren't copied directly.
    // color_length: u8, // The length in bytes of the color following.
    // color: [u8; 16], // The first color_length bytes of this array contain the UTF-8 encoding of the color. The other bytes will be zeros. Currently the following strings are defined: black, white, turquoise, green and yellow but more colors may be added later, so don't expect these are the only possible values.
    color: String,

    // Next two fields aren't copied directly.
    // serial_number_length: u8, // The length in bytes of the serial number following.
    // serial_number: [u8; 16], // The serial number of the button, in UTF-8 encoding. Only the first serial_number_length bytes are used. The other bytes will be zeros.
    serial_number: String,
}

pub fn unmarshal_get_button_info_response(data: &[u8]) -> Result<Event> {
    check_sz(data, 56)?;

    let color_len = data[22] as usize;

    let serial_number_len = data[38] as usize;

    let evt = GetButtonInfoResponse {
        bd_addr: load_bd_addr(data, 0),
        uuid: load_uuid(data, 6),
        color: load_string(data, 23, color_len)?,
        serial_number: load_string(data, 40, serial_number_len)?,
    };

    Ok(Event::GetButtonInfoResponse(evt))
}

// Sent once if a previously not verified private button is found during the scan. If this is
// received, tell the user to hold the button down for 7 seconds.
// Opcode: 15
pub struct ScanWizardFoundPrivateButton {
    scan_wizard_id: u32, // Scan wizard id.
}

pub fn unmarshal_scan_wizard_found_private_button(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = ScanWizardFoundPrivateButton {
        scan_wizard_id: load_u32(data, 0),
    };

    Ok(Event::ScanWizardFoundPrivateButton(evt))
}

// Sent once if a previously not verified public button is found during scan. Now the scan wizard
// stops scanning internally and instead initiates a connection to this button.
// Opcode: 16
pub struct ScanWizardFoundPublicButton {
    scan_wizard_id: u32, // Scan wizard id.
    bd_addr: [u8; 6],    // The bluetooth address of the Flic button that was found.

    // Next two fields aren't copied directly.
    // name_length: u8,     // The length in bytes of the name following.
    // name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
    name: String,
}

pub fn unmarshal_scan_wizard_found_public_button(data: &[u8]) -> Result<Event> {
    check_sz(data, 27)?;

    let name_len = data[10] as usize;

    let evt = ScanWizardFoundPublicButton {
        scan_wizard_id: load_u32(data, 0),
        bd_addr: load_bd_addr(data, 4),
        name: load_string(data, 11, name_len)?,
    };

    Ok(Event::ScanWizardFoundPublicButton(evt))
}

// Sent when the found button connects for the first time. Now the verification and pairing process
// will begin.
// Opcode: 17
pub struct ScanWizardButtonConnected {
    scan_wizard_id: u32, // Scan wizard id.
}

pub fn unmarshal_scan_wizard_button_connected(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = ScanWizardButtonConnected {
        scan_wizard_id: load_u32(data, 0),
    };

    Ok(Event::ScanWizardButtonConnected(evt))
}

// Sent when the scan wizard has completed. See ScanWizardResult documentation for more
// information.
// Opcode: 18
pub struct ScanWizardCompleted {
    scan_wizard_id: u32,      // Scan wizard id.
    result: ScanWizardResult, // Result of the scan wizard.
}

pub fn unmarshal_scan_wizard_completed(data: &[u8]) -> Result<Event> {
    check_sz(data, 5)?;

    let result = match num::FromPrimitive::from_u8(data[4]) {
        Some(res) => res,
        None => return Err(UnmarshalError::BadEnum(data[4], "ScanWizardResult")),
    };

    let evt = ScanWizardCompleted {
        scan_wizard_id: load_u32(data, 0),
        result,
    };

    Ok(Event::ScanWizardCompleted(evt))
}

// Sent as a response to CmdDeleteButton or when a verified button has been deleted from the
// database.
// Opcode: 19
pub struct ButtonDeleted {
    bd_addr: [u8; 6],             // The bluetooth device address of the deleted button.
    deleted_by_this_client: bool, // Whether or not the client that initiated the deletion was the current client.
}

pub fn unmarshal_button_deleted(data: &[u8]) -> Result<Event> {
    check_sz(data, 7)?;

    let evt = ButtonDeleted {
        bd_addr: load_bd_addr(data, 0),
        deleted_by_this_client: load_bool(data, 6),
    };

    Ok(Event::ButtonDeleted(evt))
}

// Sent to a battery status listener created by CmdCreateBatteryStatusListener in order to indicate
// the current battery status.
// Opcode: 20
pub struct BatteryStatus {
    listener_id: u32,       // Listener identifier.
    battery_percentage: i8, // A value between 0 and 100 that indicates the current battery status. The value can also be -1 if unknown.
    // TODO: Maybe convert this into some sort of native Rust time type.
    timestamp: i64, // UNIX timestamp (time in seconds since 1970-01-01T00:00:00Z, excluding leap seconds).
}

pub fn unmarshal_battery_status(data: &[u8]) -> Result<Event> {
    check_sz(data, 13)?;

    let evt = BatteryStatus {
        listener_id: load_u32(data, 0),
        battery_percentage: data[4] as i8,
        timestamp: load_i64(data, 5),
    };

    Ok(Event::BatteryStatus(evt))
}
