extern crate test_support;

#[test]
fn export() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            macro_rules! doit {
                ($($i:ident)*) => ($(
                    #[no_mangle]
                    #[wasm_bindgen]
                    pub extern fn $i(a: &[$i]) -> Vec<$i> {
                        assert_eq!(a.len(), 2);
                        assert_eq!(a[0], 1 as $i);
                        assert_eq!(a[1], 2 as $i);
                        a.to_vec()
                    }
                )*)
            }


            doit! { i8 u8 i16 u16 i32 u32 f32 f64 }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            function assert_arrays_equal(a: any, b: any) {
                console.log(a, b);
                assert.strictEqual(a.length, b.length);
                assert.strictEqual(a.byteLength, b.byteLength);
                for (let i = 0; i < a.length; i++) {
                    assert.strictEqual(a[i], b[i]);
                }
            }

            export function test() {
                const i8 = new Int8Array(2);
                i8[0] = 1;
                i8[1] = 2;
                assert_arrays_equal(wasm.i8(i8), i8);
                const u8 = new Uint8Array(2);
                u8[0] = 1;
                u8[1] = 2;
                assert_arrays_equal(wasm.u8(u8), u8);

                const i16 = new Int16Array(2);
                i16[0] = 1;
                i16[1] = 2;
                assert_arrays_equal(wasm.i16(i16), i16);
                const u16 = new Uint16Array(2);
                u16[0] = 1;
                u16[1] = 2;
                assert_arrays_equal(wasm.u16(u16), u16);

                const i32 = new Int32Array(2);
                i32[0] = 1;
                i32[1] = 2;
                wasm.i32(i32);
                assert_arrays_equal(wasm.i32(i32), i32);
                const u32 = new Uint32Array(2);
                u32[0] = 1;
                u32[1] = 2;
                assert_arrays_equal(wasm.u32(u32), u32);

                const f32 = new Float32Array(2);
                f32[0] = 1;
                f32[1] = 2;
                assert_arrays_equal(wasm.f32(f32), f32);
                const f64 = new Float64Array(2);
                f64[0] = 1;
                f64[1] = 2;
                assert_arrays_equal(wasm.f64(f64), f64);
          }
      "#)
        .test();
}

#[test]
fn import() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            macro_rules! doit {
                ($(($rust:ident, $js:ident, $i:ident))*) => ($(
                    #[wasm_bindgen(module = "./test")]
                    extern {
                        fn $js(a: &[$i]) -> Vec<$i>;
                    }

                    #[no_mangle]
                    #[wasm_bindgen]
                    pub extern fn $rust(a: &[$i]) -> Vec<$i> {
                        assert_eq!(a.len(), 2);
                        assert_eq!(a[0], 1 as $i);
                        assert_eq!(a[1], 2 as $i);
                        $js(a)
                    }
                )*)
            }


            doit! {
                (rust_i8, js_i8, i8)
                (rust_u8, js_u8, u8)
                (rust_i16, js_i16, i16)
                (rust_u16, js_u16, u16)
                (rust_i32, js_i32, i32)
                (rust_u32, js_u32, u32)
                (rust_f32, js_f32, f32)
                (rust_f64, js_f64, f64)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function js_i8(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_u8(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_i16(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_u16(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_i32(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_u32(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_f32(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            export function js_f64(a: any): any {
                assert.strictEqual(a.length, 2);
                assert.strictEqual(a[0], 1);
                assert.strictEqual(a[1], 2);
                return a;
            }

            function assert_arrays_equal(a: any, b: any) {
                console.log(a, b);
                assert.strictEqual(a.length, b.length);
                assert.strictEqual(a.byteLength, b.byteLength);
                for (let i = 0; i < a.length; i++) {
                    assert.strictEqual(a[i], b[i]);
                }
            }

            export function test() {
                const i8 = new Int8Array(2);
                i8[0] = 1;
                i8[1] = 2;
                assert_arrays_equal(wasm.rust_i8(i8), i8);
                const u8 = new Uint8Array(2);
                u8[0] = 1;
                u8[1] = 2;
                assert_arrays_equal(wasm.rust_u8(u8), u8);

                const i16 = new Int16Array(2);
                i16[0] = 1;
                i16[1] = 2;
                assert_arrays_equal(wasm.rust_i16(i16), i16);
                const u16 = new Uint16Array(2);
                u16[0] = 1;
                u16[1] = 2;
                assert_arrays_equal(wasm.rust_u16(u16), u16);

                const i32 = new Int32Array(2);
                i32[0] = 1;
                i32[1] = 2;
                assert_arrays_equal(wasm.rust_i32(i32), i32);
                const u32 = new Uint32Array(2);
                u32[0] = 1;
                u32[1] = 2;
                assert_arrays_equal(wasm.rust_u32(u32), u32);

                const f32 = new Float32Array(2);
                f32[0] = 1;
                f32[1] = 2;
                assert_arrays_equal(wasm.rust_f32(f32), f32);
                const f64 = new Float64Array(2);
                f64[0] = 1;
                f64[1] = 2;
                assert_arrays_equal(wasm.rust_f64(f64), f64);
            }
        "#)
        .test();
}
