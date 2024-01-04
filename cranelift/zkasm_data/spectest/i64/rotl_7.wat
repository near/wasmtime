(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xabcd1234ef567809
	i64.const 53
	i64.rotl
	i64.const 0x013579a2469deacf
	call $assert_eq_i64)
 (start $main))
