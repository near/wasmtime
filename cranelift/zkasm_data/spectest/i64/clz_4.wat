(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xff
	i64.clz
	i64.const 56
	call $assert_eq_i64)
 (start $main))
