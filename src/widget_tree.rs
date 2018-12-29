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
use crate::widget::{Renderer, Widget};

pub struct WidgetTree {
    widgets: Vec<Option<Box<dyn Widget>>>,
    tree: Vec<Option<TreeEntry>>,
}

struct TreeEntry {
    parent: usize,
    widget: usize,
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
    pub fn new<T: Widget + 'static>(root: T) -> WidgetTree {
        let mut widgets: Vec<Option<Box<dyn Widget>>> = Vec::new();
        widgets.push(Some(Box::new(root)));

        let root = TreeEntry {
            parent: 0,
            widget: 0,
            children: Vec::new(),
        };

        WidgetTree {
            widgets,
            tree: vec![Some(root)],
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

            if WidgetTree::fire_event(&mut self[index], event) {
                return true;
            }
        }

        false
    }

    fn fire_event(widget: &mut dyn Widget, event: &Event) -> bool {
        use crate::input::EventKind::*;
        match &event.kind {
            MouseMoved { delta_x, delta_y } => {
                widget.mouse_moved(*delta_x, *delta_y)
            },
            MousePressed { button } => {
                widget.mouse_pressed(*button)
            },
            MouseReleased { button } => {
                widget.mouse_released(*button)
            }
        }
    }

    pub fn add_child<T: Widget + 'static>(&mut self, parent_index: usize, child: T) {
        self.add_child_boxed(parent_index, Box::new(child));
    }

    fn add_child_boxed(&mut self, parent_index: usize, mut child: Box<dyn Widget>) {
        if parent_index >= self.widgets.len() {
            // TODO error message
            panic!();
        }

        let child_index = self.widgets.len();
        child.state_mut().index = child_index;

        if let Some(ref mut tree_entry) = self.tree[parent_index] {
            tree_entry.children.push(child_index);
        } else {
            // TODO error message
            panic!();
        }

        self.widgets.push(Some(child));
        self.tree.push(Some(TreeEntry {
            parent: parent_index,
            widget: child_index,
            children: Vec::new()
        }));

        self[child_index].on_add();

        let to_add: Vec<_> = self[child_index].state_mut().to_add.drain(..).collect();
        for child in to_add {
            self.add_child_boxed(child_index, child);
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
