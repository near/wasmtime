(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0xffffffffffffffff
	i64.eqz
	i32.const 0
	call $assert_eq_i32)
 (start $main))