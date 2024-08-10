use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};

mod peripherals;

#[derive(Parser)]
struct Args {
    /// The weapon to use
    #[arg(short, long, value_enum, default_value_t = Weapon::Vandal)]
    weapon: Weapon,

    /// Number of bullets to shoot per burst
    #[arg(short, long, default_value_t = 2)]
    bullet_count: u8,

    /// Scales how much time to wait before releasing the mouse button (not necessary to tweak unless you are shooting too many/few bullets)
    #[arg(short, long, default_value_t = 0.8)]
    compensation_scale: f32,
}

#[derive(Debug, Clone, ValueEnum)]
enum Weapon {
    Vandal,
    Phantom,
}

impl Weapon {
    const fn fire_rate(&self) -> f32 {
        match self {
            Weapon::Vandal => 9.75,
            Weapon::Phantom => 11.0,
        }
    }
}

fn main() {
    let Args {
        weapon,
        bullet_count,
        compensation_scale,
    } = Args::parse();

    let sleep_duration =
        Duration::from_secs_f32((bullet_count as f32 / weapon.fire_rate()) * compensation_scale);

    println!("VALORANT Tap Fire Enforcer v{}", env!("CARGO_PKG_VERSION"));
    println!("{weapon:?} - {bullet_count} Bullets");

    loop {
        let start = Instant::now();

        'mouse: while peripherals::is_mouse_clicked() {
            if start.elapsed() >= sleep_duration {
                peripherals::unclick_mouse();
                break 'mouse;
            }
        }
    }
}
