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

use std::any::Any;

use crate::widget::{Widget, WidgetState};
use crate::widget::Renderer;

widget! {
    #[derive(Default)]
    pub struct Label {
        text: String
    }

    fn draw(&self, renderer: &mut Renderer) {
        self.state().draw(renderer);
        println!("{}", self.text);
    }
}

impl Label {
    pub fn new(text: String) -> Label {
        let mut label= Label {
            text,
            ..Default::default()
        };

        label.set_theme("label");

        label
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
