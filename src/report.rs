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

use crate::{
    Error, FanConnection, FanCurrent, FanDutyCycle, FanMode, FanRpm, FanSelect, FanVoltage,
};

/// A report containing the status of a fan.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "postcard", derive(Deserialize, Serialize, Schema))]
pub struct FanReport {
    pub select: FanSelect,
    pub capabilities: u16,
    pub connection: FanConnection,
    pub duty_cycle: FanDutyCycle,
    pub rpm: FanRpm,
    pub mode: FanMode,
    pub voltage: FanVoltage,
    pub current: FanCurrent,
}

impl FanReport {
    const BYTE_LENGTH: usize = 17;

    pub fn write_to_slice(self, out: &mut [u8]) -> Result<(), Error> {
        if out.len() != Self::BYTE_LENGTH {
            return Err(Error::InvalidFarReportByteLength);
        }

        out[0..2].copy_from_slice(&self.select.0.to_be_bytes());
        out[2..4].copy_from_slice(&self.capabilities.to_be_bytes());
        out[4] = self.connection as u8;
        out[5] = self.duty_cycle;
        out[6..8].copy_from_slice(&self.rpm.to_be_bytes());
        out[8] = self.mode as u8;
        out[9..13].copy_from_slice(&self.voltage.to_be_bytes());
        out[13..17].copy_from_slice(&self.current.to_be_bytes());

        Ok(())
    }
}

impl TryFrom<&[u8]> for FanReport {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != Self::BYTE_LENGTH {
            return Err(Error::InvalidFarReportByteLength);
        }

        Ok(FanReport {
            select: FanSelect::from_be_bytes([data[0], data[1]]),
            capabilities: u16::from_be_bytes([data[2], data[3]]),
            connection: FanConnection::try_from(data[4])?,
            duty_cycle: data[5],
            rpm: u16::from_be_bytes([data[6], data[7]]),
            mode: FanMode::try_from(data[8])?,
            voltage: u32::from_be_bytes([data[9], data[10], data[11], data[12]]),
            current: u32::from_be_bytes([data[13], data[14], data[15], data[16]]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_u8_slice() {
        #[rustfmt::skip]
        let data: [u8; 17] = [
            0x00, 0x01,
            0x00, 0x03,
            0x02,
            0x64,
            0x02, 0x58,
            0x01,
            0x00, 0x61, 0xA8, 0x00,
            0x00, 0x00, 0x27, 0x10,
        ];

        let report = FanReport::try_from(&data[..]).unwrap();

        assert_eq!(report.select, FanSelect::new(1));
        assert_eq!(report.capabilities, 3);
        assert_eq!(report.connection, FanConnection::FourPin);
        assert_eq!(report.duty_cycle, 100);
        assert_eq!(report.rpm, 600);
        assert_eq!(report.mode, FanMode::DutyCycle);
        assert_eq!(report.voltage, 6400000);
        assert_eq!(report.current, 10000);
    }
}
