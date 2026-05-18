use truce::prelude::*;
mod drums;
mod editor;
mod events;
mod params;
mod plugin;
mod utils;
pub use drums::*;
pub use params::*;

truce::plugin! {
    logic: XrossDrumsEmulator,
    params: XrossDrumsEmulatorParams,
}
