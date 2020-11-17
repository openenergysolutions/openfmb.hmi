use crate::actors::{coordinator::openfmb::OpenFMBDevice, CircuitSegmentError};

///The different states a circuit segment can be in, wrt connectivity to another segment
/// TODO generalize this to allow connection to multiple other segments
/// Probably replace this with a more general CircuitSegmentTopologyState, but this will do for now
#[allow(unused)]
pub enum CircuitSegmentConnectionState {
    ///Also known as islanding if the circuit is self-powering
    ///But avoiding using that term here because it's not quite general enough(?)
    Disconnected,

    ///Yes, this segment is currently connected to another segment
    Connected,

    ///A marker set internally when commands have been issued to island the segment
    /// but they result has not yet been verified
    Disconnecting,

    ///A marker set internally when commands have been issued to connect the segment but they
    /// result has not yet been verified
    Connecting,

    ///Indicates that an invalid operating state has been detected
    /// Actor must cease coordination activities and raise a fault
    /// Relies on external mechanism to put devices in fail-safe mode(?)
    Error,
}

///All actors that model a circuit segment as a whole must implement this trait
pub trait CircuitSegment {
    ///Returns the currently known set of (OpenFMB) devices connected to this circuit
    fn devices(&self) -> Vec<Box<OpenFMBDevice>>;

    ///Subscribes to all openfmb messages on this segment and starts mirroring the segments state
    fn start_monitoring(&mut self);

    ///Unsubscribes from all messages on this segment and stops monitoring the segment state
    fn stop_monitoring(&mut self);

    ///Starts monitoring the segment if it wasn't already, and also starts the primary circuit
    ///coordination algorithm configured for this circuit
    fn start_coordinating(&mut self);

    ///Stops sending any control messages but remains in monitoring mode
    fn stop_coordinating(&mut self);
}

///Any circuit segment that can be connected to another circuit segment through
/// a configuration change must implement this trait
pub trait ConnectableCircuitSegment: CircuitSegment {
    ///Returns the last known connection state for the circuit
    fn connection_state(&self) -> CircuitSegmentConnectionState;

    ///Look at most recent status messages to determine the actual state
    /// and updates the internal CircuitSegmentConnectionState to match.
    /// Used to complete a connect() operation when the state is still
    /// Connecting. Returns the calculated connection state
    fn update_connection_state(&mut self) -> CircuitSegmentConnectionState;

    ///Look at most recent status messages to determine the actual state
    /// and updates the internal CircuitSegmentConnectionState to match.
    /// Used to complete a disconnect() operation when the state is still
    /// Disconnecting. Returns the calculated connection state
    fn update_disconnection_state(&mut self) -> CircuitSegmentConnectionState;

    ///Looks at recent messages to verify that the circuit state matches
    fn update_connected_state(&mut self) -> CircuitSegmentConnectionState;

    ///Issues the sequence of controls necessary to put the circuit in connected state and sets the state to Connecting
    fn connect(&mut self) -> Result<CircuitSegmentConnectionState, CircuitSegmentError>;

    ///Issues the sequence of controls necessary to put the circuit in disconnected state and sets the state to Disconnecting
    fn disconnect(&mut self) -> Result<CircuitSegmentConnectionState, CircuitSegmentError>;
}
