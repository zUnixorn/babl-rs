// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "BablIccIntent")]
pub enum IccIntent {
    #[doc(alias = "BABL_ICC_INTENT_PERCEPTUAL")]
    Perceptual,
    #[doc(alias = "BABL_ICC_INTENT_RELATIVE_COLORIMETRIC")]
    RelativeColorimetric,
    #[doc(alias = "BABL_ICC_INTENT_SATURATION")]
    Saturation,
    #[doc(alias = "BABL_ICC_INTENT_ABSOLUTE_COLORIMETRIC")]
    AbsoluteColorimetric,
    #[doc(alias = "BABL_ICC_INTENT_PERFORMANCE")]
    Performance,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for IccIntent {
    type GlibType = ffi::BablIccIntent;

    #[inline]
fn into_glib(self) -> ffi::BablIccIntent {
match self {
            Self::Perceptual => ffi::BABL_ICC_INTENT_PERCEPTUAL,
            Self::RelativeColorimetric => ffi::BABL_ICC_INTENT_RELATIVE_COLORIMETRIC,
            Self::Saturation => ffi::BABL_ICC_INTENT_SATURATION,
            Self::AbsoluteColorimetric => ffi::BABL_ICC_INTENT_ABSOLUTE_COLORIMETRIC,
            Self::Performance => ffi::BABL_ICC_INTENT_PERFORMANCE,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::BablIccIntent> for IccIntent {
    #[inline]
unsafe fn from_glib(value: ffi::BablIccIntent) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::BABL_ICC_INTENT_PERCEPTUAL => Self::Perceptual,
            ffi::BABL_ICC_INTENT_RELATIVE_COLORIMETRIC => Self::RelativeColorimetric,
            ffi::BABL_ICC_INTENT_SATURATION => Self::Saturation,
            ffi::BABL_ICC_INTENT_ABSOLUTE_COLORIMETRIC => Self::AbsoluteColorimetric,
            ffi::BABL_ICC_INTENT_PERFORMANCE => Self::Performance,
            value => Self::__Unknown(value),
}
}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "BablSpaceFlags")]
pub enum SpaceFlags {
    #[doc(alias = "BABL_SPACE_FLAG_NONE")]
    None,
    #[doc(alias = "BABL_SPACE_FLAG_EQUALIZE")]
    Equalize,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for SpaceFlags {
    type GlibType = ffi::BablSpaceFlags;

    #[inline]
fn into_glib(self) -> ffi::BablSpaceFlags {
match self {
            Self::None => ffi::BABL_SPACE_FLAG_NONE,
            Self::Equalize => ffi::BABL_SPACE_FLAG_EQUALIZE,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::BablSpaceFlags> for SpaceFlags {
    #[inline]
unsafe fn from_glib(value: ffi::BablSpaceFlags) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::BABL_SPACE_FLAG_NONE => Self::None,
            ffi::BABL_SPACE_FLAG_EQUALIZE => Self::Equalize,
            value => Self::__Unknown(value),
}
}
}

