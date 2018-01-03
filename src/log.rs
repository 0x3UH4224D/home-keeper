//
// log.rs
// This file is part of home_keeper
//
// Copyright (C) 2018 Muhannad Alrusayni 0x3UH4224D@gmail.com
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

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use slog_term;
use slog::{Logger, Drain};
use std::path::Path;
use error::Error;

fn get_time() -> u64 {
    let sys_time = SystemTime::now();
    match sys_time.duration_since(UNIX_EPOCH) {
        Ok(val) => val.as_secs(),
        Err(sys_time_err) => sys_time_err.duration().as_secs(),
    }
}

pub fn build_logger<P: AsRef<Path>, N: AsRef<str>>(app_name: N, log_path: P) -> Result<Logger, Error> {
    let filename = format!("{}-{}.log", app_name.as_ref(), get_time());
    let log_path = log_path.as_ref().join(filename);
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)?;

    let plain = slog_term::PlainSyncDecorator::new(log_file);
    let root = Logger::root(
        slog_term::FullFormat::new(plain)
            .build()
            .fuse(),
        o!(),
    );

    Ok(root)
}
