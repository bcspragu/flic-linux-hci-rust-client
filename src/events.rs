// TODO: Remove this, all this code will be used by clients eventually.
#![allow(dead_code)]

use crate::enums::*;
use crate::error::{FlicError, UnmarshalError};
use crate::BdAddr;
use crate::Result;
use num::FromPrimitive;

#[derive(FromPrimitive, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    AdvertisementPacket = 0,
    CreateConnectionChannelResponse = 1,
    ConnectionStatusChanged = 2,
    ConnectionChannelRemoved = 3,
    ButtonUpOrDown = 4,
    ButtonClickOrHold = 5,
    ButtonSingleOrDoubleClick = 6,
    ButtonSingleOrDoubleClickOrHold = 7,
    NewVerifiedButton = 8,
    GetInfoResponse = 9,
    NoSpaceForNewConnection = 10,
    GotSpaceForNewConnection = 11,
    BluetoothControllerStateChange = 12,
    PingResponse = 13,
    GetButtonInfoResponse = 14,
    ScanWizardFoundPrivateButton = 15,
    ScanWizardFoundPublicButton = 16,
    ScanWizardButtonConnected = 17,
    ScanWizardCompleted = 18,
    ButtonDeleted = 19,
    BatteryStatus = 20,
}

pub fn unmarshal(data: &[u8]) -> Result<(Event, Opcode)> {
    let opcode = match FromPrimitive::from_u8(data[0]) {
        Some(opcode) => opcode,
        None => return Err(FlicError::Unmarshal(UnmarshalError::BadOpcode(data[0]))),
    };

    let unmarshal_event = match opcode {
        Opcode::AdvertisementPacket => unmarshal_advertisement_packet,
        Opcode::CreateConnectionChannelResponse => unmarshal_create_connection_channel_response,
        Opcode::ConnectionStatusChanged => unmarshal_connection_status_changed,
        Opcode::ConnectionChannelRemoved => unmarshal_connection_channel_removed,
        Opcode::ButtonUpOrDown => unmarshal_button_up_or_down,
        Opcode::ButtonClickOrHold => unmarshal_button_click_or_hold,
        Opcode::ButtonSingleOrDoubleClick => unmarshal_button_single_or_double_click,
        Opcode::ButtonSingleOrDoubleClickOrHold => unmarshal_button_single_or_double_click_or_hold,
        Opcode::NewVerifiedButton => unmarshal_new_verified_button,
        Opcode::GetInfoResponse => unmarshal_get_info_response,
        Opcode::NoSpaceForNewConnection => unmarshal_no_space_for_new_connection,
        Opcode::GotSpaceForNewConnection => unmarshal_got_space_for_new_connection,
        Opcode::BluetoothControllerStateChange => unmarshal_bluetooth_controller_state_change,
        Opcode::PingResponse => unmarshal_ping_response,
        Opcode::GetButtonInfoResponse => unmarshal_get_button_info_response,
        Opcode::ScanWizardFoundPrivateButton => unmarshal_scan_wizard_found_private_button,
        Opcode::ScanWizardFoundPublicButton => unmarshal_scan_wizard_found_public_button,
        Opcode::ScanWizardButtonConnected => unmarshal_scan_wizard_button_connected,
        Opcode::ScanWizardCompleted => unmarshal_scan_wizard_completed,
        Opcode::ButtonDeleted => unmarshal_button_deleted,
        Opcode::BatteryStatus => unmarshal_battery_status,
    };

    let evt = unmarshal_event(&data[1..])?;

    Ok((evt, opcode))
}

