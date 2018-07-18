use super::websys_project;

#[test]
fn console() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, use_external_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_console() {
                    //web_sys::console::assert();
                    web_sys::console::clear();
                    //web_sys::console::count();
                    web_sys::console::count("test");
                    //web_sys::console::count_reset();
                    web_sys::console::count_reset("test");
                    //web_sys::console::debug();
                    //web_sys::console::error();
                    //web_sys::console::info();
                    //web_sys::console::log();
                    //web_sys::console::table();
                    //web_sys::console::trace();
                    //web_sys::console::warn();
                    //web_sys::console::dir();
                    //web_sys::console::dirxml();
                    //web_sys::console::group();
                    //web_sys::console::group_collapsed();
                    web_sys::console::group_end();
                    //web_sys::console::time();
                    web_sys::console::time("test");
                    //web_sys::console::time_log();
                    //web_sys::console::time_end();
                    web_sys::console::time_end("test");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export async function test() {
                  wasm.test_console();
                }
            "#,
        )
        .test();
}
