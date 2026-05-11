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
    bus_layouts: [
        BusLayout::new()
            .with_output("Main", ChannelConfig::Stereo)
    ],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_runs() {
        let result = truce_test::render_effect::<Plugin>(512, 44100.0);
        truce_test::assert_no_nans(&result.output);
    }

    #[test]
    fn renders_nonzero_output() {
        let result = truce_test::render_instrument::<Plugin>(
            512,
            44100.0,
            &[Event {
                sample_offset: 0,
                body: EventBody::NoteOn {
                    channel: 0,
                    note: 36,
                    velocity: 128.0,
                },
            }],
        );
        truce_test::assert_nonzero(&result.output);
    }

    #[test]
    fn bus_config_instrument() {
        truce_test::assert_bus_config_instrument::<Plugin>();
    }
    #[test]
    fn info_is_valid() {
        truce_test::assert_valid_info::<Plugin>();
    }

    #[test]
    fn has_editor() {
        truce_test::assert_has_editor::<Plugin>();
    }

    #[test]
    fn state_round_trips() {
        truce_test::assert_state_round_trip::<Plugin>();
    }

    #[test]
    fn param_defaults_match() {
        truce_test::assert_param_defaults_match::<Plugin>();
    }

    #[test]
    fn no_duplicate_param_ids() {
        truce_test::assert_no_duplicate_param_ids::<Plugin>();
    }

    #[test]
    fn corrupt_state_no_crash() {
        truce_test::assert_corrupt_state_no_crash::<Plugin>();
    }

    #[test]
    fn param_normalized_clamped() {
        truce_test::assert_param_normalized_clamped::<Plugin>();
    }
}
