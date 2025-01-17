/*!
# Hardware monitoring in Linux based on LM Sensors

`lm-sensors` provides user-space support for the hardware monitoring drivers
in Linux.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

## Listing all available sensors

```rust
// Import all useful traits of this crate.
use lm_sensors::prelude::*;

// Initialize LM sensors library.
let sensors = lm_sensors::Initializer::default().initialize()?;

// Print all chips.
for chip in sensors.chip_iter(None) {
    if let Some(path) = chip.path() {
        println!("chip: {} at {} ({})", chip, chip.bus(), path.display());
    } else {
        println!("chip: {} at {}", chip, chip.bus());
    }

    // Print all features of the current chip.
    for feature in chip.feature_iter() {
        let name = feature.name().transpose()?.unwrap_or("N/A");
        println!("    {}: {}", name, feature);

        // Print all sub-features of the current chip feature.
        for sub_feature in feature.sub_feature_iter() {
            if let Ok(value) = sub_feature.value() {
                println!("        {}: {}", sub_feature, value);
            } else {
                println!("        {}: N/A", sub_feature);
            }
        }
    }
}
# Ok::<(), lm_sensors::errors::Error>(())
```

The following is an example output of the sample above:

```text
chip: iwlwifi_1-virtual-0 at Virtual device (/sys/class/hwmon/hwmon8)
    temp1: temp1
        temp1_input: N/A
chip: thinkpad-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon6)
    fan1: fan1
        fan1_input: 2488
    fan2: fan2
        fan2_input: 2262
    temp1: CPU
        temp1_input: 54
    temp2: GPU
        temp2_input: 50
    temp3: temp3
        temp3_input: 57
    temp4: temp4
        temp4_input: 0
    temp5: temp5
        temp5_input: 54
    temp6: temp6
        temp6_input: 58
    temp7: temp7
        temp7_input: 60
    temp8: temp8
        temp8_input: 0
chip: ucsi_source_psy_USBC000:002-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon10)
    in0: in0
        in0_input: 5
        in0_min: 5
        in0_max: 5
    curr1: curr1
        curr1_input: 5
        curr1_max: 5
chip: coretemp-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon4)
    temp1: Package id 0
        temp1_input: 53
        temp1_max: 53
        temp1_crit: 53
        temp1_crit_alarm: 53
    temp2: Core 0
        temp2_input: 100
        temp2_max: 100
        temp2_crit: 100
        temp2_crit_alarm: 100
    temp3: Core 1
        temp3_input: 100
        temp3_max: 100
        temp3_crit: 100
        temp3_crit_alarm: 100
    temp4: Core 2
        temp4_input: 0
        temp4_max: 0
        temp4_crit: 0
        temp4_crit_alarm: 0
    temp5: Core 3
        temp5_input: 58
        temp5_max: 58
        temp5_crit: 58
        temp5_crit_alarm: 58
    temp6: Core 4
        temp6_input: 100
        temp6_max: 100
        temp6_crit: 100
        temp6_crit_alarm: 100
    temp7: Core 5
        temp7_input: 100
        temp7_max: 100
        temp7_crit: 100
        temp7_crit_alarm: 100
    temp8: Core 6
        temp8_input: 0
        temp8_max: 0
        temp8_crit: 0
        temp8_crit_alarm: 0
    temp9: Core 7
        temp9_input: 57
        temp9_max: 57
        temp9_crit: 57
        temp9_crit_alarm: 57
chip: nvme-pci-5500 at PCI adapter (/sys/class/hwmon/hwmon2)
    temp1: Composite
        temp1_input: 46.85
        temp1_max: 46.85
        temp1_min: 46.85
        temp1_crit: 46.85
        temp1_alarm: 46.85
    temp2: Sensor 1
        temp2_input: 83.85
        temp2_max: 83.85
        temp2_min: 83.85
    temp3: Sensor 2
        temp3_input: -273.15
        temp3_max: -273.15
        temp3_min: -273.15
chip: acpitz-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon0)
    temp1: temp1
        temp1_input: 54
        temp1_crit: 54
chip: ucsi_source_psy_USBC000:001-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon9)
    in0: in0
        in0_input: 0
        in0_min: 0
        in0_max: 0
    curr1: curr1
        curr1_input: 0
        curr1_max: 0
chip: pch_cometlake-virtual-0 at Virtual device (/sys/class/hwmon/hwmon7)
    temp1: temp1
        temp1_input: 58
chip: BAT0-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon3)
    in0: in0
        in0_input: 12.221
chip: nvme-pci-0200 at PCI adapter (/sys/class/hwmon/hwmon1)
    temp1: Composite
        temp1_input: 39.85
        temp1_max: 39.85
        temp1_min: 39.85
        temp1_crit: 39.85
        temp1_alarm: 39.85
    temp2: Sensor 1
        temp2_input: 84.85
        temp2_max: 84.85
        temp2_min: 84.85
    temp3: Sensor 2
        temp3_input: -273.15
        temp3_max: -273.15
        temp3_min: -273.15
```

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
*/

