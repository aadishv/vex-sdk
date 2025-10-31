//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};
use std::{
    sync::{LazyLock, Mutex},
    time::Instant,
};

use crate::{DEVICES, Device, DevicePacket, DeviceState, INCOMING_PACKETS, sdk::vexSystemTimeGet};

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
pub extern "C" fn vexTaskSleep(time: u32) {
    // "spam" vexTasksRun
    let start = Instant::now();
    while start.elapsed().as_millis() < time as u128 {
        // this yields so we don't need to worry about starvation
        vexTasksRun();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskHardwareConcurrency() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBackgroundProcessing() {
    // If this were a real brain, we'd print a "Program Error: SDK Mismatch" log
    //
    // TODO: If we implement the event log, do this.
}

struct SimpleTask {
    label: &'static str,
    callback: extern "C" fn(),
    interval: u32,
    timestamp: u32,
}

const SIMPLE_TASKS: [Mutex<SimpleTask>; 3] = [
    // Processes incoming device packets
    Mutex::new(SimpleTask {
        label: "V5_Device",
        callback: {
            extern "C" fn device_task() {
                for (i, packet) in INCOMING_PACKETS.iter().enumerate() {
                    if let Some(packet) = packet.lock().unwrap().as_ref() {
                        let mut device = DEVICES[i].lock().unwrap();
                        device.update(packet);
                    }
                }
            }

            device_task
        },
        interval: 10,
        timestamp: 0,
    }),
    // Flushes USB buffers
    Mutex::new(SimpleTask {
        label: "V5_Main",
        callback: {
            extern "C" fn main_task() {
                // TODO: Flush serial
            }

            main_task
        },
        interval: 1,
        timestamp: 0,
    }),
    // Touchscreen data for vexTouch*
    Mutex::new(SimpleTask {
        label: "V5_Touch",
        callback: {
            extern "C" fn touch_task() {
                // TODO: Flush serial
            }

            touch_task
        },
        interval: 5,
        timestamp: 0,
    }),
];

#[unsafe(no_mangle)]
pub extern "C" fn vexTasksRun() {
    for task in SIMPLE_TASKS {
        let mut task = task.lock().unwrap();
        let time = vexSystemTimeGet();

        if (task.timestamp <= time) {
            task.timestamp = time + task.interval;
            (task.callback)();
        }
    }
    std::thread::yield_now();
}
