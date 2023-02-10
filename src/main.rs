use std::time::Instant;

use clap::{Parser, ValueEnum};

mod peripherals;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the weapon to use
    #[clap(short, long, arg_enum, value_parser)]
    weapon_name: Weapon,

    /// Number of bullets to shoot per burst
    #[clap(short, long, value_parser, default_value_t = 2)]
    num_bullets: u8,
}

#[derive(ValueEnum, Clone, Debug)]
enum Weapon {
    Vandal,
    Phantom,
}

fn main() {
    let args = Args::parse();

    let fire_rate = match args.weapon_name {
        Weapon::Vandal => 9.75,
        Weapon::Phantom => 11.0,
    };
    let num_bullets = args.num_bullets;

    let sleep_time = (num_bullets as f32 / fire_rate) * 0.75;

    loop {
        let start = Instant::now();
        let mut mouse_down = peripherals::is_mouse_clicked();

        'mouse: while mouse_down {
            let time_passed = start.elapsed();
            if time_passed.as_secs_f32() >= sleep_time {
                peripherals::unclick_mouse();
                break 'mouse;
            }

            mouse_down = peripherals::is_mouse_clicked();
        }
    }
}
