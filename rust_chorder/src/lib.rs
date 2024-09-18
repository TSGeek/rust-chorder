use nih_plug::prelude::*;
use std::sync::Arc;

struct RustChorder {
    params: Arc<RustChorderParams>,
}

#[derive(Default, Params)]
struct RustChorderParams {}

impl Default for RustChorder {
    fn default() -> Self {
        Self {
            params: Arc::new(RustChorderParams::default()),
        }
    }
}

impl Plugin for RustChorder {
    const NAME: &'static str = "Rust Chorder";
    const VENDOR: &'static str = "Martin GRUDLER";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "devel@grudler.eu";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");


    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    context.send_event(NoteEvent::NoteOn {
                        timing,
                        voice_id,
                        channel,
                        note,
                        velocity,
                    });
                    context.send_event(NoteEvent::NoteOn {
                        timing,
                        voice_id,
                        channel,
                        note: note + 4,
                        velocity,
                    });
                    context.send_event(NoteEvent::NoteOn {
                        timing,
                        voice_id,
                        channel,
                        note: note + 7,
                        velocity,
                    });
                }
                NoteEvent::NoteOff {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    context.send_event(NoteEvent::NoteOff {
                        timing,
                        voice_id,
                        channel,
                        note,
                        velocity,
                    });
                    context.send_event(NoteEvent::NoteOff {
                        timing,
                        voice_id,
                        channel,
                        note: note + 4,
                        velocity,
                    });
                    context.send_event(NoteEvent::NoteOff {
                        timing,
                        voice_id,
                        channel,
                        note: note + 7,
                        velocity,
                    });
                }
                _ => ()
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for RustChorder {
    const CLAP_ID: &'static str = "fr.tsgeek.rust-chorder";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple chorder in rust");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for RustChorder {
    const VST3_CLASS_ID: [u8; 16] = *b"Qta7Veip3NSaKvIs";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_clap!(RustChorder);
nih_export_vst3!(RustChorder);