#[derive(Debug, PartialEq)]
pub enum Event {
    AdvertisementPacket(AdvertisementPacket),
    CreateConnectionChannelResponse(CreateConnectionChannelResponse),
    ConnectionStatusChanged(ConnectionStatusChanged),
    ConnectionChannelRemoved(ConnectionChannelRemoved),
    ButtonUpOrDown(ButtonUpOrDown),
    ButtonClickOrHold(ButtonClickOrHold),
    ButtonSingleOrDoubleClick(ButtonSingleOrDoubleClick),
    ButtonSingleOrDoubleClickOrHold(ButtonSingleOrDoubleClickOrHold),
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

fn check_sz_at_least(data: &[u8], want_len: usize) -> Result<()> {
    if data.len() >= want_len {
        return Ok(());
    }

    Err(FlicError::Unmarshal(UnmarshalError::BadLengthAtLeast(
        data.len(),
        want_len,
    )))
}

fn check_sz(data: &[u8], want_len: usize) -> Result<()> {
    if data.len() == want_len {
        return Ok(());
    }

    Err(FlicError::Unmarshal(UnmarshalError::BadLength(
        data.len(),
        want_len,
    )))
}

fn load_bool(data: &[u8], o: usize) -> bool {
    data[o] == 1
}

fn load_u16(data: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([data[o], data[o + 1]])
}

fn load_u32(data: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([data[o], data[o + 1], data[o + 2], data[o + 3]])
}

fn load_i16(data: &[u8], o: usize) -> i16 {
    i16::from_le_bytes([data[o], data[o + 1]])
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
    let res = String::from_utf8(data[o..(o + sz)].to_vec());
    match res {
        Ok(s) => Ok(s),
        Err(err) => Err(FlicError::Unmarshal(UnmarshalError::BadString(err))),
    }
}

fn load_bd_addr(data: &[u8], o: usize) -> BdAddr {
    BdAddr([
        data[o],
        data[o + 1],
        data[o + 2],
        data[o + 3],
        data[o + 4],
        data[o + 5],
    ])
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
#[derive(Debug, PartialEq)]
pub struct AdvertisementPacket {
    pub scan_id: u32, // The scan id corresponding to the scanner which this advertisement packet belongs to.
    pub bd_addr: BdAddr, // The bluetooth address of this Flic button. Use it to establish a connection chnanel.

    // Next two fields aren't copied directly
    // name_length: u8,  // The length in bytes of the name following.
    // name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
    pub name: String,
    pub rssi: i8, // Signal strength in dBm, between -126 and 20, where -127 is weakest and 20 is strongest. -127 means not available.
    pub is_private: bool, // The Flic button is currently in private mode and won't accept connections from unbonded clients. Hold it down for 7 seconds while not attempting to connect to it to make it public. First then you may connect.
    pub already_verified: bool, // If the server has the bonding key for this Flic button, this value is true. That means you should be able to connect to it.
    pub already_connected_to_this_device: bool, // This Flic 2 button is already connected to this device.
    pub already_connected_to_other_device: bool, // This Flic 2 button is already connected to another device.
}

fn unmarshal_advertisement_packet(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct CreateConnectionChannelResponse {
    pub conn_id: u32,                        // Connection channel identifier.
    pub error: CreateConnectionChannelError, // Whether the request succeeded or not.
    pub connection_status: ConnectionStatus, // The current connection status to this button. This might be a non-disconnected status if there are already other active connection channels to this button.
}

fn unmarshal_create_connection_channel_response(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let error = match FromPrimitive::from_u8(data[4]) {
        Some(err) => err,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[4],
                String::from("CreateConnectionChannelError"),
            )))
        }
    };

    let connection_status = match FromPrimitive::from_u8(data[5]) {
        Some(conn_status) => conn_status,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[5],
                String::from("ConnectionStatus"),
            )))
        }
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
#[derive(Debug, PartialEq)]
pub struct ConnectionStatusChanged {
    pub conn_id: u32,                        // Connection channel identifier.
    pub connection_status: ConnectionStatus, // New connection status.
    pub disconnect_reason: DisconnectReason, // If the connection status is Disconnected, this contains the reason. Otherwise this parameter is considered invalid.
}

