// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod golem {
        pub mod component {
            /// See https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md for more details about the WIT syntax
            #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_cabi<T: Guest>(arg0: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add(arg0 as u64);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_cabi<T: Guest>() -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get();
                    _rt::as_i64(result0)
                }
                pub trait Guest {
                    fn add(value: u64) -> ();
                    fn get() -> u64;
                }
                #[doc(hidden)]
                macro_rules! __export_golem_component_api_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[unsafe (export_name =
                        "golem:component/api#add")] unsafe extern "C" fn export_add(arg0
                        : i64,) { unsafe { $($path_to_types)*:: _export_add_cabi::<$ty >
                        (arg0) } } #[unsafe (export_name = "golem:component/api#get")]
                        unsafe extern "C" fn export_get() -> i64 { unsafe {
                        $($path_to_types)*:: _export_get_cabi::<$ty > () } } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_golem_component_api_cabi;
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #![allow(dead_code, clippy::all)]
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
}
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
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
macro_rules! __export_failing_component_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::golem::component::api::__export_golem_component_api_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::golem::component::api);
    };
}
#[doc(inline)]
pub(crate) use __export_failing_component_impl as export;
#[cfg(target_arch = "wasm32")]
#[unsafe(
    link_section = "component-type:wit-bindgen:0.41.0:golem:component:failing-component:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 239] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07h\x01A\x02\x01A\x02\x01\
B\x04\x01@\x01\x05valuew\x01\0\x04\0\x03add\x01\0\x01@\0\0w\x04\0\x03get\x01\x01\
\x04\0\x13golem:component/api\x05\0\x04\0!golem:component/failing-component\x04\0\
\x0b\x17\x01\0\x11failing-component\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.227.1\x10wit-bindgen-rust\x060.41.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
