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

use std::rc::Rc;

use crate::widget::{Point, Size, Renderer};

pub struct Image {
    source: Rc<ImageSource>,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            source: Rc::new(EmptyImage { }),
        }
    }
}

impl Image {
    pub fn new(source: Rc<ImageSource>) -> Image {
        Image {
            source: source,
        }
    }

    pub fn draw(&self, renderer: &mut Renderer, position: Point, size: Size) {
        self.source.draw(renderer, position, size);
    }
}

struct EmptyImage { }

impl ImageSource for EmptyImage { }

pub trait ImageSource {
    fn draw(&self, _renderer: &mut Renderer, _position: Point, _size: Size) { }
}
