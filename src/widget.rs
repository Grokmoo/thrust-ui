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
use std::rc::Rc;

use serde_derive::Deserialize;

use crate::widget_tree::WidgetTree;
use crate::theme::DEFAULT_THEME_ID;
use crate::image::Image;
use crate::input::{Cursor, MouseButton};

#[derive(Clone)]
pub struct Callback<T> {
    callback: Rc<Fn(&mut WidgetTree, usize, T) -> bool>,
}

impl<T> Callback<T> {
    pub fn new(cb: Rc<Fn(&mut WidgetTree, usize, T) -> bool>) -> Callback<T> {
        Callback {
            callback: cb
        }
    }

    pub fn fire(&self, tree: &mut WidgetTree, index: usize, arg: T) -> bool {
        (self.callback)(tree, index, arg)
    }
}

impl<T> Default for Callback<T> {
    fn default() -> Callback<T> {
        Callback {
            callback: Rc::new(|_, _, _| true),
        }
    }
}

pub trait Renderer {
    fn render(&mut self);
}

pub struct WidgetState {
    theme_id: String,
    position: Point,
    size: Size,
    background: Image,
    foreground: Image,

    pub(crate) mouse_pressed_callback: Callback<MouseButton>,
    pub(crate) mouse_released_callback: Callback<MouseButton>,
    pub(crate) mouse_moved_callback: Callback<(f32, f32)>,
    pub(crate) mouse_entered_callback: Callback<()>,
    pub(crate) mouse_exited_callback: Callback<()>,

    pub(crate) index: usize,
    pub(crate) to_add: Vec<Box<dyn Widget>>,
}

impl Default for WidgetState {
    fn default() -> Self {
        WidgetState {
            theme_id: DEFAULT_THEME_ID.to_string(),
            position: Point::default(),
            size: Size::default(),
            background: Image::default(),
            foreground: Image::default(),

            mouse_pressed_callback: Callback::default(),
            mouse_released_callback: Callback::default(),
            mouse_moved_callback: Callback::default(),
            mouse_entered_callback: Callback::default(),
            mouse_exited_callback: Callback::default(),
            index: 0,
            to_add: Vec::default(),
        }
    }
}

impl WidgetState {
    pub fn set_mouse_pressed_callback(&mut self, callback:
                                      Rc<Fn(&mut WidgetTree, usize, MouseButton) -> bool>) {
        self.mouse_pressed_callback = Callback::new(callback);
    }

    pub fn set_mouse_released_callback(&mut self, callback:
                                       Rc<Fn(&mut WidgetTree, usize, MouseButton) -> bool>) {
        self.mouse_released_callback = Callback::new(callback);
    }

    pub fn set_mouse_moved_callback(&mut self, callback:
                                    Rc<Fn(&mut WidgetTree, usize, (f32, f32)) -> bool>) {
        self.mouse_moved_callback = Callback::new(callback);
    }

    pub fn set_mouse_entered_callback(&mut self, callback:
                                      Rc<Fn(&mut WidgetTree, usize, ()) -> bool>) {
        self.mouse_entered_callback = Callback::new(callback);
    }

    pub fn set_mouse_exited_callback(&mut self, callback:
                                     Rc<Fn(&mut WidgetTree, usize, ()) -> bool>) {
        self.mouse_exited_callback = Callback::new(callback);
    }

    pub(crate) fn draw(&self, renderer: &mut Renderer) {
        self.background.draw(renderer, self.position, self.size);
        self.foreground.draw(renderer, self.position, self.size);
    }

    pub fn is_inside(&self, cursor: &Cursor) -> bool {
        if (cursor.x as i32) < self.position.x { return false; }
        if (cursor.y as i32) < self.position.y { return false; }

        if cursor.x.ceil() as i32 > self.position.x + self.size.width as i32 { return false; }
        if cursor.y.ceil() as i32 > self.position.y + self.size.height as i32 { return false; }

        true
    }
}

pub trait Widget {
    /// Called on each frame.  `elapsed_millis` is the number of milliseconds
    /// that have elapsed since the last frame
    fn update(&mut self, _elapsed_millis: u32) { }

    /// This method is called immediately after the widget is added to
    /// the overall tree.
    fn on_add(&mut self) { }

    /// Called immediately after the widget is removed from the overall
    /// widget tree.
    fn on_remove(&mut self) { }

    /// Called after the widget has been added to the tree and the theme
    /// has been applied.
    fn layout(&mut self) { }

    fn index(&self) -> usize { self.state().index }

    fn state(&self) -> &WidgetState;

    fn state_mut(&mut self) -> &mut WidgetState;

    fn kind(&self) -> &'static str;

    fn position(&self) -> Point { self.state().position }

    fn size(&self) -> Size { self.state().size }

    fn draw(&self, renderer: &mut Renderer) {
        self.state().draw(renderer);
    }

    fn theme_id(&self) -> &str { &self.state().theme_id }

    fn as_any(&self) -> &Any;

    fn as_any_mut(&mut self) -> &mut Any;
}

impl Widget {
    pub fn empty() -> impl Widget {
        EmptyWidget::default()
    }

    /// Adds the specified widget as a `child` of this widget.  It will be
    /// added to the tree when this widget is, if it is not already.
    pub fn add_child<T: Widget + 'static>(parent: &mut dyn Widget, child: T) {
        parent.state_mut().to_add.push(Box::new(child));
    }
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
#[serde(deny_unknown_fields, default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Default, Debug, Clone, Copy)]
#[serde(deny_unknown_fields, default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

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
    }
}

widget!{
    #[derive(Default)]
    pub struct EmptyWidget { }
}

impl EmptyWidget {
    pub fn new() -> EmptyWidget {
        EmptyWidget::default()
    }
}

widget!{
    #[derive(Default)]
    pub struct Button {
        text: String
    }

    fn on_add(&mut self) {}

    fn draw(&self, renderer: &mut Renderer) {
        self.state().draw(renderer);
        println!("{}", self.text);
    }
}

impl Button {
    pub fn new(text: String) -> Button {
        Button {
            text,
            ..Default::default()
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
