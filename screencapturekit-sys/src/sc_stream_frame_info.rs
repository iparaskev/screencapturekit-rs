use std::{mem, os::{macos::raw, raw::c_void}};

use objc::{Message, *};
use objc_foundation::{INSString, INSValue, NSString, NSValue};
use runtime::{Class, Object};

use crate::os_types::geometry::CGRect;
#[derive(Debug)]
#[repr(C)]
pub struct SCStreamFrameInfo {
    _priv: [u8; 0],
}

// TODO: Documnent using comment docs matching apple
#[derive(Debug)]
#[repr(i32)]
pub enum SCFrameStatus {
    // A status that indicates the system successfully generated a new frame.
    Complete,
    // A status that indicates the system didn’t generate a new frame because the display didn’t change.
    Idle,
    // A status that indicates the system didn’t generate a new frame because the display is blank.
    Blank,
    // A status that indicates the system didn’t generate a new frame because you suspended updates.
    Suspended,
    // A status that indicates the frame is the first one sent after the stream starts.
    Started,
    // A status that indicates the frame is in a stopped state.
    Stopped,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SCScreenRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

unsafe impl Message for SCStreamFrameInfo {}
impl SCStreamFrameInfo {
    pub fn status(&self) -> SCFrameStatus {
        unsafe {
            let key = NSString::from_str("SCStreamUpdateFrameStatus");
            let raw_status: *mut NSValue<i32> = msg_send!(self, objectForKey: key);
            if raw_status.is_null() {
                return SCFrameStatus::Idle;
            }
            mem::transmute((*raw_status).value())
        }
    }

    pub fn scale(&self) -> f64 {
        unsafe {
            let key = NSString::from_str("SCStreamUpdateFrameContentScale");
            let raw_scale: *mut NSValue<f64> = msg_send!(self, objectForKey: key);
            if raw_scale.is_null() {
                return 1.0;
            }
            (*raw_scale).value()
        }
    }

    pub fn screen_rect(&self) -> SCScreenRect {
        unsafe {
            let key = NSString::from_str("SCStreamUpdateFrameScreenRect");
            let rect_dict: *mut Object = msg_send!(self, objectForKey: key);
            let raw_width: *mut NSValue<f64> = msg_send!(rect_dict, objectForKey: NSString::from_str("Width"));
            let raw_height: *mut NSValue<f64> = msg_send!(rect_dict, objectForKey: NSString::from_str("Height"));
            let raw_x: *mut NSValue<f64> = msg_send!(rect_dict, objectForKey: NSString::from_str("X"));
            let raw_y: *mut NSValue<f64> = msg_send!(rect_dict, objectForKey: NSString::from_str("Y"));
            if raw_width.is_null() || raw_height.is_null() || raw_x.is_null() || raw_y.is_null() {
                return SCScreenRect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                };
            }
            SCScreenRect {
                x: (*raw_x).value(),
                y: (*raw_y).value(),
                width: (*raw_width).value(),
                height: (*raw_height).value(),
            }
        }
    }
}