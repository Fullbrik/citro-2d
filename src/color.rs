pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub(crate) fn as_u32(&self) -> u32 {
        unsafe { citro_2d_sys::C2D_Color32f(self.r, self.g, self.b, self.a) }
    }
}

// pub trait DynColor {
//     fn as_u32(&self) -> u32;
// }
//
// pub type ColorU8 = Color<u8>;
// pub type ColorF32 = Color<f32>;
//
// #[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
// pub struct Color<T> {
//     pub r: T,
//     pub g: T,
//     pub b: T,
//     pub a: T,
// }
//
// impl<T> Color<T> {
//     pub const fn new(r: T, g: T, b: T, a: T) -> Self {
//         Self {
//             r,
//             g,
//             b,
//             a,
//         }
//     }
// }
//
// impl DynColor for ColorU8 {
//     fn as_u32(&self) -> u32 {
//         unsafe { citro_2d_sys::C2D_Color32(self.r, self.g, self.b, self.a) }
//     }
// }
//
// impl DynColor for ColorF32 {
//     fn as_u32(&self) -> u32 {
//         unsafe { citro_2d_sys::C2D_Color32f(self.r, self.g, self.b, self.a) }
//     }
// }
//
// trait ColorNumber {
//     const ZERO: Self;
//     const HALF: Self;
//     const FULL: Self;
// }
//
// impl ColorNumber for u8 {
//     const ZERO: Self = 0;
//     const HALF: Self = 127;
//     const FULL: Self = 255;
// }
//
// impl ColorNumber for f32 {
//     const ZERO: Self = 0.0;
//     const HALF: Self = 0.5;
//     const FULL: Self = 1.0;
// }
//
// impl<T: ColorNumber> Color<T> {
//     pub const TRANSPARENT: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
//     pub const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::FULL);
//     pub const WHITE: Self = Self::new(T::FULL, T::FULL, T::FULL, T::FULL);
//
//     pub const RED: Self = Self::new(T::FULL, T::ZERO, T::ZERO, T::FULL);
//     pub const GREEN: Self = Self::new(T::ZERO, T::FULL, T::ZERO, T::FULL);
//     pub const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::FULL, T::FULL);
// }