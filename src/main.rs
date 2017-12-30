//
// main.rs
// This file is part of home_keeper
//
// Copyright (C) 2017 Muhannad Alrusayni 0x3UH4224D@gmail.com
//
// home_keeper is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// home_keeper is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with home_keeper. If not, see <http://www.gnu.org/licenses/>.

extern crate gio;
extern crate glib;

use gio::ApplicationExt;

pub mod app;
pub mod user;
pub mod error;

use app::Application;

fn main() {
    let gio_app = gio::Application::new(
        "org.muhannad.HomeKeeper",
        gio::ApplicationFlags::IS_SERVICE
    );

    // using GApplication to run app::Application::run()
    gio_app.connect_activate(|gio_app| {
        let _ = Application::new(gio_app.clone())
            .and_then(|ok_app| {
                ok_app.run()
            })
            .or_else(|err| {
                println!("home-keeper exit with error message: {}", err);
                Err(err)
            });
    });

    // gio_app.conn
}
