//! Virtual fan device driver for development, testing, and simulation.
#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

use fans::{FanControl, FanDutyCycle, FanRpm, FanSelect};
use thiserror::Error;

/// Duty cycle which virtual fans are set to by default.
const DEFAULT_DUTY_CYCLE: u8 = 50;

/// RPM which virtual fans are set to by default.
const DEFAULT_RPM: u16 = 500;

/// Maximum RPM value that a virtual fan can reach by default.
const DEFAULT_RPM_MAXIMUM: u16 = 2_000;

#[derive(Clone, Copy, Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("Duty cycle out of range")]
    DutyCycleOutOfRange,

    #[error("Invalid fan selection")]
    InvalidFan,

    #[error("RPM out of range")]
    RpmOutOfRange,
}

#[derive(Clone, Copy, Debug)]
struct VirtualFan {
    duty_cycle: FanDutyCycle,
    rpm: FanRpm,
    mode: FanControl,
}

impl Default for VirtualFan {
    fn default() -> Self {
        Self {
            duty_cycle: DEFAULT_DUTY_CYCLE,
            rpm: DEFAULT_RPM,
            mode: FanControl::DutyCycle(DEFAULT_DUTY_CYCLE),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VirtualFanDriver<const N: usize> {
    /// List of virtual fans
    fan_list: [VirtualFan; N],

    /// Maximum RPM value that a virtual fan can reach
    rpm_maximum: FanRpm,
}

impl<const N: usize> Default for VirtualFanDriver<N> {
    fn default() -> Self {
        Self {
            fan_list: [VirtualFan::default(); N],
            rpm_maximum: DEFAULT_RPM_MAXIMUM,
        }
    }
}

impl<const N: usize> VirtualFanDriver<N> {
    pub fn count(&self) -> u16 {
        N as u16
    }

    fn valid_fan(&self, select: FanSelect) -> Result<(), Error> {
        if select.0 <= self.fan_list.len() as u16 && select.0 != 0 {
            Ok(())
        } else {
            Err(Error::InvalidFan)
        }
    }

    pub fn duty_cycle(&mut self, select: FanSelect) -> Result<FanDutyCycle, Error> {
        self.valid_fan(select)?;
        Ok(self.fan_list[select.0 as usize - 1].duty_cycle)
    }

    fn set_duty_cycle(&mut self, select: FanSelect, duty_cycle: FanDutyCycle) -> Result<(), Error> {
        self.valid_fan(select)?;

        if duty_cycle > 100 {
            return Err(Error::DutyCycleOutOfRange);
        }

        self.fan_list[select.0 as usize - 1].duty_cycle = duty_cycle;
        Ok(())
    }

    pub fn rpm(&mut self, select: FanSelect) -> Result<FanRpm, Error> {
        self.valid_fan(select)?;
        Ok(self.fan_list[select.0 as usize - 1].rpm)
    }

    fn set_rpm(&mut self, select: FanSelect, rpm: FanRpm) -> Result<(), Error> {
        self.valid_fan(select)?;

        if rpm > self.rpm_maximum {
            return Err(Error::DutyCycleOutOfRange);
        }

        self.fan_list[select.0 as usize - 1].rpm = rpm;
        Ok(())
    }

    pub fn report(&mut self, select: FanSelect) -> Result<(FanDutyCycle, FanRpm), Error> {
        self.valid_fan(select)?;
        Ok((
            self.fan_list[select.0 as usize - 1].duty_cycle,
            self.fan_list[select.0 as usize - 1].rpm,
        ))
    }

    pub fn mode(&mut self, select: FanSelect) -> Result<FanControl, Error> {
        self.valid_fan(select)?;
        Ok(self.fan_list[select.0 as usize - 1].mode)
    }

    pub fn set_mode(&mut self, select: FanSelect, mode: FanControl) -> Result<(), Error> {
        self.valid_fan(select)?;
        match mode {
            FanControl::DutyCycle(duty_cycle) => self.set_duty_cycle(select, duty_cycle)?,
            FanControl::Rpm(rpm) => self.set_rpm(select, rpm)?,
        }
        self.fan_list[select.0 as usize - 1].mode = mode;

        Ok(())
    }

    pub fn maximum_rpm(&mut self) -> FanRpm {
        self.rpm_maximum
    }

    pub fn set_maximum_rpm(&mut self, rpm: FanRpm) {
        self.rpm_maximum = rpm;
    }

    #[cfg(feature = "log")]
    pub fn dump_info(&self) -> Result<(), Error> {
        use log;
        for (i, fan) in self.fan_list.iter().enumerate() {
            log::info!("Fan {}: {:?}", i + 1, fan);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_fan() {
        let quad_driver: VirtualFanDriver<4> = VirtualFanDriver::default();

        let select = FanSelect(0);
        assert!(quad_driver.valid_fan(select).is_err());

        for i in 1..=4 {
            let select = FanSelect(i);
            assert!(quad_driver.valid_fan(select).is_ok());
        }

        let select = FanSelect(5);
        assert!(quad_driver.valid_fan(select).is_err());
    }

    #[test]
    fn duty_cycle() {
        let mut quad_driver: VirtualFanDriver<4> = VirtualFanDriver::default();

        for duty in 0..=100_u8 {
            for fan in 1..=4 {
                let select = FanSelect(fan);
                quad_driver.set_duty_cycle(select, duty).unwrap();
                assert_eq!(quad_driver.duty_cycle(select).unwrap(), duty);
            }
        }

        assert!(quad_driver.set_duty_cycle(FanSelect(1), 101).is_err());
    }

    #[test]
    fn rpm() {
        let mut quad_driver: VirtualFanDriver<4> = VirtualFanDriver::default();

        for rpm in 0..=quad_driver.rpm_maximum {
            for fan in 1..=4 {
                let select = FanSelect(fan);
                quad_driver.set_rpm(select, rpm).unwrap();
                assert_eq!(quad_driver.rpm(select).unwrap(), rpm);
            }
        }

        assert!(quad_driver
            .set_rpm(FanSelect(1), DEFAULT_RPM_MAXIMUM + 1)
            .is_err());
    }

    #[test]
    fn maximum_rpm() {
        let mut quad_driver: VirtualFanDriver<4> = VirtualFanDriver::default();

        assert_eq!(quad_driver.maximum_rpm(), DEFAULT_RPM_MAXIMUM);

        quad_driver.set_maximum_rpm(2_500);
        assert_eq!(quad_driver.maximum_rpm(), 2_500);
    }

    #[test]
    fn mode() {
        let mut quad_driver: VirtualFanDriver<4> = VirtualFanDriver::default();

        for fan in 1..=4 {
            let select = FanSelect(fan);
            assert_eq!(
                quad_driver.mode(select).unwrap(),
                FanControl::DutyCycle(DEFAULT_DUTY_CYCLE)
            );
        }

        assert!(quad_driver
            .set_mode(FanSelect(1), FanControl::DutyCycle(101))
            .is_err());

        for fan in 1..=4 {
            let select = FanSelect(fan);
            quad_driver
                .set_mode(select, FanControl::Rpm(DEFAULT_RPM))
                .unwrap();
            assert_eq!(
                quad_driver.mode(select).unwrap(),
                FanControl::Rpm(DEFAULT_RPM)
            );
        }
    }
}
