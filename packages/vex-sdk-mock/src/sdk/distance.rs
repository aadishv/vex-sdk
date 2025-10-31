//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceType;

use crate::sdk::device::V5_DeviceT;
use crate::{DEVICES, DevicePacket, DeviceState, DistancePacket};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DeviceState::Distance(state)) = device.state.as_ref() {
        let distance = state.distance;
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

    if let Some(DeviceState::Distance(state)) = device.state.as_ref() {
        state.confidence
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DeviceState::Distance(state)) = device.state.as_ref() {
        state.status
    } else {
        Default::default()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DeviceState::Distance(state)) = device.state.as_ref() {
        state.object_size
    } else {
        -1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    let device = unsafe { (*device).lock() }.unwrap();

    if let Some(DeviceState::Distance(state)) = device.state.as_ref() {
        state.velocity
    } else {
        0.0
    }
}
