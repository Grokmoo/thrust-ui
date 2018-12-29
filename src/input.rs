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

#[derive(Debug, Clone)]
pub struct Event {
    pub kind: EventKind,
    pub cursor: Cursor,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    MouseMoved { delta_x: f32, delta_y: f32 },
    MousePressed { button: MouseButton },
    MouseReleased { button: MouseButton},
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
