use screencapturekit_sys::{
    cm_sample_buffer_ref::CMSampleBufferRef,
    cv_image_buffer_ref::CVImageBufferRef,
    os_types::rc::{Id, ShareId},
    sc_stream_frame_info::{SCFrameStatus, SCScreenRect},
};

use crate::cv_pixel_buffer::CVPixelBuffer;

#[derive(Debug)]
pub struct CMSampleBuffer {
    pub sys_ref: Id<CMSampleBufferRef>,
    pub image_buf_ref: Option<ShareId<CVImageBufferRef>>,
    pub pixel_buffer: Option<CVPixelBuffer>,
    pub frame_status: SCFrameStatus,
    pub frame_scale: f64,
    pub screen_rect: SCScreenRect,
}

impl CMSampleBuffer {
    pub fn new(sys_ref: Id<CMSampleBufferRef>) -> Self {
        let frame_status = sys_ref.get_frame_info().status();
        let frame_scale = sys_ref.get_frame_info().scale();
        let screen_rect = sys_ref.get_frame_info().screen_rect();
        let image_buf_ref = sys_ref.get_image_buffer();
        let pixel_buffer = image_buf_ref
            .as_ref()
            .map(|i| CVPixelBuffer::new(i.clone().as_pixel_buffer()));
        Self {
            sys_ref,
            pixel_buffer,
            image_buf_ref,
            frame_status,
            frame_scale,
            screen_rect,
        }
    }
}
