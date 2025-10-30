//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};
use std::{sync::Mutex, time::Instant};

use crate::{Device, INCOMING_PACKETS, SMART_DEVICE_STATES};

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskAdd(
    callback: unsafe extern "C" fn() -> c_int,
    interval: c_int,
    label: *const c_char,
) {
}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void {
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskSleep(time: u32) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskHardwareConcurrency() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBackgroundProcessing() {}

static TASKS_RUN_LAST_CALLED: Mutex<Option<Instant>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn vexTasksRun() {
    let mut last_called = TASKS_RUN_LAST_CALLED.lock().unwrap();
    if let Some(last_called_v) = *last_called {
        if last_called_v.elapsed().as_millis() > 10 {
            *last_called = Some(Instant::now());
            return;
        }
    } else {
        *last_called = Some(Instant::now());
    }
    for (i, packet) in INCOMING_PACKETS.iter().enumerate() {
        let packet = packet.lock().expect("Lock failed").clone();
        if let Some(packet) = packet {
            let mut device = SMART_DEVICE_STATES[i].lock().unwrap();
            device.last_packet = Some(packet);
            device.timestamp = Some(Instant::now());
        }
    }
}