#![warn(missing_docs)]
/*
#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::wildcard_imports,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
*/

pub mod bus;
pub mod chip;
pub mod errors;
pub mod feature;
pub mod sub_feature;
mod utils;
pub mod value;

#[cfg(test)]
mod tests;

use std::ffi::CStr;
use std::fs::File;
use std::marker::PhantomData;
use std::os::raw::c_short;
use std::path::PathBuf;
use std::sync::atomic;
use std::sync::atomic::AtomicBool;
use std::{io, ptr};

use sensors_sys::*;

use crate::errors::{Error, Reporter, Result};
use crate::utils::{api_access_lock, LibCFileStream};

pub mod prelude {
    //! Easily import crate traits.

    pub use crate::bus::{ExclusiveBus, SharedBus};
    pub use crate::chip::SharedChip;
}

/// Bus connection of some [`Kind`], *e.g.,* PCI.
///
/// [`Kind`]: crate::bus::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bus(pub(crate) sensors_bus_id);

/// Shared reference to a bus connection of some [`Kind`], *e.g.,* PCI.
///
/// [`Kind`]: crate::bus::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusRef<'a>(pub(crate) &'a sensors_bus_id);

/// Exclusive reference to a bus connection of some [`Kind`], *e.g.,* PCI.
///
/// [`Kind`]: crate::bus::Kind
#[derive(Debug, PartialEq, Eq)]
pub struct BusMut<'a>(pub(crate) &'a mut sensors_bus_id);

/// Shared reference to a chip connected to sensors or actuators.
#[derive(Debug, Clone, Copy, Eq)]
pub struct ChipRef<'a>(pub(crate) &'a sensors_chip_name);

/// Chip connected to sensors or actuators.
#[derive(Debug, PartialEq, Eq)]
pub struct Chip<'a> {
    pub(crate) raw: sensors_chip_name,
    pub(crate) _phantom: &'a PhantomData<crate::LMSensors>,
}

/// Shared reference to a feature of some [`Kind`] (*e.g.,* temperature),
/// provided by a [`Chip`].
///
/// [`Kind`]: crate::feature::Kind
#[derive(Debug, Clone, Copy, Eq)]
pub struct FeatureRef<'a> {
    pub(crate) chip: ChipRef<'a>,
    pub(crate) raw: &'a sensors_feature,
}

/// Shared reference to a sub-feature of some [`Kind`] (*e.g.,* temperature input),
/// provided by a [`Chip`].
///
/// [`Kind`]: crate::value::Kind
#[derive(Debug, Clone, Copy, Eq)]
pub struct SubFeatureRef<'a> {
    pub(crate) feature: FeatureRef<'a>,
    pub(crate) raw: &'a sensors_subfeature,
}

