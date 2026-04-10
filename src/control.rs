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

impl core::fmt::Display for FanControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DutyCycle(d) => write!(f, "FanControl - Duty Cycle: {d}"),
            Self::Rpm(r) => write!(f, "Fan Control - RPM: {r}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use insta::assert_snapshot;

    #[test]
    fn display_duty_cycle() {
        let control = FanControl::DutyCycle(42);
        assert_snapshot!(format!("{control}"), @"FanControl - Duty Cycle: 42");
    }

    #[test]
    fn display_rpm() {
        let control = FanControl::Rpm(1800);
        assert_snapshot!(format!("{control}"), @"Fan Control - RPM: 1800");
    }
}
