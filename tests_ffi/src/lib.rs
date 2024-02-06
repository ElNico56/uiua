#![allow(clippy::missing_safety_doc)]

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> i32 {
    let mut len = 0;
    while unsafe { *s.offset(len) } != 0 {
        len += 1;
    }
    len as i32
}

#[no_mangle]
pub unsafe extern "C" fn reverse(list: *mut i32, len: i32) {
    std::slice::from_raw_parts_mut(list, len as usize).reverse();
}

#[no_mangle]
pub unsafe extern "C" fn reversed(list: *const i32, len: i32) -> *const i32 {
    let slice = std::slice::from_raw_parts(list, len as usize);
    let mut reversed = slice.to_vec();
    reversed.reverse();
    reversed.leak().as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn head_tail(list: *mut i32, len: *mut i32) -> i32 {
    // println!("list ptr b: {:p}", list);
    // println!("len ptr b: {:p}", len);
    // println!("len: {}", *len);
    // println!("first: {}", *list);
    // println!("second: {}", *list.offset(1));
    // println!("third: {}", *list.offset(2));
    let slice = std::slice::from_raw_parts_mut(list, *len as usize);
    let head = slice[0];
    slice.rotate_left(1);
    *len -= 1;
    head
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub unsafe extern "C" fn vec2_len(v: Vec2) -> f64 {
    // println!("v: {:?}", v);
    // println!("v.x bytes: {:?}", v.x.to_ne_bytes());
    // println!("v.y bytes: {:?}", v.y.to_ne_bytes());
    (v.x * v.x + v.y * v.y).sqrt()
}

#[no_mangle]
pub unsafe extern "C" fn vec2_len_ref(v: *const Vec2) -> f64 {
    let v = &*v;
    (v.x * v.x + v.y * v.y).sqrt()
}

#[no_mangle]
pub unsafe extern "C" fn vec2_add(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

#[no_mangle]
pub unsafe extern "C" fn vec2_normalize(v: *mut Vec2) {
    let v = &mut *v;
    let len = (v.x * v.x + v.y * v.y).sqrt();
    if len == 0.0 {
        v.x = 0.0;
        v.y = 0.0;
    } else {
        v.x /= len;
        v.y /= len;
    }
}

#[repr(C)]
pub struct Person {
    pub name: *const u8,
    pub age: i32,
}

#[no_mangle]
pub unsafe extern "C" fn person_new(name: *const u8, age: i32) -> *mut Person {
    let person = Box::new(Person { name, age });
    Box::into_raw(person)
}

#[test]
fn ffi_test() {
    use std::{path::Path, process::Command};

    use uiua::*;

    Command::new("cargo")
        .args(["build", "-p", "tests_ffi", "--lib"])
        .status()
        .unwrap();

    #[cfg(windows)]
    let dll_path = "../target/debug/ffi_lib.dll";
    #[cfg(unix)]
    let dll_path = "../target/debug/libffi_lib.so";
    let lib_path = Path::new(dll_path);

    let mut uiua = Uiua::with_native_sys().with_args(vec![lib_path.to_string_lossy().into_owned()]);
    uiua.run_file("test.ua").unwrap_or_else(|e| panic!("{e}"));
}