fn unmarshal_connection_status_changed(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let connection_status = match FromPrimitive::from_u8(data[4]) {
        Some(conn_status) => conn_status,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[4],
                String::from("ConnectionStatus"),
            )))
        }
    };

    let disconnect_reason = match FromPrimitive::from_u8(data[5]) {
        Some(disconn_reason) => disconn_reason,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[5],
                String::from("DisconnectionReason"),
            )))
        }
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
#[derive(Debug, PartialEq)]
pub struct ConnectionChannelRemoved {
    pub conn_id: u32,                  // Connection channel identifier.
    pub removed_reason: RemovedReason, // Reason for this connection channel being removed.
}

fn unmarshal_connection_channel_removed(data: &[u8]) -> Result<Event> {
    check_sz(data, 5)?;

    let removed_reason = match FromPrimitive::from_u8(data[4]) {
        Some(rem_reason) => rem_reason,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[4],
                String::from("RemoveReason"),
            )))
        }
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
// No opcode because this isn't actually an event.
struct BaseButtonEvent {
    conn_id: u32,          // Connection channel identifier.
    click_type: ClickType, // The click type. For each opcode, there are different possible values.
    was_queued: bool, // If this button event happened during the button was disconnected or not.
    time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

fn unmarshal_base_button_event(data: &[u8]) -> Result<BaseButtonEvent> {
    check_sz(data, 10)?;

    let click_type = match FromPrimitive::from_u8(data[4]) {
        Some(click_type) => click_type,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[4],
                String::from("ClickType"),
            )))
        }
    };

    Ok(BaseButtonEvent {
        conn_id: load_u32(data, 0),
        click_type,
        was_queued: load_bool(data, 5),
        time_diff: load_u32(data, 6),
    })
}

// Possible ClickTypes are ButtonUp and ButtonDown. Used to simply know when the button was pressed
// or released.
// Opcode: 4
#[derive(Debug, PartialEq)]
pub struct ButtonUpOrDown {
    pub conn_id: u32,          // Connection channel identifier.
    pub click_type: ClickType, // The click type. For each opcode, there are different possible values.
    pub was_queued: bool, // If this button event happened during the button was disconnected or not.
    pub time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

fn unmarshal_button_up_or_down(data: &[u8]) -> Result<Event> {
    let base_event = unmarshal_base_button_event(data)?;

    match base_event.click_type {
        ClickType::ButtonUp | ClickType::ButtonDown => (), // This is fine
        _ => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadClickType(
                base_event.click_type,
                String::from("ButtonUpOrDown"),
            )))
        }
    };

    Ok(Event::ButtonUpOrDown(ButtonUpOrDown {
        conn_id: base_event.conn_id,
        click_type: base_event.click_type,
        was_queued: base_event.was_queued,
        time_diff: base_event.time_diff,
    }))
}

// Possible ClickTypes are ButtonClick and ButtonHold. Used if you want to distinguish between
// click and hold.
// Opcode: 5
#[derive(Debug, PartialEq)]
pub struct ButtonClickOrHold {
    pub conn_id: u32,          // Connection channel identifier.
    pub click_type: ClickType, // The click type. For each opcode, there are different possible values.
    pub was_queued: bool, // If this button event happened during the button was disconnected or not.
    pub time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

fn unmarshal_button_click_or_hold(data: &[u8]) -> Result<Event> {
    let base_event = unmarshal_base_button_event(data)?;

    match base_event.click_type {
        ClickType::ButtonClick | ClickType::ButtonHold => (), // This is fine
        _ => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadClickType(
                base_event.click_type,
                String::from("ButtonClickOrHold"),
            )))
        }
    };

    Ok(Event::ButtonClickOrHold(ButtonClickOrHold {
        conn_id: base_event.conn_id,
        click_type: base_event.click_type,
        was_queued: base_event.was_queued,
        time_diff: base_event.time_diff,
    }))
}

