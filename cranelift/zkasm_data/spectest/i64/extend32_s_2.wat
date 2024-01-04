(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x7fff
	i64.extend32_s
	i64.const 32767
	call $assert_eq_i64)
 (start $main))
