extern crate num;

#[derive(Clone, FromPrimitive, Debug, PartialEq)]
pub enum CreateConnectionChannelError {
    // There were space in the bluetooth controller's white list to accept a physical pending connection for this button
    NoError = 0,

    // There were no space left in the bluetooth controller to allow a new pending connection
    MaxPendingConnectionsReached = 1,
}

#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum ConnectionStatus {
    // Not currently an established connection, but will connect as soon as the button is pressed and it is in range as long as the connection channel hasn't been removed (and unless maximum number of concurrent connections has been reached or the bluetooth controller has been detached).
    Disconnected = 0,

    // The physical bluetooth connection has just been established and the server and the button are currently verifying each other. As soon as this is done, it will switch to the ready status.
    Connected = 1,

    // The verification is done and button events may now arrive.
    Ready = 2,
}

#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum DisconnectReason {
    // Unknown reason
    Unspecified = 0,

    // The bluetooth controller established a connection, but the Flic button didn't answer in time.
    ConnectionEstablishmentFailed = 1,

    // The connection to the Flic button was lost due to either being out of range or some radio communication problems.
    TimedOut = 2,

    // The server and the Flic button for some reason don't agree on the previously established bonding keys.
    BondingKeysMismatch = 3,
}

#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum RemovedReason {
    // The connection channel was removed by this client.
    RemovedByThisClient = 0,

    // The connection channel was removed due to a force disconnect by this client.
    ForceDisconnectedByThisClient = 1,

    // Another client force disconnected the button used in this connection channel.
    ForceDisconnectedByOtherClient = 2,

    // The next four reasons might only happen if the Flic button is previously not verified, i.e. these are errors that might happen during the bonding process.

    // The button is not in public mode. Hold it down for 7 seconds while not trying to establish a connection, then try to reconnect by creating a new connection channel.
    ButtonIsPrivate = 3,

    // After the connection was established, the bonding procedure didn't complete in time.
    VerifyTimeout = 4,

    // The internet request to the Flic backend failed.
    InternetBackendError = 5,

    // According to the Flic backend, this Flic button supplied invalid identity data.
    InvalidData = 6,

    // The next reason may only occur on Windows (i.e. the Windows daemon is used).

    // The file representing the Flic Bluetooth device could not be opened, or it is reporting invalid status. If this happens, manually unpair the device in Windows's Bluetooth settings.
    CouldntLoadDevice = 7,

    // The button was deleted by this client by a call to CmdDeleteButton.
    DeletedByThisClient = 8,

    // The button was deleted by another client by a call to CmdDeleteButton.
    DeletedByOtherClient = 9,

    // The button belongs to another PbF partner.
    ButtonBelongsToOtherPartner = 10,

    // The button was factory reset, or the pairing has been removed to fit a new one.
    DeletedFromButton = 11,
}

#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum ClickType {
    // The button was pressed.
    ButtonDown = 0,

    // The button was released.
    ButtonUp = 1,

    // The button was clicked, and was held for at most 1 seconds between press and release.
    ButtonClick = 2,

    // The button was clicked once.
    ButtonSingleClick = 3,

    // The button was clicked twice. The time between the first and second press must be at most 0.5 seconds.
    ButtonDoubleClick = 4,

    // The button was held for at least 1 second.
    ButtonHold = 5,
}

// The server can be configured to either use the burnt-in public address stored inside the
// bluetooth controller, or to use a custom random static address. This custom address is a good
// idea if you want to be able to use your database with bonding information with a different
// bluetooth controller.
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum BdAddrType {
    PublicBdAddrType = 0,
    RandomBdAddrType = 1,
}

// This specifies the accepted latency mode for the corresponding connection channel. The physical
// bluetooth connection will use the lowest mode set by any connection channel. The battery usage
// for the Flic button is normally about the same for all modes if the connection is stable.
// However lower modes will have higher battery usage if the connection is unstable. Lower modes
// also consumes more power for the client, which is normally not a problem since most computers
// run on wall power or have large batteries.
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum LatencyMode {
    // Up to 100 ms latency.
    Normal = 0,

    // Up to 17.5 ms latency.
    Low = 1,

    // Up to 275 ms latency.
    High = 2,
}

// The server software detects when the bluetooth controller is removed or is made unavailable. It
// will then repeatedly retry to re-established a connection to the same bluetooth controller.
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum BluetoothControllerState {
    // The server software has lost the HCI socket to the bluetooth controller and is trying to reconnect.
    Detached = 0,

    // The server software has just got connected to the HCI socket and initiated a reset of the bluetooth controller.
    Resetting = 1,

    // The bluetooth controller has done initialization and is up and running.
    Attached = 2,
}

//The result of a scan wizard. When the scan wizard is completed it will stop and return a result.
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
pub enum ScanWizardResult {
    // Indicates that a button was successfully paired and verified. You may now create a connection channel to that button.
    WizardSuccess = 0,

    // A CmdCancelScanWizard was sent.
    WizardCancelledByUser = 1,

    // The scan wizard did not make any progress for some time. Current timeouts are 20 seconds for finding any button, 20 seconds for finding a public button (in case of a private button was found), 10 seconds for connecting the button, 30 seconds for pairing and verifying the button.
    WizardFailedTimeout = 2,

    // First the button was advertising public status, but after connecting it reports private. Probably it switched from public to private just when the connection attempt was started.
    WizardButtonIsPrivate = 3,

    // The bluetooth controller is not attached.
    WizardBluetoothUnavailable = 4,

    // The internet request to the Flic backend failed.
    WizardInternetBackendError = 5,

    // According to the Flic backend, this Flic button supplied invalid identity data.
    WizardInvalidData = 6,

    // The button belongs to another PbF partner.
    WizardButtonBelongsToOtherPartner = 7,

    // The Flic 2 button is already connected to another device. Please disconnect it first so it becomes available.
    WizardButtonAlreadyConnectedToOtherDevice = 8,
}
