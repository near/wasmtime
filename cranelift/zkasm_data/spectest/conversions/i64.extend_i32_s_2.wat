(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i32.const 10000
	i64.extend_i32_s
	i64.const 10000
	call $assert_eq_i64)
 (start $main))
