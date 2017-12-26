//
// error.rs
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

use std::error;
use std::convert::From;
use std::io;
use std::fmt;
use std::path::PathBuf;

use glib;

#[derive(Debug)]
pub enum Error {
    // IO Errors..
    Io(io::Error),
    // GLib Error
    GLibError(glib::error::Error),
    // Can find username in OS
    NoUserFound(String),
    // Not directory
    NotDirectory(PathBuf),
    // error for 'rsync' process
    RsyncError(String),
    // No configuration file found
    NoConfFileFound(PathBuf),
    // No usernames found
    EmptyUsernames,
    // backup files don't exists
    NoBackupFilesFound(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref io_error) => io_error.fmt(f),
            Error::GLibError(ref glib_error) => glib_error.fmt(f),
            Error::NoUserFound(ref username) => write!(f, "Couldn't find user '{}'", username),
            Error::NotDirectory(ref path) => write!(f, "{} is not directory", path.to_string_lossy()),
            Error::RsyncError(ref error_msg) => write!(f, "Error while using `rsync` command saying: {}", error_msg),
            Error::NoConfFileFound(ref path) => write!(f, "Couldn't find configuration file in {} , you may want to \
                                                           run home-keeper --help to for more info configuration",
                                                       path.to_string_lossy()),
            Error::EmptyUsernames => write!(f, "usernames is empty in configuration file"),
            Error::NoBackupFilesFound(ref path) => write!(f, "Couldn't find backup files in: {}",
                                                          path.to_string_lossy()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref io_error) => io_error.description(),
            Error::GLibError(ref glib_error) => glib_error.description(),
            Error::NoUserFound(_) => "Username not found",
            Error::NotDirectory(_) => "Path is not directory",
            Error::RsyncError(_) => "Error while using `rsync` command",
            Error::NoConfFileFound(_) => "Configuration file not found",
            Error::EmptyUsernames => "usernames is empty",
            Error::NoBackupFilesFound(_) => "Backup files not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref io_error) => Some(io_error),
            Error::GLibError(ref glib_error) => Some(glib_error),
            Error::NoUserFound(_) => None,
            Error::NotDirectory(_) => None,
            Error::RsyncError(_) => None,
            Error::NoConfFileFound(_) => None,
            Error::EmptyUsernames => None,
            Error::NoBackupFilesFound(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<glib::error::Error> for Error {
    fn from(err: glib::error::Error) -> Error {
        Error::GLibError(err)
    }
}
