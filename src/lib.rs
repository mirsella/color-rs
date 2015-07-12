// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate num;
extern crate angle;

pub use alpha::AlphaColor;
pub use alpha::{Rgba, Hsva, Srgba, YCbCra, ToRgba};
pub use channel::{Channel, FloatChannel};
pub use hsv::{Hsv, ToHsv};
pub use rgb::{Rgb, Rg, ToRgb, consts};
pub use srgb::Srgb;
pub use ycbcr::YCbCr;
pub use angle::Deg;

#[macro_use] mod rgb;
#[macro_use] mod alpha;
mod channel;
mod hsv;
mod srgb;
mod ycbcr;

pub trait Color<T>: Copy {
    fn clamp_s(self, lo: T, hi: T) -> Self;
    fn clamp_c(self, lo: Self, hi: Self) -> Self;
    fn inverse(self) -> Self;
    fn mix(self, other: Self, value: T) -> Self;
    // fn saturation(&self, value: T) -> Self;
    // fn exposure(&self, value: T) -> Self;
    // fn brightness(&self, value: T) -> Self;
}

pub trait FloatColor<T>: Color<T> {
    fn saturate(self) -> Self;
}