// Possible ClickTypes are ButtonSingleClick and ButtonDoubleClick. Used if you want to distinguish
// between a single click and a double click.
// Opcode: 6
#[derive(Debug, PartialEq)]
pub struct ButtonSingleOrDoubleClick {
    pub conn_id: u32,          // Connection channel identifier.
    pub click_type: ClickType, // The click type. For each opcode, there are different possible values.
    pub was_queued: bool, // If this button event happened during the button was disconnected or not.
    pub time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

fn unmarshal_button_single_or_double_click(data: &[u8]) -> Result<Event> {
    let base_event = unmarshal_base_button_event(data)?;

    match base_event.click_type {
        ClickType::ButtonSingleClick | ClickType::ButtonDoubleClick => (), // This is fine
        _ => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadClickType(
                base_event.click_type,
                String::from("ButtonSingleOrDoubleClick"),
            )))
        }
    };

    Ok(Event::ButtonSingleOrDoubleClick(
        ButtonSingleOrDoubleClick {
            conn_id: base_event.conn_id,
            click_type: base_event.click_type,
            was_queued: base_event.was_queued,
            time_diff: base_event.time_diff,
        },
    ))
}

// Possible ClickTypes are ButtonSingleClick, ButtonDoubleClick and ButtonHold. Used if you want to
// distinguish between a single click, a double click and a hold.
// Opcode: 7
#[derive(Debug, PartialEq)]
pub struct ButtonSingleOrDoubleClickOrHold {
    pub conn_id: u32,          // Connection channel identifier.
    pub click_type: ClickType, // The click type. For each opcode, there are different possible values.
    pub was_queued: bool, // If this button event happened during the button was disconnected or not.
    pub time_diff: u32, // If this button event happened during the button was disconnected, this will be the number of seconds since that event happened (otherwise it will most likely be 0). Depending on your application, you might want to discard too old events.
}

fn unmarshal_button_single_or_double_click_or_hold(data: &[u8]) -> Result<Event> {
    let base_event = unmarshal_base_button_event(data)?;

    match base_event.click_type {
        ClickType::ButtonSingleClick | ClickType::ButtonDoubleClick | ClickType::ButtonHold => (), // This is fine
        _ => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadClickType(
                base_event.click_type,
                String::from("ButtonSingleOrDoubleClickOrHold"),
            )))
        }
    };

    Ok(Event::ButtonSingleOrDoubleClickOrHold(
        ButtonSingleOrDoubleClickOrHold {
            conn_id: base_event.conn_id,
            click_type: base_event.click_type,
            was_queued: base_event.was_queued,
            time_diff: base_event.time_diff,
        },
    ))
}

// This is sent to all clients when a button has been successfully verified that was not verified
// before (for the current bluetooth controller bluetooth address). Note: The
// EvtConnectionStatusChanged with connection_status = Ready will be sent just before this event.
// Opcode: 8
#[derive(Debug, PartialEq)]
pub struct NewVerifiedButton {
    pub bd_addr: BdAddr, // The bluetooth address for the verified Flic button.
}

fn unmarshal_new_verified_button(data: &[u8]) -> Result<Event> {
    check_sz(data, 6)?;

    let evt = NewVerifiedButton {
        bd_addr: load_bd_addr(data, 0),
    };

    Ok(Event::NewVerifiedButton(evt))
}

// This is sent as a response to a CmdGetInfo.
// Opcode: 9
#[derive(Debug, PartialEq)]
pub struct GetInfoResponse {
    pub bluetooth_controller_state: BluetoothControllerState, // Current state of the HCI connection to the bluetooth controller.
    pub my_bd_addr: BdAddr, // Current bluetooth address / identity of this device.
    pub my_bd_addr_type: BdAddrType, // Current bluetooth address type of this device.
    pub max_pending_connections: u8, // The max number of Flic buttons that can be monitored at the same time, regardless of having an established connection or not.
    pub max_concurrently_connected_buttons: i16, // The max number of Flic buttons that can have an established bluetooth connection at the same time. If this amount is reached, no other pending connection will succeed until another one has disconnected. This value will be -1 until the value becomes known. It becomes known first when the maximum number of connections is currently established and there is an attempt to establish yet another connection. Not all bluetooth controllers handle this correctly; some simply hides the fact that the maximum is reached and further connections won't succeed successfully, until a previously established connection is disconnected. Note: For some bluetooth controllers we have tested we have already hardcoded the correct value and this parameter will thus not be -1 but the correct one.
    pub current_pending_connections: u8, // Current number of Flic buttons that are monitored by the server, among all clients.
    pub currently_no_space_for_new_connection: bool, // The maximum number of concurrently connected buttons has been reached.
    pub nb_verified_buttons: u16, // Number of verified buttons for this my_bd_addr/my_bd_addr_type pair.
    pub bd_addr_of_verified_buttons: Vec<BdAddr>, // An array of all the verified buttons.
}

