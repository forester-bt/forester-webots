use std::ffi::CString;
use crate::bindings::WbDeviceTag;

pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings.rs");
}

pub fn wb_touch_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_touch_sensor_enable(tag, sampling_period)
    }
}

pub fn wb_position_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_position_sensor_enable(tag, sampling_period)
    }
}

pub fn wb_receiver_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_receiver_enable(tag, sampling_period)
    }
}

pub fn wb_robot_get_basic_time_step() -> f64 {
    unsafe {
        bindings::wb_robot_get_basic_time_step()
    }
}

pub fn wb_distance_sensor_enable(tag: WbDeviceTag, sampling_period: i32) {
    unsafe {
        bindings::wb_distance_sensor_enable(tag, sampling_period);
    }
}

pub fn wb_distance_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe { bindings::wb_distance_sensor_get_value(tag) }
}

pub fn wb_motor_set_position(device: WbDeviceTag, position: f64) {
    unsafe { bindings::wb_motor_set_position(device, position) }
}

pub fn wb_motor_set_velocity(device: WbDeviceTag, velocity: f64) {
    unsafe { bindings::wb_motor_set_velocity(device, velocity) }
}

pub fn wb_touch_sensor_get_value(device: WbDeviceTag) -> f64 {
    unsafe { bindings::wb_touch_sensor_get_value(device) }
}

pub fn wb_robot_get_device(id: &str) -> WbDeviceTag {
    let name = CString::new(id).expect("CString::new failed");
    unsafe { bindings::wb_robot_get_device(name.as_ptr()) }
}

pub fn wb_robot_cleanup() {
    unsafe { bindings::wb_robot_cleanup() }
}

pub fn wb_receiver_get_queue_length(device: WbDeviceTag) -> i32 {
    unsafe { bindings::wb_receiver_get_queue_length(device) }
}

pub fn wb_receiver_next_packet(device: WbDeviceTag) {
    unsafe { bindings::wb_receiver_next_packet(device) }
}

pub fn wb_robot_init() -> i32 {
    unsafe {
        bindings::wb_robot_init()
    }
}

pub fn wb_robot_get_time() -> f64 {
    unsafe {
        bindings::wb_robot_get_time()
    }
}

pub fn wb_position_sensor_get_value(tag: WbDeviceTag) -> f64 {
    unsafe {
        bindings::wb_position_sensor_get_value(tag)
    }
}

pub fn wb_led_set(tag: WbDeviceTag, value: i32) {
    unsafe {
        bindings::wb_led_set(tag, value);
    }
}

pub fn wb_robot_step(step: i32) -> i32 {
    unsafe { bindings::wb_robot_step(step) }
}

#[cfg(test)]
pub mod tests {
    use crate::wb_robot_init;

    #[test]
    pub fn test() {
        let res = wb_robot_init();
        println!("{}", res);
    }
}