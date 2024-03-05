(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x00008000
	i64.clz
	i64.const 48
	call $assert_eq_i64)
 (export "main" (func $main)))
