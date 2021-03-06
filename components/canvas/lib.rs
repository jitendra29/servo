/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(core)]
#![feature(nonzero)]
#![feature(slice_bytes)]
#![feature(vec_push_all)]
#![feature(plugin)]
#![plugin(plugins)]

extern crate azure;
extern crate canvas_traits;
extern crate core;
extern crate cssparser;
extern crate euclid;
extern crate gfx_traits;
extern crate gleam;
extern crate ipc_channel;
extern crate layers;
#[macro_use]
extern crate log;
extern crate num;
extern crate offscreen_gl_context;
extern crate util;

pub mod canvas_paint_task;
mod premultiplytable;
pub mod webgl_paint_task;
