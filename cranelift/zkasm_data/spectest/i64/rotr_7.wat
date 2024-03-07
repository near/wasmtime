(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xabcd1234ef567809
	i64.const 53
	i64.rotr
	i64.const 0x6891a77ab3c04d5e
	call $assert_eq_i64)
 (export "main" (func $main)))
