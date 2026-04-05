use bevy::prelude::*;

#[derive(Message)]
pub struct WorldGeneratedEvent;

// #[derive(Message)]
// pub struct HexChangedEvent {
//     pub coordinates: HexCoordinates,
//     pub old_type: HexType,
//     pub new_type: HexType,
// }

// -----------------------------------------------------------------------------
// Событие применения настроек
// -----------------------------------------------------------------------------
#[derive(Message)]
pub struct ApplySettings;
