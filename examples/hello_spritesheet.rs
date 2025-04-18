use citro_2d::color::Color;
use citro_2d::render_target::{RenderTarget, Screen};
use citro_2d::Citro2d;
use ctru::prelude::*;
use ctru::services::romfs::RomFS;
use citro_2d::spritesheet::Spritesheet;

macro_rules! ok_or_bail {
    ($resul:expr, $gfx:ident, $hid:ident, $apt:ident) => {
        ok_or_bail(
            $resul,
            stringify!($resul),
            &$gfx,
            &mut $hid,
            &$apt,
        )
    };
}

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.bottom_screen.borrow_mut());
    let _romfs = ok_or_bail!(RomFS::new(), gfx, hid, apt);

    let citro_2d = ok_or_bail!(Citro2d::new(), gfx, hid, apt);

    let mut top_screen = ok_or_bail!(RenderTarget::new(Screen::TopLeft, &citro_2d), gfx, hid, apt);
    let mut spritesheet = ok_or_bail!(Spritesheet::new("romfs:/gfx/image.t3x"), gfx, hid, apt);
    spritesheet.set_center(0.5, 0.5);

    let mut x : f32 = 0.0;
    let mut y : f32 = 0.0;

    while apt.main_loop() {
        const SPEED: f32 = 100.0;
        const DELTA_TIME: f32 = 1.0/60.0;

        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if hid.keys_held().contains(KeyPad::DPAD_LEFT) {
            x -= SPEED * DELTA_TIME;
        }
        if hid.keys_held().contains(KeyPad::DPAD_RIGHT) {
            x += SPEED * DELTA_TIME;
        }

        if hid.keys_held().contains(KeyPad::DPAD_UP) {
            y -= SPEED * DELTA_TIME;
        }
        if hid.keys_held().contains(KeyPad::DPAD_DOWN) {
            y += SPEED * DELTA_TIME;
        }

        spritesheet.set_position(x, y);
        println!("\x1b[2;0HPosition: ({}, {})                                              ", x, y);

        {
            let _frame = citro_2d.frame();
            top_screen.clear(Color::WHITE);
            top_screen.scene_begin();

            spritesheet.draw();
        }

        gfx.wait_for_vblank();
    }
}

fn ok_or_bail<TOk, TErr: std::error::Error>(result: Result<TOk, TErr>, call: &str, gfx: &Gfx, hid: &mut Hid, apt: &Apt) -> TOk {
    match result {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error in call \"{}\": \n{}", call, err);
            while apt.main_loop() {
                hid.scan_input();

                if hid.keys_down().contains(KeyPad::START) {
                    break;
                }

                gfx.wait_for_vblank();
            }
            panic!();
        }
    }
}