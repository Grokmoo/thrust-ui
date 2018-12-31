//  This file is part of thrust-ui, the themable UI tooklit written in Rust.
//  Copyright 2018/2019 Jared Stephen
//
//  thrust-ui is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  thrust-ui is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with thrust-ui.  If not, see <http://www.gnu.org/licenses/>

use std::io::{Error, ErrorKind};
use std::str::FromStr;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields, default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

fn get_component(text: &str, max: f32) -> Result<f32, Error> {
    let component = i32::from_str_radix(&text, 16);
    match component {
        Err(_) => Err(Error::new(ErrorKind::InvalidInput,
                                 format!("Unable to parse color component from '{}'", text))),
        Ok(c) => Ok(c as f32 / max),
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.len() == 3 || text.len() == 4 {
            let r = get_component(&text[0..1], 16.0)?;
            let g = get_component(&text[1..2], 16.0)?;
            let b = get_component(&text[2..3], 16.0)?;
            let a = if text.len() == 4 {
                get_component(&text[3..4], 16.0)?
            } else {
                1.0
            };

            Ok(Color { r, g, b, a })
        } else if text.len() == 6 || text.len() == 8 {
            let r = get_component(&text[0..2], 255.0)?;
            let g = get_component(&text[2..4], 255.0)?;
            let b = get_component(&text[4..6], 255.0)?;
            let a = if text.len() == 8 {
                get_component(&text[6..8], 255.0)?
            } else {
                1.0
            };

            Ok(Color { r, g, b, a })
        } else {
            Err(Error::new(ErrorKind::InvalidInput,
                           format!("Unable to parse color from string '{}'", text)))
        }
    }
}
