//
// app.rs
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

use std::path::{PathBuf, Path};
use std::fs;
use slog::Logger;

use gio;
use glib;

use error::Error as Error;
use super::user::User;

pub struct Application {
    _parent: gio::Application,
    conf: glib::KeyFile,
    logger: Logger,
    users: Vec<User>,
}

impl Application {
    pub fn new(gio_app: gio::Application, logger: Logger) -> Result<Application, Error> {
        let conf_dir = Path::new("/etc/home-keeper/");
        let conf_file = Path::new("/etc/home-keeper/conf");

         // create folders in case of first run, although package maintainer should create them.
        if !conf_dir.exists() {
            fs::create_dir_all(conf_dir)?;
            info!(logger, "Create directory"; "conf-dir" => conf_dir.to_string_lossy().as_ref());
        }

        // exit with error, if there is no configuration file
        if !conf_file.exists() {
            return Err(Error::NoConfFileFound(conf_file.to_path_buf()));
        }

        // create GKeyFile
        let conf = glib::KeyFile::new();
        conf.load_from_file(conf_file, glib::KeyFileFlags::NONE)?;

        // get backup-path from our conf file
        let data_dir = PathBuf::from(conf.get_string("Backup", "backup-path")?);
        if !data_dir.exists() {
            // create if not exists
            fs::create_dir_all(data_dir.clone())?;
            info!(logger, "Create directory"; "data-dir" => data_dir.to_string_lossy().as_ref());
        }

        let usernames: Vec<String> = conf.get_string_list("Users", "usernames")?;

        if usernames.is_empty() {
            return Err(Error::EmptyUsernames);
        }

        let mut users = vec![];
        for username in usernames {
            match User::new(username) {
                Ok(user) => users.push(user),
                Err(error) => error.log(&logger),
            };
        }

        Ok(Self {
            _parent: gio_app,
            conf: conf,
            logger: logger,
            users: users,
        })
    }

    // TODO: removed when glib issue 279 fixed
    fn str_to_bool<T: AsRef<str>>(val: T) -> Result<bool, ()> {
        match val.as_ref() {
            "true;" => Ok(true),
            "false;" => Ok(false),
            _ => Err(()),
        }
    }

    // TODO: use get_boolean() insted of get_string, wating
    // for https://github.com/gtk-rs/glib/issues/279 to be fixed
    pub fn run(&self) -> Result<(), Error> {
        // get checksum value from our conf file
        let checksum = self.conf.get_string("Rsync Flags", "checksum")?;
        let checksum = Application::str_to_bool(&checksum)
            .unwrap_or_else(|_| {
                warn!(
                    self.logger,
                    "Invaild configuration";
                    "checksum" => checksum,
                    "default" => false
                );
                false
            });

        // get compress value from our conf file
        let compress = self.conf.get_string("Rsync Flags", "compress")?;
        let compress = Application::str_to_bool(&compress)
            .unwrap_or_else(|_| {
                warn!(
                    self.logger,
                    "Invaild configuration";
                    "compress" => compress,
                    "default" => true
                );
                true
            });

        // get backup-path value from our conf file
        let backup_path = self.conf.get_string("Backup", "backup-path")?;

        // restroing users data
        for user in self.users.iter() {
            if let Err(error) = user.restore_from(&backup_path, checksum, compress) {
                error.log(&self.logger);
            }
        }
        Ok(())
    }
}
