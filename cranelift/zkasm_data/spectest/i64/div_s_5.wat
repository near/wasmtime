(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x8000000000000000
	i64.const 2
	i64.div_s
	i64.const 0xc000000000000000
	call $assert_eq_i64)
 (export "main" (func $main)))
