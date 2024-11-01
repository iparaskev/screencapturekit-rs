mod internal {
    use std::sync::mpsc::{channel, Receiver};
    use std::vec;

    use crate::sc_content_filter::SCContentFilter;
    use block::{ConcreteBlock, RcBlock};
    use core_foundation::base::{CFTypeID, TCFType};
    use core_foundation::{declare_TCFType, impl_TCFType};
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl, Encode, Message};
    use objc_foundation::INSData;
    use objc_id::Id;
    use screencapturekit_sys::cm_sample_buffer_ref::CMSampleBufferRef;
    use screencapturekit_sys::stream_configuration::UnsafeStreamConfigurationRef;

    extern "C" {
        pub fn SCScreenshotManagerGetTypeID() -> CFTypeID;
    }

    type CompletionHandlerBlock = RcBlock<(*mut Object, *mut Object), ()>;
    unsafe fn new_completion_handler() -> (CompletionHandlerBlock, Receiver<Result<(Vec<u8>), String>>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |sample_buffer: *mut Object, error: *mut Object| {
            let mut buffer = vec![];
            if error.is_null() {
                let sample_buffer: Id<CMSampleBufferRef> = Id::from_ptr(sample_buffer.cast::<CMSampleBufferRef>());
                let image_buffer = sample_buffer.get_image_buffer().unwrap();
                let jpeg_data = image_buffer.get_jpeg_data();
                buffer.extend_from_slice(jpeg_data.bytes());
            } else {
                tx.send(Err("error".to_string()))
                    .expect("Cannot send error message");
            }
            tx.send(Ok(buffer)).expect("Cannot send message");
        });
        (handler.copy(), rx)
    }
    pub fn capture(
        //screenshot_manager: &SCScreenshotManager,
        content_filter: SCContentFilter,
        config: Id<UnsafeStreamConfigurationRef>,
    ) -> Vec<u8> {
        let filter = content_filter._unsafe_ref;
        unsafe {
            let (handler, rx) = new_completion_handler();
            let _: () = msg_send![class!(SCScreenshotManager), captureSampleBufferWithFilter: filter configuration: config completionHandler: handler];
            let res = rx.recv()
                .expect("Should receive a return from completion handler");
            res.unwrap()
        }
    }
}

use self::internal::{capture};
use crate::{
    sc_content_filter::SCContentFilter, sc_display::SCDisplay,
    sc_stream_configuration::SCStreamConfiguration,
};
pub struct SCScreenshotManager();

impl  SCScreenshotManager {
    pub fn capture(content_filter: SCContentFilter, stream_config: SCStreamConfiguration) -> Vec<u8> {
        capture(content_filter, stream_config.into())
    }
}