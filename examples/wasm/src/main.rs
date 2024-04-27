use bevy::prelude::*;
use bevy_glicol::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GlicolPlugin))
        .insert_resource(Code::default())
        .add_systems(Update, play_tone)
        .run()
}

#[derive(Resource)]
pub struct Code {
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub mixer: String,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            t1: "~t1: speed 4.0 >> seq 60 >> bd 0.1".to_string(),
            t2: "~t2: seq 33_33_ _33 33__33 _33 >> sawsynth 0.01 0.1 >> mul 0.5".to_string(),
            t3: "~t3: speed 4.0 >> seq 60 61 61 63 >> hh 0.02 >> mul 0.4".to_string(),
            mixer: "out: mix ~t.. >> plate 0.1".to_string(),
        }
    }
}

fn play_tone(
    engine: ResMut<GlicolEngine>,
    mut code: ResMut<Code>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::Digit1) {
        // toggle t1 with // or remove //
        code.t1 = if code.t1.starts_with("//") {
            code.t1[2..].to_string()
        } else {
            format!("//{}", code.t1)
        };
    }
    if input.pressed(KeyCode::Digit2) {
        code.t2 = if code.t2.starts_with("//") {
            code.t2[2..].to_string()
        } else {
            format!("//{}", code.t2)
        };
    }
    if input.pressed(KeyCode::Digit3) {
        code.t3 = if code.t3.starts_with("//") {
            code.t3[2..].to_string()
        } else {
            format!("//{}", code.t3)
        };
    }
    let c = format!(
        "{}\n\n{}\n\n{}\n\n{}",
        code.t1, code.t2, code.t3, code.mixer
    );
    info!("code updated to: \n\n{}\n\n will update on next bar", c);
    engine.update_with_code(&c);
}
