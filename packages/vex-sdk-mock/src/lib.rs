#![feature(c_variadic)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    os::raw::c_double,
    sync::{LazyLock, Mutex, atomic::AtomicBool},
};

use vex_sdk::{V5_DeviceType, V5MotorEncoderUnits, V5MotorGearset};

use crate::sdk::{SYSTEM_TIME_START, vexSystemTimeGet};

pub mod sdk;

// caches for specific sensors
#[derive(Clone)]
struct MotorCache {
    gearset: V5MotorGearset,
    reverse_flag: i32,
    encoder_units: V5MotorEncoderUnits,
}
impl MotorCache {
    const fn const_default() -> Self {
        MotorCache {
            gearset: V5MotorGearset::kMotorGearSet_18,
            reverse_flag: 0,
            encoder_units: V5MotorEncoderUnits::kMotorEncoderDegrees,
        }
    }
}
impl Default for MotorCache {
    fn default() -> Self {
        Self::const_default()
    }
}

#[derive(Clone)]
struct AbsEncCache {
    position_offset: i32,
    reverse_flag: bool,
    data_rate: u32,
}
impl AbsEncCache {
    const fn const_default() -> Self {
        AbsEncCache {
            position_offset: 0,
            reverse_flag: false,
            data_rate: 0,
        }
    }
}
impl Default for AbsEncCache {
    fn default() -> Self {
        Self::const_default()
    }
}

/// Should be called by consumers of this library in main.
pub fn init() {
    LazyLock::force(&SYSTEM_TIME_START);
}

static INCOMING_PACKETS: [Mutex<Option<DevicePacket>>; 23] = [
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
    Mutex::new(None),
];
static DEVICES: [Mutex<Device>; 23] = [
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
    Mutex::new(Device::const_default()),
];

#[derive(Debug, Clone)]
pub struct DistancePacket {
    pub distance: u16,
    pub confidence: u8,
    pub status: u8,
    /// The number of photons that hit a "reference" in a background.
    pub ref_hits: u32,
    /// The number of photons that hit the object we are measuring.
    pub obj_hits: u32,
}
#[derive(Debug, Clone)]
pub struct DistanceState {
    pub distance: u32,
    pub confidence: u32,
    pub status: u32,
    pub object_size: i32,
    pub velocity: c_double,
    /// ???
    pub detection_count: u32,
}

#[derive(Debug, Clone)]
pub struct AbsEncPacket {
    status: u32,
    angle: i32,
    velocity: i32,
    position: i32,
}

/// A device-agnostic type for the most recent packet received on a port.
/// Eventually will include types for all devices in the SDK.
#[derive(Clone)]
pub enum DevicePacket {
    Distance(DistancePacket),
    AbsEnc(AbsEncPacket),
}
/// A device-agnostic type for derived state from devices. Generated in
/// vexTasksRun.
#[derive(Clone)]
pub enum DeviceState {
    Distance(DistanceState),
}
/// DeviceState represents the internal state of a device.
/// It includes the last packet received from the port, the timestamp of the
/// packet, whether the packet is from a generic serial device, and a cache
/// for motor-specific data.
///
/// It is using the DEVICE_STATES global static array and is updated by `vexTasksRun`.
#[derive(Clone)]
pub struct Device {
    /// State generated from the most recently received packet.
    state: Option<DeviceState>,

    /// timestamp of the last packet processing
    timestamp: u32,

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
    abs_enc_cache: AbsEncCache,
}
impl Device {
    const fn const_default() -> Self {
        Device {
            state: None,
            timestamp: 0,
            is_generic_serial: false,
            motor_cache: MotorCache::const_default(),
            abs_enc_cache: AbsEncCache::const_default(),
        }
    }

    fn device_type(&self) -> V5_DeviceType {
        match self.state {
            Some(DeviceState::Distance { .. }) => V5_DeviceType::kDeviceTypeDistanceSensor,
            _ => todo!(),
        }
    }
}
impl Default for Device {
    fn default() -> Self {
        Device::const_default()
    }
}
impl Device {
    fn update(&mut self, packet: &DevicePacket) {
        let new_time = vexSystemTimeGet();
        self.state = match packet {
            DevicePacket::Distance(packet) => {
                let ratio = (packet.ref_hits as f32) / (packet.obj_hits as f32);
                let distance = packet.distance as f32;

                let object_size = 400.min(((distance * 80.0) / (ratio.sqrt() * 0.25 * 1000.0)) as i32);

                let detection_count: u32;
                let velocity: c_double;
                let detected = if let Some(DeviceState::Distance(old)) = self.state.as_ref() {
                    let dist = packet.distance as u32;
                    let detected = if dist <= 25 {
                        detection_count = if dist == 0 { 0 } else { 4 };
                        dist != 0
                    } else {
                        if packet.confidence <= 12 {
                            detection_count = 0;
                            false
                        } else if old.detection_count > 3 {
                            detection_count = old.detection_count;
                            true
                        } else {
                            detection_count = old.detection_count + 1;
                            false
                        }
                    };
                    if detected {
                        velocity = (old.distance - dist) as c_double
                            / (self.timestamp - new_time) as c_double;
                    } else {
                        velocity = 0.0;
                    }
                    detected
                } else {
                    detection_count = 0;
                    velocity = 0.0;
                    false
                };
                Some(DeviceState::Distance(DistanceState {
                    distance: packet.distance as _,
                    confidence: packet.confidence as _,
                    status: packet.status as _,
                    object_size: if detected { object_size } else { -1 },
                    velocity,
                    detection_count,
                }))
            }
            _ => todo!(),
        };
        self.timestamp = new_time;
    }
}

/// SmartPort represents the state of a smart port.
/// `index` is between 0 and 21.
pub struct SmartPort {
    index: u8,
}

impl SmartPort {
    pub fn send_packet(&self, packet: DevicePacket) {
        *INCOMING_PACKETS[self.index as usize].lock().unwrap() = Some(packet);
    }
}

pub struct Brain {
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
        Self {
            port_1: SmartPort { index: 0 },
            port_2: SmartPort { index: 1 },
            port_3: SmartPort { index: 2 },
            port_4: SmartPort { index: 3 },
            port_5: SmartPort { index: 4 },
            port_6: SmartPort { index: 5 },
            port_7: SmartPort { index: 6 },
            port_8: SmartPort { index: 7 },
            port_9: SmartPort { index: 8 },
            port_10: SmartPort { index: 9 },
            port_11: SmartPort { index: 10 },
            port_12: SmartPort { index: 11 },
            port_13: SmartPort { index: 12 },
            port_15: SmartPort { index: 13 },
            port_14: SmartPort { index: 14 },
            port_16: SmartPort { index: 15 },
            port_17: SmartPort { index: 16 },
            port_18: SmartPort { index: 17 },
            port_19: SmartPort { index: 18 },
            port_20: SmartPort { index: 19 },
            port_21: SmartPort { index: 20 },
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
    init();

    let brain = Brain::take().unwrap();
    brain
        .port_1
        .send_packet(DevicePacket::Distance(DistancePacket {
            distance: todo!(),
            confidence: todo!(),
            status: todo!(),
            ref_hits: todo!(),
            obj_hits: todo!(),
        }));
}
