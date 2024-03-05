(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x8ff00ff00ff00ff0
	i64.const 0x100000001
	i64.div_u
	i64.const 0x8ff00fef
	call $assert_eq_i64)
 (export "main" (func $main)))
