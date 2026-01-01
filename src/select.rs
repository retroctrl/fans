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

/// Generic mechanism to select a fan.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
pub struct FanSelect(pub u16);

impl FanSelect {
    /// Create a new [`FanSelect`] from a raw value.
    pub const fn new(value: u16) -> Self {
        FanSelect(value)
    }

    pub fn from_be_bytes(bytes: [u8; 2]) -> Self {
        FanSelect(u16::from_be_bytes(bytes))
    }
}
