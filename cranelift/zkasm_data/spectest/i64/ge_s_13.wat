(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0x8000000000000000
	i64.const 0x7fffffffffffffff
	i64.ge_s
	i32.const 0
	call $assert_eq_i32)
 (start $main))