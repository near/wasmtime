(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xabcd987602468ace
	i64.const 1
	i64.rotr
	i64.const 0x55e6cc3b01234567
	call $assert_eq_i64)
 (start $main))