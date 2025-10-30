//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceType;

use crate::sdk::device::V5_DeviceT;
use crate::{DevicePacket, DistancePacket, SMART_DEVICE_STATES};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DevicePacket::Distance(packet)) = device.last_packet.clone() {
        let distance = packet.distance;
        if distance == 0 {
            9999 // object out of range
        } else {
            distance // found an object
        }
    } else {
        0 // not connected
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DevicePacket::Distance(packet)) = device.last_packet.clone() {
        packet.confidence
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DevicePacket::Distance(packet)) = device.last_packet.clone() {
        packet.status
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DevicePacket::Distance(packet)) = device.last_packet.clone() {
        packet.size
    } else {
        -1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DevicePacket::Distance(packet)) = device.last_packet.clone() {
        packet.velocity
    } else {
        0.0
    }
}
