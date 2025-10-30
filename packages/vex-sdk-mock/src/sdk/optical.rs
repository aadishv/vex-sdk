//! V5 Optical Sensor

use core::ffi::c_double;

use crate::sdk::device::V5_DeviceT;
pub use vex_sdk::{V5_DeviceOpticalGesture, V5_DeviceOpticalRaw, V5_DeviceOpticalRgb};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureGet(
    device: V5_DeviceT,
    pData: *mut V5_DeviceOpticalGesture,
) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureEnable(device: V5_DeviceT) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureDisable(device: V5_DeviceT) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
