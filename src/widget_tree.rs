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

use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::input::{Event};
use crate::theme::{Theme, ThemeSet, Kind};
use crate::widget::{Renderer, Widget, EmptyWidget};
use crate::label::Label;

pub struct WidgetTree {
    widgets: Vec<Option<Box<dyn Widget>>>,
    tree: Vec<Option<TreeEntry>>,
    themes: ThemeSet,
}

#[derive(Debug)]
struct TreeEntry {
    parent: usize,
    children: Vec<usize>,
}

impl Index<usize> for WidgetTree {
    type Output = dyn Widget;

    fn index<'a>(&'a self, index: usize) -> &'a (dyn Widget + 'static) {
        self.widgets[index].as_ref().unwrap().deref()
    }
}

impl IndexMut<usize> for WidgetTree {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut (dyn Widget + 'static) {
        self.widgets[index].as_mut().unwrap().deref_mut()
    }
}

impl WidgetTree {
    pub fn new<T: Widget + 'static>(root: T, themes: ThemeSet) -> WidgetTree {
        let mut tree = WidgetTree {
            widgets: Vec::new(),
            tree: Vec::new(),
            themes,
        };

        let mut root = Box::new(root);
        root.state_mut().index = 0;
        tree.add_child_internal(0, 0, root);

        tree
    }

    pub fn theme(&self, id: &str) -> &Theme {
        self.themes.get(id)
    }

    /// Traverses the widget tree up, starting from the parent of `index` looking for a widget
    /// with the specified concrete type `T`, returning the first one found.  If no such
    /// widget is found, panics.  Panics if `index` is invalid
    pub fn parent_mut<'a, T: Widget + 'static>(&'a mut self, index: usize) -> &'a mut T {
        let mut index = self.tree(index).parent;
        loop {
            if let Some(_) = self[index].as_any_mut().downcast_mut::<T>() {
                break;
            }

            let new_index = self.tree(index).parent;

            if new_index == index {
                // this widget is its own parent, i.e. the root
                panic!();
            }
            index = new_index;
        }

        // TODO not sure why putting this directly in the loop causes a borrow checker problem
        let widget = &mut self[index];
        widget.as_any_mut().downcast_mut::<T>().unwrap()
    }

    /// Traverses the widget tree up, starting from the parent of `index` looking for a widget
    /// with the specified concrete type `T`, returning the first one found.  If no such
    /// widget is found, panics.  Panics if `index` is invalid
    pub fn parent<T: Widget + 'static>(&self, index: usize) -> &T {
        let mut index = self.tree(index).parent;
        loop {
            match self[index].as_any().downcast_ref::<T>() {
                None => (),
                Some(widget) => return widget,
            }

            let new_index = self.tree(index).parent;

            if new_index == index {
                // this widget is its own parent, i.e. the root
                panic!();
            }
            index = new_index;
        }
    }

    /// Returns the widget at the specified `index` downcast to the concrete type `T`.
    /// Panics if the widget is not this type, or if the index is invalid.
    pub fn widget_mut<T: Widget + 'static>(&mut self, index: usize) -> &mut T {
        match self[index].as_any_mut().downcast_mut::<T>() {
            None => panic!(),
            Some(widget) => widget,
        }
    }

    /// Returns the widget at the specified `index` downcast to the concrete type `T`.
    /// Panics if the widget is not this type, or if the index is invalid.
    pub fn widget<T: Widget + 'static>(&self, index: usize) -> &T {
        match self[index].as_any().downcast_ref::<T>() {
            None => panic!(),
            Some(widget) => widget,
        }
    }

    pub fn root(&self) -> &dyn Widget {
        self.widgets[0].as_ref().unwrap().deref()
    }

    fn tree(&self, index: usize) -> &TreeEntry {
        self.tree[index].as_ref().unwrap()
    }

    pub fn handle_input(&mut self, input: &fn() -> Option<Event>) {
        loop {
            let event = match input() {
                None => break,
                Some(event) => event,
            };

            self.dispatch_event(&event, 0);
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.dispatch_event(&event, 0);
    }

    fn dispatch_event(&mut self, event: &Event, index: usize) -> bool {
        if self[index].state().is_inside(&event.cursor) {
            let len = self.tree(index).children.len();
            for i in 0..len {
                let child_index = self.tree(index).children[i];
                if self.dispatch_event(event, child_index) {
                    return true;
                }
            }

            if self.fire_event(index, event) {
                return true;
            }
        }

        false
    }

    fn fire_event(&mut self, index: usize, event: &Event) -> bool {
        use crate::input::EventKind::*;
        match &event.kind {
            MouseMoved { delta_x, delta_y } => {
                let cb = self[index].state().mouse_moved_callback.clone();
                cb.fire(self, index, (*delta_x, *delta_y))
            },
            MousePressed { button } => {
                let cb = self[index].state().mouse_pressed_callback.clone();
                cb.fire(self, index, *button)
            },
            MouseReleased { button } => {
                let cb = self[index].state().mouse_released_callback.clone();
                cb.fire(self, index, *button)
            }
        }
    }

    pub fn add_child<T: Widget + 'static>(&mut self, parent_index: usize, child: T) {
        self.add_child_boxed(parent_index, Box::new(child));
    }

    // Finds the parent index for the specified child inside the specified parent.  As the
    // theme can define widgets that we have not defined in code, this may not be the parent
    // directly, but instead one of its children
    fn find_theme_parent_index(&self, parent_index: usize,
                               child: &mut dyn Widget) -> Option<usize> {
        if parent_index >= self.widgets.len() {
            // TODO error message
            panic!();
        }

        let parent = match &self.widgets[parent_index] {
            None => {
                // TODO error message for invalid parent
                panic!();
            }, Some(widget) => widget,
        };

        let full_id = format!("{}.{}", parent.theme_id(), child.theme_partial_id());
        if self.themes.contains(&full_id) {
            println!("Set full theme id {}", full_id);
            child.set_full_theme_id(full_id);
            return Some(parent_index);
        }

        for index in self.tree[parent_index].as_ref().unwrap().children.iter() {
            if let Some(parent_index) = self.find_theme_parent_index(*index, child) {
                return Some(parent_index);
            }
        }

        None
    }

    fn add_child_boxed(&mut self, mut parent_index: usize, mut child: Box<dyn Widget>) {
        match self.find_theme_parent_index(parent_index, child.deref_mut()) {
            None => {
                println!("Unable to find valid theme for {}", child.theme_id());
                // TODO warn for no valid parent, but continue with default
            }, Some(index) => parent_index = index,
        }

        self.add_child_known_parent(parent_index, child);
    }

    fn add_child_known_parent(&mut self, parent_index: usize, mut child: Box<dyn Widget>) {
        let child_index = self.widgets.len();
        child.state_mut().index = child_index;
        self.tree[parent_index].as_mut().unwrap().children.push(child_index);
        self.add_child_internal(parent_index, child_index, child);
    }

    fn add_child_internal(&mut self, parent_index: usize, child_index: usize,
                          child: Box<dyn Widget>) {
        let child_theme_id = child.theme_id().to_string();
        println!("Add child '{}' to {}", child_theme_id, parent_index);

        self.widgets.push(Some(child));
        self.tree.push(Some(TreeEntry {
            parent: parent_index,
            children: Vec::new()
        }));

        self[child_index].on_add();

        // add custom children that have been added in code recursively
        let to_add: Vec<_> = self[child_index].state_mut().to_add.drain(..).collect();
        for child in to_add {
            self.add_child_boxed(child_index, child);
        }

        // add theme defined 'dumb' children recursively
        let children = self.themes.get(&child_theme_id).children.clone();
        for child_id in children {
            let theme = self.themes.get(&child_id);
            match theme.kind {
                Kind::Label => {
                    let mut label = Label::new("".to_string());
                    label.set_theme(theme.id.deref());
                    self.add_child_known_parent(parent_index, Box::new(label));
                },
                Kind::Container => {
                    let mut widget = EmptyWidget::new();
                    widget.set_theme(theme.id.deref());
                    self.add_child_known_parent(parent_index, Box::new(widget));
                },
                Kind::Ref => (),
            }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        for widget in self.iter(0) {
            widget.draw(renderer);
        }
    }

    fn check_index(&self, index: usize) {
        if index >= self.widgets.len() {
            // TODO error message
            panic!();
        }

        if self.widgets[index].is_none() {
            // TODO error message
            panic!();
        }
    }

    /// Iterates over widgets in this tree in graph traversal order (drawing order),
    /// starting from the specified root index.  Will panic if index is invalid.
    /// The root is drawn first, then its first child, its first child's children, then
    /// second child, and so on, recursively.
    /// `iter(0)` will iterate over all widgets
    pub fn iter<'a>(&'a self, root: usize) -> impl Iterator<Item=&'a dyn Widget> {
        self.check_index(root);

        WidgetIterator {
            tree: self,
            next: root,
            stack: Vec::new(),
        }
    }

    /// Iterates over widgets in this tree mutably in graph traversal order.  See `iter`
    pub fn iter_mut<'a>(&'a mut self, root: usize) -> impl Iterator<Item=&'a mut dyn Widget> {
        self.check_index(root);

        WidgetIteratorMut {
            tree: self,
            next: root,
            stack: Vec::new(),
        }
    }
}

