// Window Container
// Contains a PistonWindow and a list of widgets
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::core::point::*;
use crate::widget::widget::*;

use piston_window::*;

pub struct PushrodWindow {
    pub window: PistonWindow,
    pub widgets: Vec<Box<dyn PushrodWidget>>,
}

impl PushrodWindow {
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn PushrodWidget>) {
        self.widgets.push(widget);
    }

    // TODO Need to fix to walk children instead of one by one.  Walking children will be far more accurate.
    pub fn get_widget_id_for_point(&mut self, point: Point) -> i32 {
        let mut found_id: i32 = -1;

        for (pos, obj) in self.widgets.iter_mut().enumerate() {
            let widget_point = &obj.get_origin();
            let widget_size: crate::core::point::Size = obj.get_size();

            if point.x >= widget_point.x
                && point.x <= widget_point.x + widget_size.w
                && point.y >= widget_point.y
                && point.y <= widget_point.y + widget_size.h
            {
                found_id = pos as i32;
            }
        }

        found_id
    }

    pub fn mouse_entered_for_id(&mut self, id: i32) {
        &self.widgets[id as usize].mouse_entered();
    }

    pub fn mouse_exited_for_id(&mut self, id: i32) {
        &self.widgets[id as usize].mouse_exited();
    }

    pub fn mouse_scrolled_for_id(&mut self, id: i32, point: Point) {
        &self.widgets[id as usize].mouse_scrolled(point);
    }

    pub fn get_widget_for_id(&mut self, id: i32) -> &Box<dyn PushrodWidget> {
        &self.widgets[id as usize]
    }
}
