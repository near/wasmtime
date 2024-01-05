(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xfedcba98_765432_80
	i64.extend8_s
	i64.const -0x80
	call $assert_eq_i64)
 (start $main))
