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
use std::error::Error;

use gio;
use glib;

use super::error::Error as HKError;
use super::user::User;

pub struct Application {
    _parent: gio::Application,
    conf: glib::KeyFile,
    users: Vec<User>,
}

impl Application {
    pub fn new(gio_app: gio::Application) -> Result<Application, HKError> {
        let conf_dir = Path::new("/etc/home-keeper/");
        let conf_file = Path::new("/etc/home-keeper/conf");

        // create folders in case of first run, although package maintainer should create them.
        if !conf_dir.exists() {
            fs::create_dir_all(conf_dir)?;
        }

        // exit with error, if there is no configuration file
        if !conf_file.exists() {
            return Err(HKError::NoConfFileFound(conf_file.to_path_buf()));
        }

        // load users info from conf file
        let conf = glib::KeyFile::new();
        conf.load_from_file(conf_file, glib::KeyFileFlags::NONE)?;

        let data_dir = PathBuf::from(conf.get_string("Backup", "backup-path")?);
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }

        let usernames: Vec<String> = conf.get_string_list("Users", "usernames")?;

        if usernames.is_empty() {
            return Err(HKError::EmptyUsernames);
        }

        let mut users = vec![];
        for username in usernames {
            users.push(User::new(username)?);
        }

        Ok(Self {
            _parent: gio_app,
            conf: conf,
            users: users,
        })
    }

    pub fn run(&self) -> Result<(), HKError> {
        for user in self.users.iter() {
            let checksum = self.conf.get_string("Rsync Flags", "checksum")
                .and_then(|flag| {
                    if flag == "true;" {
                        Ok(true)
                    } else if flag == "false;" {
                        Ok(false)
                    } else {
                        Err(glib::Error::new(glib::KeyFileError::Parse,
                                             "checksum should be 'true' or 'false'"))
                    }
                })?;
            let compress = self.conf.get_string("Rsync Flags", "compress")
                .and_then(|flag| {
                    if flag == "true;" {
                        Ok(true)
                    } else if flag == "false;" {
                        Ok(false)
                    } else {
                        Err(glib::Error::new(glib::KeyFileError::Parse,
                                             "compress should be 'true' or 'false'"))
                    }
                })?;
            let backup_path = self.conf.get_string("Backup", "backup-path")?;

            match user.restore_from(backup_path.clone(), checksum, compress) {
                Ok(()) => {},
                Err(HKError::NoBackupFilesFound(ref _path)) => {
                    let remove_old_files = self.conf.get_string("Backup", "remove-old-files")
                        .and_then(|flag| {
                            if flag == "true;" {
                                Ok(true)
                            } else if flag == "false;" {
                                Ok(false)
                            } else {
                                Err(glib::Error::new(glib::KeyFileError::Parse,
                                                     "remove-old-files should be 'true' or 'false'"))
                            }
                        })?;
                    user.backup_to(backup_path, compress, remove_old_files)?;
                }
                // I should use log insted of stdout
                Err(ref error) => println!("{:?}", error.description()),
            }
        }
        Ok(())
    }
}
