// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */
use std::{
    error::Error as StdError,
    io::{
        Error as IoError,
        ErrorKind as IoErrorKind,
    },
    fmt::{
        Display,
        Formatter,
        Result,
    },
};
use rppal::{
    uart::Error as UartError,
    gpio::Error as GpioError,
};

/* internal mods */

/* internal uses */

#[derive(Debug)]
pub enum Error {
    Uninitialized,
    InvalidValue(u16),
    FaultyRead {
        actual_count: usize,
        expected_count: usize,
    },
    FaultyWrite {
        actual_count: usize,
        expected_count: usize,
    },
    Io(IoError),
}

impl Error {
    pub fn new_io_error<E>(err_kind: IoErrorKind, err_msg: E) -> Self
    where
    E: Into<Box<dyn StdError + Send + Sync>>, {
        return Error::Io(IoError::new(err_kind, err_msg));
    }
}

impl StdError for Error {}

impl Display for Error {
    #[allow(unused)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!();
    }
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        return Self::Io(io_error);
    }
}

impl From<UartError> for Error {
    fn from(uart_error: UartError) -> Self {
        return match uart_error {
            UartError::Io(std_err) => Error::from(std_err),
            UartError::Gpio(gpio_err) => match gpio_err {
                GpioError::UnknownModel => Error::new_io_error(IoErrorKind::Other, "unknown model"),
                GpioError::PinNotAvailable(pin) => Error::new_io_error(IoErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
                GpioError::PermissionDenied(err_string) => Error::new_io_error(IoErrorKind::PermissionDenied, format!("permission denied: {} ", err_string)),
                GpioError::Io(error) => Error::from(error),
                GpioError::ThreadPanic => Error::new_io_error(IoErrorKind::Other, "thread panic"),
            },
            UartError::InvalidValue => Error::new_io_error(IoErrorKind::Other, "invalid value"),
        };
    }
}
