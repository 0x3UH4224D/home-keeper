//
// user.rs
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

// use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use std::process::Command;
use std::ffi::OsStr;
use std::ffi::OsString;

use super::error::Error;

pub struct User {
    pub name: String,
    pub home_dir: PathBuf,
}

impl User {
    // this function read /etc/passwd file and get the user home directory from there..
    fn home_dir_for<T: AsRef<str>>(username: T) -> Result<PathBuf, Error> {
        let mut file = fs::File::open("/etc/passwd")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents.find(username.as_ref())
            .ok_or(Error::NoUserFound(username.as_ref().to_string()))
            .and_then(|index| {
                let contents = &contents[index..];
                let mut lines = contents.lines();
                lines.next()
                    .ok_or(Error::NoUserFound(username.as_ref().to_string()))
                    .and_then(|line| {
                        let mut user_info = line.split(':');
                        return user_info.nth(5)
                            .ok_or(Error::NoUserFound(username.as_ref().to_string()))
                            .map(|str_path| str_path.into());
                    })
            })
    }

    fn user_exists<T: AsRef<str>>(username: T) -> bool {
        unimplemented!();
    }

    pub fn new(username: String) -> Result<User, Error> {
        // combaine user_home_dir() with user name so we got the home directory for this user
        let user_home_dir = User::home_dir_for(username.as_str())?;

        Ok(User {
            name: username,
            home_dir: user_home_dir,
        })
    }

    // rsync --checksum --archive --compress --delete --perms [source] [dest]
    // todo: use rsync insted of cp
    pub fn backup_to<P: AsRef<Path>>(
        &self,
        path: P,
        compress: bool,
        remove_old_files: bool,
    ) -> Result<(), Error> {
        // put args together
        let mut args = vec!["--perms", "--delete", "--archive"];
        if compress {
            args.push("--compress");
        }

        let dest_path = path.as_ref().join(&self.name);
        if dest_path.exists() {
            if dest_path.is_dir() {
                fs::remove_dir_all(dest_path.clone())?;
            } else {
                fs::remove_file(dest_path.clone())?;
            }
        }

        // run rsync
        let output = Command::new("rsync")
            .args(args)
            .arg(self.home_dir.as_ref() as &OsStr)
            .arg(path.as_ref())
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::RsyncError(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }

    pub fn restore_from<P: AsRef<Path>>(
        &self,
        path: P,
        checksum: bool,
        compress: bool
    ) -> Result<(), Error> {
        // put args together
        let mut args = vec!["--perms", "--delete", "--archive"];
        if checksum {
            args.push("--checksum");
        }
        if compress {
            args.push("--compress");
        }

        let user_backup_files = path.as_ref().join(self.name.clone() + "/");
        if !user_backup_files.is_dir() || !user_backup_files.exists() {
            return Err(Error::NoBackupFilesFound(user_backup_files));
        }

        // run rsync
        let output = Command::new("rsync")
            .args(args)
            .arg(user_backup_files)
            .arg(self.home_dir.as_ref() as &OsStr)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::RsyncError(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
