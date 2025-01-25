// Copyright (c) 2025 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Drive method for a fan.
///
/// A fan can be controlled by either setting the duty cycle or a target RPM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FanControl {
    /// Duty cycle for a fan to be driven at.
    DutyCycle(u8),

    /// Target RPM for a fan to reach.
    Rpm(u16),
}

/// Generic mechanism to select a fan.
#[derive(Clone, Copy, Debug)]
pub struct FanSelect(pub u8);