macro_rules! widget_iter_next {
    ($iter:expr) => {
        {
            if $iter.next == $iter.tree.widgets.len() { return None; }

            let current = $iter.next;

            let mut entry = $iter.tree.tree($iter.next);
            if !entry.children.is_empty() {
                $iter.stack.push(1);
                $iter.next = entry.children[0];
            } else {
                loop {
                    if let Some(child_index) = $iter.stack.pop() {
                        entry = $iter.tree.tree(entry.parent);
                        if child_index == entry.children.len() { continue; }

                        $iter.next = entry.children[child_index];
                        $iter.stack.push(child_index + 1);
                        break;
                    } else {
                        // done
                        $iter.next = $iter.tree.widgets.len();
                        break;
                    }

                }
            }

            current
        }
    }
}

struct WidgetIterator<'a> {
    tree: &'a WidgetTree,
    next: usize,
    stack: Vec<usize>, // previous child positions in tree
}

impl<'a> Iterator for WidgetIterator<'a> {
    type Item = &'a dyn Widget;

    fn next(&mut self) -> Option<&'a dyn Widget> {
        let current = widget_iter_next!(self);
        Some(&self.tree[current])
    }
}

struct WidgetIteratorMut<'a> {
    tree: &'a mut WidgetTree,
    next: usize,
    stack: Vec<usize>, // previous child positions in tree
}

impl<'a> Iterator for WidgetIteratorMut<'a> {
    type Item = &'a mut dyn Widget;

    fn next<'b>(&'b mut self) -> Option<&'a mut dyn Widget> {
        let current = widget_iter_next!(self);

        let result = &mut self.tree[current];
        // Transmute the lifetime of the result to match the WidgetTree lifetime.
        // This should be safe because it should not be possible for repeated calls
        // to next to ever yield the same Widget twice
        unsafe {
            Some(std::mem::transmute::<_, &'a mut (dyn Widget + 'static)>(result))
        }
    }
}
