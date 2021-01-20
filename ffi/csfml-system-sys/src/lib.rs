/* automatically generated by rust-bindgen */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)] // added manually

pub const CSFML_VERSION_MAJOR: u32 = 2;
pub const CSFML_VERSION_MINOR: u32 = 5;
pub const CSFML_VERSION_PATCH: u32 = 0;
pub const sfFalse: sfBool = 0; // edited manually
pub const sfTrue: sfBool = 1; // edited manually
pub type sfBool = ::std::os::raw::c_int;
pub type sfInt8 = ::std::os::raw::c_schar;
pub type sfUint8 = ::std::os::raw::c_uchar;
pub type sfInt16 = ::std::os::raw::c_short;
pub type sfUint16 = ::std::os::raw::c_ushort;
pub type sfInt32 = ::std::os::raw::c_int;
pub type sfUint32 = ::std::os::raw::c_uint;
pub type sfInt64 = ::std::os::raw::c_longlong;
pub type sfUint64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfTime {
    pub microseconds: sfInt64,
}
#[test]
fn bindgen_test_layout_sfTime() {
    assert_eq!(
        ::std::mem::size_of::<sfTime>(),
        8usize,
        concat!("Size of: ", stringify!(sfTime))
    );
    assert_eq!(
        ::std::mem::align_of::<sfTime>(),
        8usize,
        concat!("Alignment of ", stringify!(sfTime))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfTime>())).microseconds as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfTime),
            "::",
            stringify!(microseconds)
        )
    );
}
extern "C" {
    pub static mut sfTime_Zero: sfTime;
}
extern "C" {
    pub fn sfTime_asSeconds(time: sfTime) -> f32;
}
extern "C" {
    pub fn sfTime_asMilliseconds(time: sfTime) -> sfInt32;
}
extern "C" {
    pub fn sfTime_asMicroseconds(time: sfTime) -> sfInt64;
}
extern "C" {
    pub fn sfSeconds(amount: f32) -> sfTime;
}
extern "C" {
    pub fn sfMilliseconds(amount: sfInt32) -> sfTime;
}
extern "C" {
    pub fn sfMicroseconds(amount: sfInt64) -> sfTime;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfClock {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfMutex {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfThread {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sfClock_create() -> *mut sfClock;
}
extern "C" {
    pub fn sfClock_copy(clock: *const sfClock) -> *mut sfClock;
}
extern "C" {
    pub fn sfClock_destroy(clock: *mut sfClock);
}
extern "C" {
    pub fn sfClock_getElapsedTime(clock: *const sfClock) -> sfTime;
}
extern "C" {
    pub fn sfClock_restart(clock: *mut sfClock) -> sfTime;
}
pub type sfInputStreamReadFunc = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        size: sfInt64,
        userData: *mut ::std::os::raw::c_void,
    ) -> sfInt64,
>;
pub type sfInputStreamSeekFunc = ::std::option::Option<
    unsafe extern "C" fn(position: sfInt64, userData: *mut ::std::os::raw::c_void) -> sfInt64,
>;
pub type sfInputStreamTellFunc =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void) -> sfInt64>;
pub type sfInputStreamGetSizeFunc =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void) -> sfInt64>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfInputStream {
    pub read: sfInputStreamReadFunc,
    pub seek: sfInputStreamSeekFunc,
    pub tell: sfInputStreamTellFunc,
    pub getSize: sfInputStreamGetSizeFunc,
    pub userData: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_sfInputStream() {
    assert_eq!(
        ::std::mem::size_of::<sfInputStream>(),
        40usize,
        concat!("Size of: ", stringify!(sfInputStream))
    );
    assert_eq!(
        ::std::mem::align_of::<sfInputStream>(),
        8usize,
        concat!("Alignment of ", stringify!(sfInputStream))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfInputStream>())).read as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfInputStream),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfInputStream>())).seek as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sfInputStream),
            "::",
            stringify!(seek)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfInputStream>())).tell as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sfInputStream),
            "::",
            stringify!(tell)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfInputStream>())).getSize as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sfInputStream),
            "::",
            stringify!(getSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfInputStream>())).userData as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(sfInputStream),
            "::",
            stringify!(userData)
        )
    );
}
extern "C" {
    pub fn sfMutex_create() -> *mut sfMutex;
}
extern "C" {
    pub fn sfMutex_destroy(mutex: *mut sfMutex);
}
extern "C" {
    pub fn sfMutex_lock(mutex: *mut sfMutex);
}
extern "C" {
    pub fn sfMutex_unlock(mutex: *mut sfMutex);
}
extern "C" {
    pub fn sfSleep(duration: sfTime);
}
extern "C" {
    pub fn sfThread_create(
        function: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        userData: *mut ::std::os::raw::c_void,
    ) -> *mut sfThread;
}
extern "C" {
    pub fn sfThread_destroy(thread: *mut sfThread);
}
extern "C" {
    pub fn sfThread_launch(thread: *mut sfThread);
}
extern "C" {
    pub fn sfThread_wait(thread: *mut sfThread);
}
extern "C" {
    pub fn sfThread_terminate(thread: *mut sfThread);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfVector2i {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfVector2i() {
    assert_eq!(
        ::std::mem::size_of::<sfVector2i>(),
        8usize,
        concat!("Size of: ", stringify!(sfVector2i))
    );
    assert_eq!(
        ::std::mem::align_of::<sfVector2i>(),
        4usize,
        concat!("Alignment of ", stringify!(sfVector2i))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2i>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2i),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2i>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2i),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfVector2u {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfVector2u() {
    assert_eq!(
        ::std::mem::size_of::<sfVector2u>(),
        8usize,
        concat!("Size of: ", stringify!(sfVector2u))
    );
    assert_eq!(
        ::std::mem::align_of::<sfVector2u>(),
        4usize,
        concat!("Alignment of ", stringify!(sfVector2u))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2u>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2u),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2u>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2u),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfVector2f {
    pub x: f32,
    pub y: f32,
}
#[test]
fn bindgen_test_layout_sfVector2f() {
    assert_eq!(
        ::std::mem::size_of::<sfVector2f>(),
        8usize,
        concat!("Size of: ", stringify!(sfVector2f))
    );
    assert_eq!(
        ::std::mem::align_of::<sfVector2f>(),
        4usize,
        concat!("Alignment of ", stringify!(sfVector2f))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2f>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2f),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector2f>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector2f),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfVector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_sfVector3f() {
    assert_eq!(
        ::std::mem::size_of::<sfVector3f>(),
        12usize,
        concat!("Size of: ", stringify!(sfVector3f))
    );
    assert_eq!(
        ::std::mem::align_of::<sfVector3f>(),
        4usize,
        concat!("Alignment of ", stringify!(sfVector3f))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector3f>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector3f),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector3f>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector3f),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sfVector3f>())).z as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sfVector3f),
            "::",
            stringify!(z)
        )
    );
}
