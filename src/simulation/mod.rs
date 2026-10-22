pub mod budget;
pub mod clock;
pub mod debug;
pub mod entity;
pub mod messages;
pub mod plugin;
pub mod time_jump;
pub mod updates;

pub use budget::{SimulationUpdateBudget, schedule_updates};
pub use clock::{SimClock, TimeJumpRequest, update_sim_clock};
pub use debug::debug_time_controls;
pub use entity::{LODLevel, LastUpdate, Location, Npc, SimulationUpdateTag};
pub use messages::{SimulationRequest, handle_simulation_requests};
pub use plugin::SimulationPlugin;
pub use time_jump::{
    TimeJumpProgress, TimeJumpState, advance_time_jump, finish_time_jump, handle_time_jump_requests,
};
pub use updates::{apply_location_updates, apply_npc_updates, cleanup_unknown_simulation_updates};
