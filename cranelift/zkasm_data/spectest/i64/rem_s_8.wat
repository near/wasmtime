(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x8000000000000001
	i64.const 1000
	i64.rem_s
	i64.const -807
	call $assert_eq_i64)
 (start $main))
