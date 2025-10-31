//! V5 Rotation Sensor

use crate::sdk::device::V5_DeviceT;
use crate::{DevicePacket, AbsEncPacket};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReset(device: V5_DeviceT) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32) {
    let mut device = unsafe { (*device).lock() }.unwrap();
    device.abs_enc_cache.reset_position = position;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { (*device).lock() }.unwrap();
    if let Some(DevicePacket::AbsEnc(packet)) = device.last_packet.clone() {
        // packet.position is the raw tick count, so we add the cached tare
        // position (which persists across connections, by virtue of being
        // cached)
        (packet.position + device.abs_enc_cache.reset_position) * (if device.abs_enc_cache.reverse_flag { -1 } else { 1 })
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { (*device).lock() }.unwrap();
    if let Some(DevicePacket::AbsEnc(packet)) = device.last_packet.clone() {
        packet.velocity
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { (*device).lock() }.unwrap();
    if let Some(DevicePacket::AbsEnc(packet)) = device.last_packet.clone() {
        packet.angle
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool) {
    let mut device = unsafe { (*device).lock() }.unwrap();
    device.abs_enc_cache.reverse_flag = value;
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool {
    let device = unsafe { (*device).lock() }.unwrap();
    device.abs_enc_cache.reverse_flag
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();
    if let Some(DevicePacket::AbsEnc(packet)) = device.last_packet.clone() {
        packet.status
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32) {
    let mut device = unsafe { (*device).lock() }.unwrap();
    device.abs_enc_cache.data_rate = rate;
}