fn unmarshal_get_info_response(data: &[u8]) -> Result<Event> {
    // We can't use check_sz off the bat for this one since it's dynamically sized. So first, we
    // check it's at least large enough to include the number of entries in its repeated field.
    check_sz_at_least(data, 15)?;

    let nb_verified_buttons = load_u16(data, 13) as usize;

    // Now we can see if the total size makes sense.
    check_sz(data, 15 + nb_verified_buttons * 6)?;

    let bluetooth_controller_state = match FromPrimitive::from_u8(data[0]) {
        Some(status) => status,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[0],
                String::from("BluetoothControllerState"),
            )))
        }
    };

    let my_bd_addr_type = match FromPrimitive::from_u8(data[7]) {
        Some(addr_type) => addr_type,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[7],
                String::from("BdAddrType"),
            )))
        }
    };

    let mut bd_addr_of_verified_buttons = Vec::with_capacity(nb_verified_buttons);

    for i in 0..nb_verified_buttons {
        bd_addr_of_verified_buttons[i] = load_bd_addr(data, 15 + i * 6);
    }

    let evt = GetInfoResponse {
        bluetooth_controller_state,
        my_bd_addr: load_bd_addr(data, 1),
        my_bd_addr_type,
        max_pending_connections: data[8],
        max_concurrently_connected_buttons: load_i16(data, 9),
        current_pending_connections: data[11],
        currently_no_space_for_new_connection: load_bool(data, 12),
        nb_verified_buttons: nb_verified_buttons as u16,
        bd_addr_of_verified_buttons,
    };

    Ok(Event::GetInfoResponse(evt))
}

// Sent when the maximum number of connections has been reached (immediately after the
// EvtConnectionStatusChanged event). If the maximum number of connections is unknown, it is sent
// when the maximum number of connections are reached and an attempt is made to connect yet another
// button.
// Opcode: 10
#[derive(Debug, PartialEq)]
pub struct NoSpaceForNewConnection {
    pub max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

fn unmarshal_no_space_for_new_connection(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct GotSpaceForNewConnection {
    pub max_concurrently_connected_buttons: u8, // Same as in EvtGetInfoResponse.
}

fn unmarshal_got_space_for_new_connection(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct BluetoothControllerStateChange {
    pub state: BluetoothControllerState, // The new state.
}

fn unmarshal_bluetooth_controller_state_change(data: &[u8]) -> Result<Event> {
    check_sz(data, 1)?;

    let state = match FromPrimitive::from_u8(data[0]) {
        Some(state) => state,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[0],
                String::from("BluetoothControllerState"),
            )))
        }
    };

    let evt = BluetoothControllerStateChange { state };

    Ok(Event::BluetoothControllerStateChange(evt))
}

// Sent in response to a CmdPing
// Opcode: 13
#[derive(Debug, PartialEq)]
pub struct PingResponse {
    pub ping_id: u32, // Same ping id as sent in the CmdPing.
}

fn unmarshal_ping_response(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = PingResponse {
        ping_id: load_u32(data, 0),
    };

    Ok(Event::PingResponse(evt))
}

