(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x0123456789abcdef
	i64.const 0xfedcba9876543210
	i64.mul
	i64.const 0x2236d88fe5618cf0
	call $assert_eq_i64)
 (export "main" (func $main)))
