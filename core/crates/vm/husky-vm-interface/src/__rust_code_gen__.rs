// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

// void
#[no_mangle]
pub unsafe extern "C" fn __void_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_void;
    false
}
#[no_mangle]
pub unsafe extern "C" fn __void_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_void;
    let ptr: *mut void = Box::<void>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __void_clone(data: *mut ()) -> *mut () {
    Box::<void>::into_raw(Box::new((*(data as *mut void)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __void_drop(data: *mut ()) {
    Box::from_raw(data as *mut void);
}
extern "C" {
    pub static __VOID_VTABLE: __RegisterVTable;
}

// bool
#[no_mangle]
pub unsafe extern "C" fn __bool_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_bool;
    data
}
#[no_mangle]
pub unsafe extern "C" fn __bool_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_bool;
    let ptr: *mut bool = Box::<bool>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __bool_clone(data: *mut ()) -> *mut () {
    Box::<bool>::into_raw(Box::new((*(data as *mut bool)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __bool_drop(data: *mut ()) {
    Box::from_raw(data as *mut bool);
}
extern "C" {
    pub static __BOOL_VTABLE: __RegisterVTable;
}

// i32
#[no_mangle]
pub unsafe extern "C" fn __i32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_i32;
    data != 0
}
#[no_mangle]
pub unsafe extern "C" fn __i32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_i32;
    let ptr: *mut i32 = Box::<i32>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __i32_clone(data: *mut ()) -> *mut () {
    Box::<i32>::into_raw(Box::new((*(data as *mut i32)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __i32_drop(data: *mut ()) {
    Box::from_raw(data as *mut i32);
}
extern "C" {
    pub static __I32_VTABLE: __RegisterVTable;
}

// i64
#[no_mangle]
pub unsafe extern "C" fn __i64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_i64;
    data != 0
}
#[no_mangle]
pub unsafe extern "C" fn __i64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_i64;
    let ptr: *mut i64 = Box::<i64>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __i64_clone(data: *mut ()) -> *mut () {
    Box::<i64>::into_raw(Box::new((*(data as *mut i64)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __i64_drop(data: *mut ()) {
    Box::from_raw(data as *mut i64);
}
extern "C" {
    pub static __I64_VTABLE: __RegisterVTable;
}

// b32
#[no_mangle]
pub unsafe extern "C" fn __b32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_b32;
    data != 0
}
#[no_mangle]
pub unsafe extern "C" fn __b32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_b32;
    let ptr: *mut b32 = Box::<b32>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __b32_clone(data: *mut ()) -> *mut () {
    Box::<b32>::into_raw(Box::new((*(data as *mut b32)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __b32_drop(data: *mut ()) {
    Box::from_raw(data as *mut b32);
}
extern "C" {
    pub static __B32_VTABLE: __RegisterVTable;
}

// b64
#[no_mangle]
pub unsafe extern "C" fn __b64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_b64;
    data != 0
}
#[no_mangle]
pub unsafe extern "C" fn __b64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_b64;
    let ptr: *mut b64 = Box::<b64>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __b64_clone(data: *mut ()) -> *mut () {
    Box::<b64>::into_raw(Box::new((*(data as *mut b64)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __b64_drop(data: *mut ()) {
    Box::from_raw(data as *mut b64);
}
extern "C" {
    pub static __B64_VTABLE: __RegisterVTable;
}

// f32
#[no_mangle]
pub unsafe extern "C" fn __f32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_f32;
    data != 0.0
}
#[no_mangle]
pub unsafe extern "C" fn __f32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_f32;
    let ptr: *mut f32 = Box::<f32>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __f32_clone(data: *mut ()) -> *mut () {
    Box::<f32>::into_raw(Box::new((*(data as *mut f32)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __f32_drop(data: *mut ()) {
    Box::from_raw(data as *mut f32);
}
extern "C" {
    pub static __F32_VTABLE: __RegisterVTable;
}

// f64
#[no_mangle]
pub unsafe extern "C" fn __f64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_f64;
    data != 0.0
}
#[no_mangle]
pub unsafe extern "C" fn __f64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_f64;
    let ptr: *mut f64 = Box::<f64>::into_raw(Box::new(data));
    ptr as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __f64_clone(data: *mut ()) -> *mut () {
    Box::<f64>::into_raw(Box::new((*(data as *mut f64)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __f64_drop(data: *mut ()) {
    Box::from_raw(data as *mut f64);
}
extern "C" {
    pub static __F64_VTABLE: __RegisterVTable;
}
