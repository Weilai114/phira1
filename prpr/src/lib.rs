pub mod bin;
pub mod config;
pub mod core;
pub mod dir;
pub mod ext;
pub mod fs;
pub mod info;
pub mod judge;
pub mod parse;
pub mod particle;
pub mod scene;
pub mod task;
pub mod time;
pub mod ui;

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "closed")]
pub mod inner;

#[cfg(target_os = "ios")]
pub mod objc;

pub use scene::Main;

pub fn build_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Phira".to_string(),
        window_width: 973,
        window_height: 608,
        ..Default::default()
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern "C" fn Java_quad_1native_QuadNative_preprocessInput(
    _: *mut std::ffi::c_void,
    _: *const std::ffi::c_void,
    #[allow(dead_code)] motionEvent: ndk_sys::AInputEvent,
    #[allow(dead_code)] f: ndk_sys::jfloat,
    #[allow(dead_code)] f2: ndk_sys::jfloat,
    #[allow(dead_code)] z: ndk_sys::jboolean,
    #[allow(dead_code)] z2: ndk_sys::jboolean,
) {
    
}
