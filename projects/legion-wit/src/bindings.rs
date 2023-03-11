#[repr(C)]
#[derive(Clone, Copy)]
pub struct EncodeConfig {
    pub generate_dwarf: bool,
}
impl ::core::fmt::Debug for EncodeConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EncodeConfig")
            .field("generate-dwarf", &self.generate_dwarf)
            .finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DecodeConfig {
    pub skeleton_only: bool,
}
impl ::core::fmt::Debug for DecodeConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DecodeConfig")
            .field("skeleton-only", &self.skeleton_only)
            .finish()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_encode_wasm_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: i32,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::encode_wasm(
        _rt::string_lift(bytes0),
        EncodeConfig {
            generate_dwarf: _rt::bool_lift(arg2 as u8),
        },
    );
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let vec3 = (e).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr2.add(8).cast::<usize>() = len3;
            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
        }
        Err(_) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_encode_wasm<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            let base3 = l1;
            let len3 = l2;
            _rt::cabi_dealloc(base3, len3 * 1, 1);
        }
        _ => {}
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_decode_wasm_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: i32,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let result1 = T::decode_wasm(
        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
        DecodeConfig {
            skeleton_only: _rt::bool_lift(arg2 as u8),
        },
    );
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let vec3 = (e.into_bytes()).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr2.add(8).cast::<usize>() = len3;
            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
        }
        Err(_) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_decode_wasm<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
        _ => {}
    }
}
pub trait Guest {
    fn encode_wasm(s: _rt::String, config: EncodeConfig) -> Result<_rt::Vec<u8>, ()>;
    fn decode_wasm(s: _rt::Vec<u8>, config: DecodeConfig) -> Result<_rt::String, ()>;
}
#[doc(hidden)]
macro_rules! __export_world_tools_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "encode-wasm"] unsafe extern "C" fn
        export_encode_wasm(arg0 : * mut u8, arg1 : usize, arg2 : i32,) -> * mut u8 {
        $($path_to_types)*:: _export_encode_wasm_cabi::<$ty > (arg0, arg1, arg2) }
        #[export_name = "cabi_post_encode-wasm"] unsafe extern "C" fn
        _post_return_encode_wasm(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_encode_wasm::<$ty > (arg0) } #[export_name = "decode-wasm"] unsafe
        extern "C" fn export_decode_wasm(arg0 : * mut u8, arg1 : usize, arg2 : i32,) -> *
        mut u8 { $($path_to_types)*:: _export_decode_wasm_cabi::<$ty > (arg0, arg1, arg2)
        } #[export_name = "cabi_post_decode-wasm"] unsafe extern "C" fn
        _post_return_decode_wasm(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_decode_wasm::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_tools_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_tools_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_tools_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_tools_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:v:legion:tools:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 301] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb1\x01\x01A\x02\x01\
A\x0b\x01r\x01\x0egenerate-dwarf\x7f\x03\0\x0dencode-config\x03\0\0\x01r\x01\x0d\
skeleton-only\x7f\x03\0\x0ddecode-config\x03\0\x02\x01p}\x01j\x01\x04\0\x01@\x02\
\x01ss\x06config\x01\0\x05\x04\0\x0bencode-wasm\x01\x06\x01j\x01s\0\x01@\x02\x01\
s\x04\x06config\x03\0\x07\x04\0\x0bdecode-wasm\x01\x08\x04\0\x0ev:legion/tools\x04\
\0\x0b\x0b\x01\0\x05tools\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit\
-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
