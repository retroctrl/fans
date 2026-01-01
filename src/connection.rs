// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "postcard")]
use postcard_schema::Schema;

#[cfg(feature = "postcard")]
use serde::{Deserialize, Serialize};

use crate::Error;

/// Connection type for a fan.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
#[repr(u8)]
pub enum FanConnection {
    /// No physical connection detected.
    #[default]
    NotConnected,

    /// 3-pin fan connection without PWM control.
    ThreePin,

    /// Standard Intel 4-pin fan connection with PWM control.
    FourPin,

    /// Virtual fan with no physical connection.
    Virtual,
}

impl TryFrom<u8> for FanConnection {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FanConnection::NotConnected),
            1 => Ok(FanConnection::ThreePin),
            2 => Ok(FanConnection::FourPin),
            3 => Ok(FanConnection::Virtual),
            _ => Err(Error::InvalidFanConnectionValue),
        }
    }
}
