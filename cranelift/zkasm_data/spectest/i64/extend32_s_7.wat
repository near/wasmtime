(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xffffffff
	i64.extend32_s
	i64.const -1
	call $assert_eq_i64)
 (start $main))