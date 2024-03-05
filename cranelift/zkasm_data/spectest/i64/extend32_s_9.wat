(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xfedcba98_80000000
	i64.extend32_s
	i64.const -0x80000000
	call $assert_eq_i64)
 (export "main" (func $main)))
