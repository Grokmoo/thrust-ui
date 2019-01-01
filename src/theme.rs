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

impl Default for HorizontalAlignment {
    fn default() -> Self { HorizontalAlignment::Left }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self { VerticalAlignment::Center }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum SizeRelative {
    Zero,
    Parent,
    ChildMax,
    ChildSum,
    Custom,
}

impl Default for SizeRelative {
    fn default() -> Self {
        SizeRelative::Zero
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

impl Default for PositionRelative {
    fn default() -> Self { PositionRelative::Zero }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub enum LayoutKind {
    Normal,
    BoxVertical,
    BoxHorizontal,
    Grid,
}

impl Default for LayoutKind {
    fn default() -> Self { LayoutKind::Normal }
}

#[derive(Debug, Clone, Copy)]
pub struct Relative {
    pub x: PositionRelative,
    pub y: PositionRelative,
    pub width: SizeRelative,
    pub height: SizeRelative,
}

#[derive(Debug, Clone)]
pub struct TextParams {
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,

    pub color: Color,
    pub scale: f32,
    pub font: String,
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

pub const DEFAULT_THEME_ID: &'static str = "default";

#[derive(Debug)]
pub struct Theme {
    pub id: String,
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
            id: DEFAULT_THEME_ID.to_string(),
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

#[derive(Debug)]
pub struct ThemeSet {
    themes: HashMap<String, Theme>,
}

impl ThemeSet {
    pub(crate) fn new(themes: HashMap<String, Theme>) -> ThemeSet {
        ThemeSet {
            themes,
        }
    }
}
