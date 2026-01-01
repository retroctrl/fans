// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]

mod connection;
mod control;
mod error;
mod mode;
mod report;
mod select;

pub use connection::FanConnection;
pub use control::FanControl;
pub use error::Error;
pub use mode::FanMode;
pub use report::FanReport;
pub use select::FanSelect;

/// Type alias for a fan duty cycle.
pub type FanDutyCycle = u8;

/// Type alias for a fan RPM.
pub type FanRpm = u16;

/// Type alias for a fan voltage (mV).
pub type FanVoltage = u32;

/// Type alias for a fan current (mA).
pub type FanCurrent = u32;
