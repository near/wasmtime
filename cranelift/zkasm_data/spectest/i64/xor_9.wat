(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xf0f0ffff
	i64.const 0xfffff0f0
	i64.xor
	i64.const 0x0f0f0f0f
	call $assert_eq_i64)
 (export "main" (func $main)))
