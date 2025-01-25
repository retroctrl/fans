// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]

#[cfg(feature = "postcard")]
use postcard_schema::Schema;

#[cfg(feature = "postcard")]
use serde::{Deserialize, Serialize};

/// Drive method for a fan.
///
/// A fan can be controlled by either setting the duty cycle or a target RPM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
pub enum FanControl {
    /// Duty cycle for a fan to be driven at.
    DutyCycle(u8),

    /// Target RPM for a fan to reach.
    Rpm(u16),
}

/// Generic mechanism to select a fan.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
pub struct FanSelect(pub u8);

/// Type alias for a fan duty cycle.
pub type FanDutyCycle = u8;

/// Type alias for a fan RPM.
pub type FanRpm = u16;

/// A simple report for a fan containing the duty cycle and RPM.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
pub struct FanReport {
    /// Selected fan that this report belongs to.
    pub select: FanSelect,

    /// Reported Duty Cycle
    pub duty_cycle: FanDutyCycle,

    /// Reported RPM
    pub rpm: FanRpm,
}