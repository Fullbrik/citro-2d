use std::ptr::null_mut;
use std::rc::Rc;
use thiserror::Error;

pub mod render_target;
pub mod color;
pub mod spritesheet;

/// An instance of the Citro2d library
///
/// Only create one of these
///
/// The Rc is to enforce the lifetime. We only deinit the lib once all resources are freed
pub struct Citro2d(Rc<Citro2dInner>);

impl Citro2d {
    pub fn new() -> Result<Self, Citro2dError> {
        unsafe {
            Ok(Self(Rc::new(Citro2dInner::new()?)))
        }
    }

    pub(crate) fn inner(&self) -> Rc<Citro2dInner> {
        self.0.clone()
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self)
    }
}

pub(crate) struct Citro2dInner;

impl Citro2dInner {
    fn new() -> Result<Self, Citro2dError> {
        unsafe {
            if !citro_2d_sys::C3D_Init(citro_2d_sys::C3D_DEFAULT_CMDBUF_SIZE as _) {
                return Err(Citro2dError::Citro3dInitFailed);
            }
            if !citro_2d_sys::C2D_Init(citro_2d_sys::C2D_DEFAULT_MAX_OBJECTS as _) {
                return Err(Citro2dError::Citro2dInitFailed);
            }
            citro_2d_sys::C2D_Prepare();

            Ok(Self)
        }
    }
}

impl Drop for Citro2dInner {
    fn drop(&mut self) {
        unsafe {
            citro_2d_sys::C2D_Fini();
            citro_2d_sys::C3D_Fini();
        }
    }
}

pub struct Frame<'a>(&'a Citro2d);

impl<'a> Frame<'a> {
    fn new(citro2d: &'a Citro2d) -> Self {
        unsafe {
            citro_2d_sys::C3D_FrameBegin(citro_2d_sys::C3D_FRAME_SYNCDRAW);
        }

        Self(citro2d)
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        unsafe {
            citro_2d_sys::C3D_FrameEnd(0);
        }
    }
}

#[derive(Debug, Error)]
pub enum Citro2dError {
    #[error("Failed to init Citro3d")]
    Citro3dInitFailed,
    #[error("Failed to init Citro2d")]
    Citro2dInitFailed,
}