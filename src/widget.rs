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

use crate::image::Image;
use crate::input::{Cursor, MouseButton};

pub struct Callback<T> {
    callback: Box<Fn(&mut T)>,
}

impl<T> Callback<T> {
    pub fn new(cb: Box<Fn(&mut T)>) -> Callback<T> {
        Callback {
            callback: cb
        }
    }

    pub fn fire(&self, arg: &mut T) {
        (self.callback)(arg)
    }
}

pub trait Renderer {
    fn render(&mut self);
}

#[derive(Default)]
pub struct WidgetState {
    position: Point,
    size: Size,
    background: Image,

    pub(crate) index: usize,
    pub(crate) to_add: Vec<Box<dyn Widget>>,
}

impl WidgetState {
    pub(crate) fn draw(&self, renderer: &mut Renderer) {
        self.background.draw(renderer, self.position, self.size);
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

    fn mouse_pressed(&mut self, _button: MouseButton) -> bool { false }

    fn mouse_released(&mut self, _button: MouseButton) -> bool { false }

    fn mouse_moved(&mut self, _x: f32, _y: f32) -> bool { false }

    fn mouse_entered(&mut self) -> bool { false }

    fn mouse_exited(&mut self) -> bool { false }
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

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Clone, Copy)]
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
        text: String,
        callbacks: Vec<Callback<Button>>
    }

    fn on_add(&mut self) {}

    fn draw(&self, renderer: &mut Renderer) {
        self.state().draw(renderer);
        println!("{}", self.text);
    }

    fn mouse_pressed(&mut self, _button: MouseButton) -> bool{
        let mut callbacks: Vec<_> = self.callbacks.drain(..).collect();

        for cb in callbacks.iter() {
            cb.fire(self);
        }

        self.callbacks.append(&mut callbacks);

        true
    }
}

impl Button {
    pub fn new(text: String) -> Button {
        Button {
            text,
            ..Default::default()
        }
    }

    pub fn add_callback(&mut self, cb: Box<Fn(&mut Button)>) {
        self.callbacks.push(Callback::new(cb));
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
