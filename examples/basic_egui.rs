use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_glicol::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GlicolPlugin))
        .insert_resource(AudioState {
            amplitude: 0.09,
            freq: 440.0,
            mute: true,
        })
        .add_plugins(EguiPlugin)
        .add_systems(Update, play_tone)
        .add_systems(Update, audio_control_ui)
        .run()
}

#[derive(Default, Resource)]
struct AudioState {
    pub amplitude: f32,
    pub freq: f32,
    pub mute: bool,
}

#[derive(Event)]
struct PlayTone;

fn play_tone(engine: Res<GlicolEngine>, audio_state: ResMut<AudioState>) {
    let freq = if audio_state.mute {
        0.0
    } else {
        audio_state.freq
    };
    let amplitude = audio_state.amplitude;
    engine.update_with_code(&format!("o: sin {freq} >> mul {amplitude}"));
}

fn audio_control_ui(mut contexts: EguiContexts, mut audio_state: ResMut<AudioState>) {
    egui::Window::new("Audio Control").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut audio_state.amplitude, 0.0..=1.0).text("Amp"));
        ui.add(
            egui::Slider::new(&mut audio_state.freq, 10.0..=10000.0)
                .logarithmic(true)
                .text("Freq"),
        );

        if audio_state.mute {
            if ui.button("Unmute").clicked() {
                audio_state.mute = false;
            }
        } else {
            if ui.button("Mute").clicked() {
                audio_state.mute = true;
            }
        }
    });
}
