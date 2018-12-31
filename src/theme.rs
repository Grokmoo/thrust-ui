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

use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize};

use crate::color::Color;
use crate::widget::{Size, Point};

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum SizeRelative {
    Zero,
    Max,
    ChildMax,
    ChildSum,
    Custom,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum LayoutKind {
    Normal,
    BoxVertical,
    BoxHorizontal,
    Grid,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct TextParams {
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,

    #[serde(deserialize_with="de_color")]
    pub color: Color,

    pub scale: f32,
    pub font: String,
}

fn de_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where D:Deserializer<'de> {
    let input = String::deserialize(deserializer)?;

    use serde::de::Error;
    Color::from_str(&input).map_err(|err| Error::custom(err.to_string()))
}

impl Default for TextParams {
    fn default() -> Self {
        TextParams {
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Center,
            color: Color::default(),
            scale: 1.0,
            font: "Default".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum PositionRelative {
    Zero,
    Center,
    Max,
    Custom,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields, default)]
pub struct Relative {
    x: PositionRelative,
    y: PositionRelative,
    width: SizeRelative,
    height: SizeRelative,
}

impl Default for Relative {
    fn default() -> Self {
        Relative {
            x: PositionRelative::Zero,
            y: PositionRelative::Zero,
            width: SizeRelative::Zero,
            height: SizeRelative::Zero,
        }
    }
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
pub struct Border {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

#[derive(Deserialize, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub name: String,
    pub layout: LayoutKind,
    pub layout_spacing: Border,
    pub border: Border,
    pub size: Size,
    pub position: Point,
    pub relative: Relative,

    pub text: Option<String>,
    pub text_params: TextParams,
    pub background: Option<String>,
    pub foreground: Option<String>,

    pub custom: HashMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "Default".to_string(),
            layout: LayoutKind::Normal,
            layout_spacing: Border::default(),
            border: Border::default(),
            size: Size::default(),
            position: Point::default(),
            relative: Relative::default(),
            text: None,
            text_params: TextParams::default(),
            background: None,
            foreground: None,
            custom: HashMap::default(),
        }
    }
}
