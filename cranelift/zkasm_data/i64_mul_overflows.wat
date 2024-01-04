(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 2
	i64.const 9223372036854775808
    i64.mul
    i64.const 0
	call $assert_eq_i64)
 (start $main))
