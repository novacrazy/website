use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[rustfmt::skip]
#[macro_export]
macro_rules! log {
    ($conststr:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::js::log(($conststr).as_ref()); }
    }};

    ($($t:tt)+) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::js::log(format!($($t)+).as_ref()); }
    }};
}

#[rustfmt::skip]
#[macro_export]
macro_rules! error {
    ($conststr:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::js::error(($conststr).as_ref()); }
    }};

    ($($t:tt)+) => {{
        #[allow(unused_unsafe)]
        unsafe { $crate::js::error(format!($($t)+).as_ref()); }
    }};
}
