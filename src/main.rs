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

#[macro_use]
extern crate slog;
extern crate slog_term;

pub mod app;
pub mod user;
pub mod error;
pub mod log;

use app::Application;
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;
use std::env;

fn main() {
    let gio_app = gio::Application::new(
        "org.muhannad.HomeKeeper",
        gio::ApplicationFlags::IS_SERVICE
    );

    // using GApplication to run app::Application::run()
    gio_app.connect_activate(move |gio_app| {
        let prgname = "home-keeper";
        let app_name = "Home keeper";
        glib::set_application_name(app_name);
        glib::set_prgname(Some(prgname));

        // create logger
        let logger = log::build_logger(prgname, "/var/log/")
            .expect("Couldn't open log file in /var/log/");

        let _ = Application::new(gio_app.clone(), logger.clone())
            .and_then(|ok_app| {
                ok_app.run()
            })
            .or_else(|err| {
                err.log(&logger);
                Err(err)
            });
    });

    // run our GApplication
    let args: Vec<String> = env::args().collect();
    gio_app.run(&args);
}
