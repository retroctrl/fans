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

/// Operating mode for a fan.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
#[repr(u8)]
pub enum FanMode {
    /// Completely disable the fan (if supported).
    #[default]
    Disabled,

    /// The fan is targeting a specific duty cycle.
    DutyCycle,

    /// The fan is targeting a specific RPM.
    Rpm,

    /// The fan is targeting a specific temperature.
    Temperature,

    /// The fan is following a temperature curve.
    TemperatureCurve,
}

impl TryFrom<u8> for FanMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FanMode::Disabled),
            1 => Ok(FanMode::DutyCycle),
            2 => Ok(FanMode::Rpm),
            3 => Ok(FanMode::Temperature),
            4 => Ok(FanMode::TemperatureCurve),
            _ => Err(Error::InvalidFanModeValue),
        }
    }
}
