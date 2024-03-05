(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x7fffffffffffffff
	i64.const -1
	i64.mul
	i64.const 0x8000000000000001
	call $assert_eq_i64)
 (export "main" (func $main)))
