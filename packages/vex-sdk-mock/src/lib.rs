#![feature(c_variadic)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(private_interfaces)]

use std::{os::raw::c_double, sync::{Mutex, atomic::AtomicBool}, time::Instant};

use vex_sdk::{V5_DeviceBumperState, V5_DeviceType, V5MotorEncoderUnits, V5MotorGearset};

static INCOMING_PACKETS: [Mutex<Option<DevicePacket>>; 22] = [
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
    Mutex::new(None),
];
static SMART_DEVICE_STATES: [Mutex<Device>; 22] = [
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
    Mutex::new(Device::const_default()),
];

#[derive(Debug, Clone)]
struct DistancePacket {
    distance: u32,
    confidence: u32,
    status: u32,
    size: i32,
    velocity: c_double,
}

/// A device-agnostic type for the most recent packet received on a port.
/// Eventually will include types for all devices in the SDK.
#[derive(Clone)]
enum DevicePacket {
    Distance(DistancePacket), // need to add more ofc
}

/// DeviceState represents the internal state of a device.
/// It includes the last packet received from the port, the timestamp of the
/// packet, whether the packet is from a generic serial device, and a cache
/// for motor-specific data.
///
/// It is using the DEVICE_STATES global static array and is updated by `vexTasksRun`.
#[derive(Clone)]
struct Device {
    last_packet: Option<DevicePacket>,
    /// last device packet timestamp
    timestamp: Option<Instant>,
    /// SDK methods ignore last_packet if this is set.
    is_generic_serial: bool,
    /// When a motor disconnects, its gearset/direction/etc... is still
    /// saved locally in VEXos so it doesnt break when it reconnects. This
    /// is where gearset/direction is stored, also probably the last position.
    ///
    /// A lot of other devices do this too, like ADI saves port configurations
    /// if an ADI expander disconnects, so we'd have a cache for every device
    /// type that does this.
    motor_cache: MotorCache,
}
impl Default for Device {
    fn default() -> Self {
        Self {
            last_packet: None,
            timestamp: None,
            is_generic_serial: false,
            motor_cache: MotorCache::default(),
        }
    }
}
impl Device {
    const fn const_default() -> Self {
        Self {
            last_packet: None,
            timestamp: None,
            is_generic_serial: false,
            motor_cache: MotorCache {
                gearset: V5MotorGearset::kMotorGearSet_18,
                reverse_flag: 0,
                encoder_units: V5MotorEncoderUnits::kMotorEncoderDegrees,
            },
        }
    }

    fn device_type(&self) -> V5_DeviceType {
        match self.last_packet {
            None => V5_DeviceType::kDeviceTypeNoSensor,
            Some(DevicePacket::Distance { .. }) => V5_DeviceType::kDeviceTypeDistanceSensor,
            _ => V5_DeviceType::kDeviceTypeUndefinedSensor,
        }
    }
}

/// SmartPort represents the state of a smart port.
/// field 0 is between 0 and 21.
struct SmartPort(u8);

impl SmartPort {
    fn send_packet(&self, packet: DevicePacket) {
        *INCOMING_PACKETS[self.0 as usize].lock().unwrap() = Some(packet);
    }

}

#[derive(Clone)]
struct MotorCache {
    gearset: V5MotorGearset,
    reverse_flag: i32,
    encoder_units: V5MotorEncoderUnits,
}
impl Default for MotorCache {
    fn default() -> Self {
        Self {
            gearset: Default::default(),
            reverse_flag: Default::default(),
            encoder_units: Default::default(),
        }
    }
}

struct Brain {
    /// Smart Port 1 on the Brain
    pub port_1: SmartPort,
    /// Smart Port 2 on the Brain
    pub port_2: SmartPort,
    /// Smart Port 3 on the Brain
    pub port_3: SmartPort,
    /// Smart Port 4 on the Brain
    pub port_4: SmartPort,
    /// Smart Port 5 on the Brain
    pub port_5: SmartPort,
    /// Smart Port 6 on the Brain
    pub port_6: SmartPort,
    /// Smart Port 7 on the Brain
    pub port_7: SmartPort,
    /// Smart Port 8 on the Brain
    pub port_8: SmartPort,
    /// Smart Port 9 on the Brain
    pub port_9: SmartPort,
    /// Smart Port 10 on the Brain
    pub port_10: SmartPort,
    /// Smart Port 11 on the Brain
    pub port_11: SmartPort,
    /// Smart Port 12 on the Brain
    pub port_12: SmartPort,
    /// Smart Port 13 on the Brain
    pub port_13: SmartPort,
    /// Smart Port 14 on the Brain
    pub port_14: SmartPort,
    /// Smart Port 15 on the Brain
    pub port_15: SmartPort,
    /// Smart Port 16 on the Brain
    pub port_16: SmartPort,
    /// Smart Port 17 on the Brain
    pub port_17: SmartPort,
    /// Smart Port 18 on the Brain
    pub port_18: SmartPort,
    /// Smart Port 19 on the Brain
    pub port_19: SmartPort,
    /// Smart Port 20 on the Brain
    pub port_20: SmartPort,
    /// Smart Port 21 on the Brain
    pub port_21: SmartPort,
    // TODO: add brain, ADI, controllers
}


static PERIPHERALS_TAKEN: AtomicBool = AtomicBool::new(false);
impl Brain {
    unsafe fn new() -> Self {
        unsafe {
            Self {
                port_1: SmartPort(1),
                port_2: SmartPort(2),
                port_3: SmartPort(3),
                port_4: SmartPort(4),
                port_5: SmartPort(5),
                port_6: SmartPort(6),
                port_7: SmartPort(7),
                port_8: SmartPort(8),
                port_9: SmartPort(9),
                port_10: SmartPort(10),
                port_11: SmartPort(11),
                port_12: SmartPort(12),
                port_13: SmartPort(13),
                port_15: SmartPort(15),
                port_14: SmartPort(14),
                port_16: SmartPort(16),
                port_17: SmartPort(17),
                port_18: SmartPort(18),
                port_19: SmartPort(19),
                port_20: SmartPort(20),
                port_21: SmartPort(21),
            }
        }
    }
    pub fn take() -> Option<Self> {
        if PERIPHERALS_TAKEN.swap(true, core::sync::atomic::Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::new() })
        }
    }
}

// if this builds then we good
fn test() {
    let brain = Brain::take().unwrap();
    brain.port_1.send_packet(DevicePacket::Distance(DistancePacket {
        distance: todo!(),
        confidence: todo!(),
        status: todo!(),
        size: todo!(),
        velocity: todo!(),
    }));
}

pub mod sdk;