/// Value reported by a sensor or set for an actuator,
/// controlled by a [`SubFeatureRef`] instance.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    VoltageInput(f64),
    VoltageMinimum(f64),
    VoltageMaximum(f64),
    VoltageLCritical(f64),
    VoltageCritical(f64),
    VoltageAverage(f64),
    VoltageLowest(f64),
    VoltageHighest(f64),
    VoltageAlarm(bool),
    VoltageMinimumAlarm(bool),
    VoltageMaximumAlarm(bool),
    VoltageBeep(bool),
    VoltageLCriticalAlarm(bool),
    VoltageCriticalAlarm(bool),

    FanInput(f64),
    FanMinimum(f64),
    FanMaximum(f64),
    FanAlarm(bool),
    FanFault(bool),
    FanDivisor(f64),
    FanBeep(bool),
    FanPulses(f64),
    FanMinimumAlarm(bool),
    FanMaximumAlarm(bool),

    TemperatureInput(f64),
    TemperatureMaximum(f64),
    TemperatureMaximumHysteresis(f64),
    TemperatureMinimum(f64),
    TemperatureCritical(f64),
    TemperatureCriticalHysteresis(f64),
    TemperatureLCritical(f64),
    TemperatureEmergency(f64),
    TemperatureEmergencyHysteresis(f64),
    TemperatureLowest(f64),
    TemperatureHighest(f64),
    TemperatureMinimumHysteresis(f64),
    TemperatureLCriticalHysteresis(f64),
    TemperatureAlarm(bool),
    TemperatureMaximumAlarm(bool),
    TemperatureMinimumAlarm(bool),
    TemperatureCriticalAlarm(bool),
    TemperatureFault(bool),
    TemperatureType(crate::value::TemperatureSensorKind),
    TemperatureOffset(f64),
    TemperatureBeep(bool),
    TemperatureEmergencyAlarm(bool),
    TemperatureLCriticalAlarm(bool),

    PowerAverage(f64),
    PowerAverageHighest(f64),
    PowerAverageLowest(f64),
    PowerInput(f64),
    PowerInputHighest(f64),
    PowerInputLowest(f64),
    PowerCap(f64),
    PowerCapHysteresis(f64),
    PowerMaximum(f64),
    PowerCritical(f64),
    PowerMinimum(f64),
    PowerLCritical(f64),
    PowerAverageInterval(f64),
    PowerAlarm(bool),
    PowerCapAlarm(bool),
    PowerMaximumAlarm(bool),
    PowerCriticalAlarm(bool),
    PowerMinimumAlarm(bool),
    PowerLCriticalAlarm(bool),

    EnergyInput(f64),

    CurrentInput(f64),
    CurrentMinimum(f64),
    CurrentMaximum(f64),
    CurrentLCritical(f64),
    CurrentCritical(f64),
    CurrentAverage(f64),
    CurrentLowest(f64),
    CurrentHighest(f64),
    CurrentAlarm(bool),
    CurrentMinimumAlarm(bool),
    CurrentMaximumAlarm(bool),
    CurrentBeep(bool),
    CurrentLCriticalAlarm(bool),
    CurrentCriticalAlarm(bool),

    HumidityInput(f64),

    VoltageID(f64),

    IntrusionAlarm(bool),
    IntrusionBeep(bool),

    BeepEnable(bool),

    Unknown {
        kind: crate::value::Kind,
        value: f64,
    },
}

/// LM sensors library initializer, producing an instance of [`LMSensors`].
#[derive(Debug, Default)]
pub struct Initializer {
    error_listener: Option<Box<dyn crate::errors::Listener>>,
    config_path: Option<PathBuf>,
    config_file: Option<File>,
}

/// LM sensors library instance, producing instances of [`Chip`]s, [`Bus`]es, etc.
#[derive(Debug)]
pub struct LMSensors {
    error_reporter: Reporter,
}

