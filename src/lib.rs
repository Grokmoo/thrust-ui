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

#[macro_export]
macro_rules! widget {
    ($(#[$attr:meta])* pub struct $name:ident { $($id:ident: $ty:ty),* }
     $($fn_data:tt)*) => {
        $(#[$attr])*
        pub struct $name {
            state: WidgetState,
            $( $id: $ty ),*
        }

        impl Widget for $name {
            fn state(&self) -> &WidgetState { &self.state }

            fn state_mut(&mut self) -> &mut WidgetState { &mut self.state }

            fn kind(&self) -> &'static str { stringify!($name) }

            fn as_any(&self) -> &Any { self }

            fn as_any_mut(&mut self) -> &mut Any { self }

            $($fn_data)*
        }

        impl $name {
            pub fn set_theme<S: Into<String>>(&mut self, id: S) {
                let id = id.into();
                self.state_mut().theme_partial_id = id.clone();
                self.state_mut().theme_full_id = id;
            }
        }
    }
}

pub mod widget;
pub mod button;
pub mod color;
pub mod image;
pub mod input;
pub mod label;
pub mod theme;
pub mod theme_builder;
pub mod widget_tree;
