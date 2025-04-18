use std::ffi::CString;
use std::str::FromStr;
use thiserror::Error;

pub struct Spritesheet {
    spritesheet: citro_2d_sys::C2D_SpriteSheet,
    current_sprite: citro_2d_sys::C2D_Sprite,
}

impl Spritesheet {
    pub fn new(file_name: &str) -> Result<Self, CreateSpriteSheetError> {
        unsafe {
            let file_name_cstr = CString::from_str(file_name)
                .map_err(|_| CreateSpriteSheetError {
                    path: file_name.to_string(),
                })?;
            let spritesheet = citro_2d_sys::C2D_SpriteSheetLoad(file_name_cstr.as_ptr());

            if spritesheet.is_null() {
                return Err(CreateSpriteSheetError {
                    path: file_name.to_string(),
                });
            }

            let mut current_sprite : citro_2d_sys::C2D_Sprite = std::mem::zeroed();
            citro_2d_sys::C2D_SpriteFromSheet(&mut current_sprite as *mut citro_2d_sys::C2D_Sprite, spritesheet, 0);

            Ok(Self {
                spritesheet,
                current_sprite,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            citro_2d_sys::C2D_DrawSprite(&self.current_sprite as *const citro_2d_sys::C2D_Sprite);
        }
    }

    pub fn set_current_sprite(&mut self, index: usize) {
        unsafe { citro_2d_sys::C2D_SpriteFromSheet(&mut self.current_sprite as *mut citro_2d_sys::C2D_Sprite, self.spritesheet, index); }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        unsafe { citro_2d_sys::C2D_SpriteSetPos(&mut self.current_sprite as *mut citro_2d_sys::C2D_Sprite, x, y); }
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        unsafe { citro_2d_sys::C2D_SpriteSetCenter(&mut self.current_sprite as *mut citro_2d_sys::C2D_Sprite, x, y); }
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        unsafe { citro_2d_sys::C2D_SpriteSetScale(&mut self.current_sprite as *mut citro_2d_sys::C2D_Sprite, x, y); }
    }
}

impl Drop for Spritesheet {
    fn drop(&mut self) {
        unsafe { citro_2d_sys::C2D_SpriteSheetFree(self.spritesheet); }
    }
}

#[derive(Debug, Error)]
#[error("Failed to load spritesheet at path \"{path}\"")]
pub struct CreateSpriteSheetError {
    path: String,
}