impl Initializer {
    /**
    Set the path of the configuration file to be read during LM sensors
    library initialization.

    # Example

    ```rust
    let sensors = lm_sensors::Initializer::default()
        .config_path("/dev/null")
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn config_path(self, path: impl Into<PathBuf>) -> Self {
        Self {
            error_listener: self.error_listener,
            config_path: Some(path.into()),
            config_file: None,
        }
    }

    /**
    Set the configuration contents to be used during LM sensors
    library initialization.

    # Example

    ```rust
    # use std::fs::File;
    let config_file = File::open("/dev/null").unwrap();
    let sensors = lm_sensors::Initializer::default()
        .config_file(config_file)
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn config_file(self, file: File) -> Self {
        Self {
            error_listener: self.error_listener,
            config_path: None,
            config_file: Some(file),
        }
    }

    /**
    Set the error listener to be used during LM sensors library initialization.

    # Example

    ```rust
    #[derive(Debug)]
    struct EL;

    impl lm_sensors::errors::Listener for EL {
        fn on_lm_sensors_config_error(&self, error: &str,
            file_name: Option<&std::path::Path>, line_number: usize)
        {
            if let Some(file_name) = file_name {
                eprintln!("[ERROR] lm-sensors config: {} @{}:{}",
                          error, file_name.display(), line_number);
            } else {
                eprintln!("[ERROR] lm-sensors config: {} @<config>:{}",
                          error, line_number);
            }
        }

        fn on_lm_sensors_fatal_error(&self, error: &str, procedure: &str) {
            eprintln!("[FATAL] lm-sensors: {} @{}", error, procedure);
        }
    }

    let sensors = lm_sensors::Initializer::default()
        .error_listener(Box::new(EL))
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn error_listener(self, listener: Box<dyn crate::errors::Listener>) -> Self {
        Self {
            error_listener: Some(listener),
            config_path: self.config_path,
            config_file: self.config_file,
        }
    }

    /**
    Return an instance of a loaded and initialized LM sensors library.

    # Example

    ```rust
    let sensors = lm_sensors::Initializer::default().initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    pub fn initialize(self) -> Result<LMSensors> {
        let config_file_fp = match (self.config_path, self.config_file) {
            (None, None) => None,
            (None, Some(config_file)) => LibCFileStream::from_file(config_file).map(Some)?,
            (Some(config_path), None) => LibCFileStream::from_path(&config_path).map(Some)?,
            _ => unreachable!(),
        };

        let error_listener = self
            .error_listener
            .map_or_else(ptr::null_mut, |v| Box::into_raw(Box::new(v)));

        let result = LMSensors::new(config_file_fp, error_listener);

        if result.is_err() && !error_listener.is_null() {
            drop(unsafe { Box::from_raw(error_listener) });
        }
        result
    }
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

impl LMSensors {
    /// Returns the version of the LM sensors library,
    /// if available and valid UTF-8.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.raw_version().and_then(|s| s.to_str().ok())
    }

    /// Returns the raw version of the LM sensors library, if available.
    #[must_use]
    pub fn raw_version(&self) -> Option<&CStr> {
        let p = unsafe { libsensors_version };
        (!p.is_null()).then(move || unsafe { CStr::from_ptr(p) })
    }

    /// Return a new instance of [`ChipRef`], given a shared reference
    /// to a raw chip.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_chip_name`] reference must have been returned from
    ///   [`sensors_get_detected_chips`].
    #[must_use]
    pub unsafe fn new_chip_ref<'a>(&'a self, chip: &'a sensors_chip_name) -> ChipRef<'a> {
        ChipRef(chip)
    }

    /// Return a new instance of [`Chip`], given a raw chip.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_chip_name`] must have been previously initialized
    ///   by calling [`sensors_parse_chip_name`].
    #[must_use]
    pub unsafe fn new_raw_chip(&'_ self, chip: sensors_chip_name) -> Chip<'_> {
        Chip {
            raw: chip,
            _phantom: &PhantomData,
        }
    }

    /// Return a new instance of [`Chip`], given a chip name.
    pub fn new_chip(&self, name: &str) -> Result<Chip> {
        Chip::new(name)
    }

    /// Return a new instance of [`Bus`], given a raw *(bus type, bus number)*.
    #[must_use]
    pub fn new_raw_bus(&self, kind: c_short, number: c_short) -> Bus {
        Bus(sensors_bus_id {
            type_: kind,
            nr: number,
        })
    }

    /// Return a new instance of [`Bus`], given a *(bus type, bus number)*.
    #[must_use]
    pub fn new_bus(&self, kind: bus::Kind, number: bus::Number) -> Bus {
        Bus(sensors_bus_id {
            type_: c_short::from(kind),
            nr: number.into(),
        })
    }

    /// Return a new instance of [`BusRef`], given a shared reference
    /// to a raw bus.
    #[must_use]
    pub fn new_bus_ref<'a>(&'a self, raw: &'a sensors_bus_id) -> BusRef<'a> {
        BusRef(raw)
    }

    /// Return a new instance of [`BusMut`], given an exclusive reference
    /// to a raw bus.
    #[must_use]
    pub fn new_bus_mut<'a>(&'a self, raw: &'a mut sensors_bus_id) -> BusMut<'a> {
        BusMut(raw)
    }

    /// Return a new default instance of [`Bus`].
    #[must_use]
    pub fn default_bus(&self) -> Bus {
        Bus(sensors_bus_id {
            type_: SENSORS_BUS_TYPE_ANY as c_short,
            nr: SENSORS_BUS_NR_ANY as c_short,
        })
    }

    /// Return a new instance of [`FeatureRef`] given a shared reference
    /// to a raw feature.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_feature`] reference must have been returned from
    ///   [`sensors_get_features`].
    #[must_use]
    pub unsafe fn new_feature_ref<'a>(
        &'a self,
        chip: ChipRef<'a>,
        raw: &'a sensors_feature,
    ) -> FeatureRef<'a> {
        FeatureRef { chip, raw }
    }

    /// Return a new instance of [`SubFeatureRef`] given a shared reference
    /// to a raw sub-feature.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_subfeature`] reference must have been returned
    ///   either from [`sensors_get_all_subfeatures`] or from
    ///   [`sensors_get_subfeature`].
    #[must_use]
    pub unsafe fn new_sub_feature_ref<'a>(
        &'a self,
        feature: FeatureRef<'a>,
        raw: &'a sensors_subfeature,
    ) -> SubFeatureRef<'a> {
        SubFeatureRef { feature, raw }
    }

    /// Return an iterator which yields all chips matching the given pattern.
    ///
    /// Specifying `None` for the `match_pattern` yields all chips.
    #[must_use]
    pub fn chip_iter<'a>(&'a self, match_pattern: Option<ChipRef<'a>>) -> crate::chip::Iter<'a> {
        crate::chip::Iter {
            state: 0,
            match_pattern,
        }
    }

    /// See: [`sensors_init`].
    fn new(
        config_file_stream: Option<LibCFileStream>,
        error_listener: *mut Box<dyn crate::errors::Listener>,
    ) -> Result<Self> {
        let config_file_fp = config_file_stream
            .as_ref()
            .map_or(ptr::null_mut(), LibCFileStream::as_mut_ptr);

        let locked_self = api_access_lock().lock()?;

        if INITIALIZED.load(atomic::Ordering::Acquire) {
            drop(locked_self); // Unlock early.

            let err = io::ErrorKind::AlreadyExists.into();
            return Err(Error::from_io("sensors_init()", err));
        }

        // We're creating the only instance.
        let error_reporter = Reporter::new(error_listener);

        let r = unsafe { sensors_init(config_file_fp.cast()) };
        if r == 0 {
            INITIALIZED.store(true, atomic::Ordering::Release);

            return Ok(Self { error_reporter });
        }

        // sensors_init() failed.
        // Restore previous global state.
        error_reporter.restore();

        drop(locked_self); // Unlock early.

        Err(Error::from_lm_sensors("sensors_init()", r))
    }
}

impl Drop for LMSensors {
    /// See: [`sensors_cleanup`].
    fn drop(&mut self) {
        let error_listener = api_access_lock()
            .lock()
            .map(move |_guard| {
                unsafe { sensors_cleanup() };

                let error_listener = self.error_reporter.restore();

                INITIALIZED.store(false, atomic::Ordering::Release);

                error_listener
            })
            .unwrap_or(ptr::null_mut());

        if !error_listener.is_null() {
            drop(unsafe { Box::from_raw(error_listener) });
        }
    }
}
