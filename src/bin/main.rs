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

use thrust_ui::input::{Cursor, Event, EventKind, MouseButton};
use thrust_ui::widget_tree::WidgetTree;
use thrust_ui::widget::{EmptyWidget, Button, Renderer};

struct DefaultRenderer { }

impl Renderer for DefaultRenderer {
    fn render(&mut self) { }
}

fn main() {
    let mut tree = WidgetTree::new(EmptyWidget::new());
    let mut renderer = DefaultRenderer { };

    let root = tree.root().index();
    let mut button1 = Button::new("button1".to_string());
    button1.add_callback(Box::new(|button| {
        println!("Hello, world: {}", button.text());
    }));
    let button2 = Button::new("button2".to_string());

    tree.add_child(root, button1);
    tree.add_child(root, button2);

    let evt = Event {
        kind: EventKind::MousePressed { button: MouseButton::Left },
        cursor: Cursor { x: 0.0, y: 0.0 }
    };

    tree.draw(&mut renderer);

    tree.handle_event(evt);
}
