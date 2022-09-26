use std::time::Instant;

use clap::Parser;

mod peripherals;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Name of the weapon (either Vandal or Phantom) to use
    #[clap(short, long, value_parser)]
    weapon_name: String,

    /// Number of bullets to shoot per burst
    #[clap(short, long, value_parser, default_value_t = 2)]
    num_bullets: u8,
}

fn main() {
    let args = Args::parse();

    let fire_rate = match args.weapon_name.trim().to_lowercase().as_str() {
        "vandal" => 9.75f32,
        "phantom" => 11.0f32,
        _ => todo!(),
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