// Sent in return to a CmdGetButtonInfo. If the button was not verified, all parameters except
// bd_addr will contain zero-bytes.
// Opcode: 14
#[derive(Debug, PartialEq)]
pub struct GetButtonInfoResponse {
    pub bd_addr: BdAddr, // The bluetooth device address of the request.
    pub uuid: [u8; 16],  // The uuid of the button. Each button has a unique 128-bit identifier.

    // Next two fields aren't copied directly.
    // color_length: u8, // The length in bytes of the color following.
    // color: [u8; 16], // The first color_length bytes of this array contain the UTF-8 encoding of the color. The other bytes will be zeros. Currently the following strings are defined: black, white, turquoise, green and yellow but more colors may be added later, so don't expect these are the only possible values.
    pub color: String,

    // Next two fields aren't copied directly.
    // serial_number_length: u8, // The length in bytes of the serial number following.
    // serial_number: [u8; 16], // The serial number of the button, in UTF-8 encoding. Only the first serial_number_length bytes are used. The other bytes will be zeros.
    pub serial_number: String,
}

fn unmarshal_get_button_info_response(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct ScanWizardFoundPrivateButton {
    pub scan_wizard_id: u32, // Scan wizard id.
}

fn unmarshal_scan_wizard_found_private_button(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = ScanWizardFoundPrivateButton {
        scan_wizard_id: load_u32(data, 0),
    };

    Ok(Event::ScanWizardFoundPrivateButton(evt))
}

// Sent once if a previously not verified public button is found during scan. Now the scan wizard
// stops scanning internally and instead initiates a connection to this button.
// Opcode: 16
#[derive(Debug, PartialEq)]
pub struct ScanWizardFoundPublicButton {
    pub scan_wizard_id: u32, // Scan wizard id.
    pub bd_addr: BdAddr,     // The bluetooth address of the Flic button that was found.

    // Next two fields aren't copied directly.
    // name_length: u8,     // The length in bytes of the name following.
    // name: [u8; 16], // The first name_length bytes of this array contain the UTF-8 encoding of the advertised name. The other bytes will be zeros.
    pub name: String,
}

fn unmarshal_scan_wizard_found_public_button(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct ScanWizardButtonConnected {
    pub scan_wizard_id: u32, // Scan wizard id.
}

fn unmarshal_scan_wizard_button_connected(data: &[u8]) -> Result<Event> {
    check_sz(data, 4)?;

    let evt = ScanWizardButtonConnected {
        scan_wizard_id: load_u32(data, 0),
    };

    Ok(Event::ScanWizardButtonConnected(evt))
}

// Sent when the scan wizard has completed. See ScanWizardResult documentation for more
// information.
// Opcode: 18
#[derive(Debug, PartialEq)]
pub struct ScanWizardCompleted {
    pub scan_wizard_id: u32,      // Scan wizard id.
    pub result: ScanWizardResult, // Result of the scan wizard.
}

fn unmarshal_scan_wizard_completed(data: &[u8]) -> Result<Event> {
    check_sz(data, 5)?;

    let result = match FromPrimitive::from_u8(data[4]) {
        Some(res) => res,
        None => {
            return Err(FlicError::Unmarshal(UnmarshalError::BadEnum(
                data[4],
                String::from("ScanWizardResult"),
            )))
        }
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
#[derive(Debug, PartialEq)]
pub struct ButtonDeleted {
    pub bd_addr: BdAddr, // The bluetooth device address of the deleted button.
    pub deleted_by_this_client: bool, // Whether or not the client that initiated the deletion was the current client.
}

fn unmarshal_button_deleted(data: &[u8]) -> Result<Event> {
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
#[derive(Debug, PartialEq)]
pub struct BatteryStatus {
    pub listener_id: u32,       // Listener identifier.
    pub battery_percentage: i8, // A value between 0 and 100 that indicates the current battery status. The value can also be -1 if unknown.
    // TODO: Maybe convert this into some sort of native Rust time type.
    pub timestamp: i64, // UNIX timestamp (time in seconds since 1970-01-01T00:00:00Z, excluding leap seconds).
}

fn unmarshal_battery_status(data: &[u8]) -> Result<Event> {
    check_sz(data, 13)?;

    let evt = BatteryStatus {
        listener_id: load_u32(data, 0),
        battery_percentage: data[4] as i8,
        timestamp: load_i64(data, 5),
    };

    Ok(Event::BatteryStatus(evt))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! unmarshal_tests {
        ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (data, want): (&[u8], Event) = $value;
            let (got, _) = unmarshal(data).expect("failed to unmarshal valid data");
            assert_eq!(want, got);
        }

    )*
    }
    }

    unmarshal_tests! {
        unmarshal_advertisement_packet: (
            &vec![
                0x00, // opcode
                0x78, 0x56, 0x34, 0x12, // scan_id
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
                0x08, // name_length
                0x54, 0x68, 0x65, 0x20, 0x4E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // name
                0x78, // rssi
                0x01, // is_private,
                0x00, // already_verified
                0x01, // already_connected_to_this_device,
                0x00, // already_connected_to_other_device,
            ],
            Event::AdvertisementPacket(AdvertisementPacket{
                scan_id: 0x12345678,
                bd_addr: BdAddr([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]),
                name: String::from("The Name"),
                rssi: 120,
                is_private: true,
                already_verified: false,
                already_connected_to_this_device: true,
                already_connected_to_other_device: false,
            })
            ),
        unmarshal_create_connection_channel_response: (
            &vec![
                0x01, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x01, // error
                0x02, // connection_status
            ],
            Event::CreateConnectionChannelResponse(CreateConnectionChannelResponse{
                conn_id: 0x12345678,
                error: CreateConnectionChannelError::MaxPendingConnectionsReached,
                connection_status: ConnectionStatus::Ready,
            })
            ),
        unmarshal_connection_status_changed: (
            &vec![
                0x02, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x01, // connection_status
                0x02, // disconnect_reason
            ],
            Event::ConnectionStatusChanged(ConnectionStatusChanged{
                conn_id: 0x12345678,
                connection_status: ConnectionStatus::Connected,
                disconnect_reason: DisconnectReason::TimedOut,
            })
            ),
        unmarshal_connection_channel_removed: (
            &vec![
                0x03, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x03, // removed_reason
            ],
            Event::ConnectionChannelRemoved(ConnectionChannelRemoved{
                conn_id: 0x12345678,
                removed_reason: RemovedReason::ButtonIsPrivate,
            })
            ),
        unmarshal_button_up_or_down: (
            &vec![
                0x04, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x01, // click_type
                0x01, // was_queued
                0x89, 0x67, 0x45, 0x23, // time_diff
            ],
            Event::ButtonUpOrDown(ButtonUpOrDown{
                conn_id: 0x12345678,
                click_type: ClickType::ButtonUp,
                was_queued: true,
                time_diff: 0x23456789,
            })
            ),
        unmarshal_button_click_or_hold: (
            &vec![
                0x05, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x02, // click_type
                0x01, // was_queued
                0x89, 0x67, 0x45, 0x23, // time_diff
            ],
            Event::ButtonClickOrHold(ButtonClickOrHold{
                conn_id: 0x12345678,
                click_type: ClickType::ButtonClick,
                was_queued: true,
                time_diff: 0x23456789,
            })
            ),
        unmarshal_button_single_or_double_click: (
            &vec![
                0x06, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x04, // click_type
                0x01, // was_queued
                0x89, 0x67, 0x45, 0x23, // time_diff
            ],
            Event::ButtonSingleOrDoubleClick(ButtonSingleOrDoubleClick{
                conn_id: 0x12345678,
                click_type: ClickType::ButtonDoubleClick,
                was_queued: true,
                time_diff: 0x23456789,
            })
            ),
        unmarshal_button_single_or_double_click_or_hold: (
            &vec![
                0x07, // opcode
                0x78, 0x56, 0x34, 0x12, // conn_id
                0x05, // click_type
                0x01, // was_queued
                0x89, 0x67, 0x45, 0x23, // time_diff
            ],
            Event::ButtonSingleOrDoubleClickOrHold(ButtonSingleOrDoubleClickOrHold{
                conn_id: 0x12345678,
                click_type: ClickType::ButtonHold,
                was_queued: true,
                time_diff: 0x23456789,
            })
            ),
        unmarshal_new_verified_button: (
            &vec![
                0x08, // opcode
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, // bd_addr
            ],
            Event::NewVerifiedButton(NewVerifiedButton{
                bd_addr: BdAddr([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]),
            })
            ),
            /*
        TODO: Fill out and uncomment the rest of these tests.
        unmarshal_get_info_response: (
            &vec![],
            Event::GetInfoResponse(GetInfoResponse{
                bluetooth_controller_state: BluetoothControllerState,
                my_bd_addr: BdAddr,
                my_bd_addr_type: BdAddrType,
                max_pending_connections: u8,
                max_concurrently_connected_buttons: i16,
                current_pending_connections: u8,
                currently_no_space_for_new_connection: bool,
                nb_verified_buttons: u16,
                bd_addr_of_verified_buttons: Vec<BdAddr>,
            })
            ),
        unmarshal_no_space_for_new_connection: (
            &vec![],
            Event::NoSpaceForNewConnection(NoSpaceForNewConnection{
                max_concurrently_connected_buttons: u8,
            })
            ),
        unmarshal_got_space_for_new_connection: (
            &vec![],
            Event::GotSpaceForNewConnection(GotSpaceForNewConnection{
                max_concurrently_connected_buttons: u8,
            })
            ),
        unmarshal_bluetooth_controller_state_change: (
            &vec![],
            Event::BluetoothControllerStateChange(BluetoothControllerStateChange{
                state: BluetoothControllerState,
            })
            ),
        unmarshal_ping_response: (
            &vec![],
            Event::PingResponse(PingResponse{
                ping_id: u32,
            })
            ),
        unmarshal_get_button_info_response: (
            &vec![],
            Event::GetButtonInfoResponse(GetButtonInfoResponse{
                bd_addr: BdAddr,
                uuid: [u8; 16],

                // Next two fields aren't copied directly.
                // color_length: u8,
                // color: [u8; 16],
                color: String,

                // Next two fields aren't copied directly.
                // serial_number_length: u8,
                // serial_number: [u8; 16],
                serial_number: String,
            })
            ),
        unmarshal_scan_wizard_found_private_button: (
            &vec![],
            Event::ScanWizardFoundPrivateButton(ScanWizardFoundPrivateButton{
                scan_wizard_id: u32,
            })
            ),
        unmarshal_scan_wizard_found_public_button: (
            &vec![],
            Event::ScanWizardFoundPublicButton(ScanWizardFoundPublicButton{
                scan_wizard_id: u32,
                bd_addr: BdAddr,

                // Next two fields aren't copied directly.
                // name_length: u8,
                // name: [u8; 16],
                name: String,
            })
            ),
        unmarshal_scan_wizard_button_connected: (
            &vec![],
            Event::ScanWizardButtonConnected(ScanWizardButtonConnected{
                scan_wizard_id: u32, // Scan wizard id.
            })
            ),
        unmarshal_scan_wizard_completed: (
            &vec![],
            Event::ScanWizardCompleted(ScanWizardCompleted{
                scan_wizard_id: u32,
                result: ScanWizardResult,
            })
            ),
        unmarshal_button_deleted: (
            &vec![],
            Event::ButtonDeleted(ButtonDeleted{
                bd_addr: BdAddr,
                deleted_by_this_client: bool,
            })
            ),
        unmarshal_battery_status: (
            &vec![],
            Event::BatteryStatus(BatteryStatus{
                listener_id: u32,
                battery_percentage: i8,
                // TODO: Maybe convert this into some sort of native Rust time type.
                timestamp: i64,
            })
            ),
        */
    }
}
