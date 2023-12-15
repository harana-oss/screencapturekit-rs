use std::ptr;
use objc::{class, runtime::Object, *};
use objc_foundation::{INSDictionary, INSValue, NSData, NSDictionary, NSString, NSValue};
use objc_id::ShareId;

use crate::{as_ptr::AsMutPtr, cv_pixel_buffer_ref::CVPixelBufferRef, macros::declare_ref_type};

declare_ref_type!(CVImageBufferRef);

pub enum ImageFormat {
    JPEG,
    HEIF,
    PNG,
    TIFF
}

impl CVImageBufferRef {
    pub fn as_pixel_buffer(&self) -> ShareId<CVPixelBufferRef> {
        unsafe { ShareId::from_retained_ptr(self.as_mut_ptr().cast()) }
    }
    pub fn get_data(&self, format: ImageFormat) -> ShareId<NSData> {
        unsafe {
            let ci_image_class = class!(CIImage);
            let ci_context_class = class!(CIContext);
            let ci_context: *mut Self = msg_send![ci_context_class, alloc];
            let ci_context: *mut Self = msg_send![ci_context, init];
            let ci_image: *mut Self = msg_send![ci_image_class, alloc];
            let ci_image: *mut Self = msg_send![ci_image, initWithCVImageBuffer: self.as_mut_ptr()];
            let pixel_buffer: *mut Object = msg_send![ci_image, pixelBuffer];
            let color_space = CVImageBufferGetColorSpace(pixel_buffer);

            let data: *mut NSData = match format {
                ImageFormat::JPEG   => {
                    let options = NSDictionary::from_keys_and_objects(&[&*kCGImageDestinationLossyCompressionQuality], vec![NSValue::from_value(1000.0f32)]);
                    msg_send![ci_context, JPEGRepresentationOfImage: ci_image colorSpace: color_space options: options]
                },
                ImageFormat::HEIF   => msg_send![ci_context, HEIFRepresentationOfImage: ci_image colorSpace: color_space options: ptr::null_mut::<Object>()],
                ImageFormat::PNG    => msg_send![ci_context, PNGRepresentationOfImage: ci_image colorSpace: color_space options: ptr::null_mut::<Object>()],
                ImageFormat::TIFF   => msg_send![ci_context, TIFFRepresentationOfImage: ci_image colorSpace: color_space options: ptr::null_mut::<Object>()]
            };

            ShareId::from_ptr(data)
        }
    }
}

extern "C" {
    #[allow(improper_ctypes)]
    static kCGImageDestinationLossyCompressionQuality: *const NSString;

    fn CVImageBufferGetColorSpace(image: *mut Object) -> *mut Object;
}
