use std::fmt::Display;
use std::ptr::NonNull;
use std::rc::Rc;
use thiserror::Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Screen {
    TopLeft,
    TopRight,
    Bottom,
}

pub struct RenderTarget {
    citro2d: Rc<super::Citro2dInner>,
    ptr: NonNull<citro_2d_sys::C3D_RenderTarget>,
}

impl RenderTarget {
    pub fn new(screen: Screen, citro2d: &super::Citro2d) -> Result<Self, CreateRenderTargetError> {
        unsafe {
            let ptr = match screen {
                Screen::TopLeft => citro_2d_sys::C2D_CreateScreenTarget(0, 0),
                Screen::TopRight => citro_2d_sys::C2D_CreateScreenTarget(0, 1),
                Screen::Bottom => citro_2d_sys::C2D_CreateScreenTarget(1, 0),
            };

            let ptr = NonNull::new(ptr)
                .ok_or(CreateRenderTargetError {
                    screen,
                })?;

            Ok(Self {
                citro2d: citro2d.inner(),
                ptr,
            })
        }
    }

    pub fn clear(&mut self, color: super::color::Color) {
        unsafe {
            citro_2d_sys::C2D_TargetClear(self.ptr.as_ptr(), color.as_u32());
        }
    }

    pub fn scene_begin(&mut self) {
        unsafe {
            citro_2d_sys::C2D_SceneBegin(self.ptr.as_ptr());
        }
    }
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        unsafe {
            citro_2d_sys::C3D_RenderTargetDelete(self.ptr.as_ptr());
        }
    }
}

#[derive(Debug, Error)]
#[error("Failed to create from buffer for screen {screen:?}")]
pub struct CreateRenderTargetError {
    screen: Screen,
}