(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xabcd987602468ace
	i64.const 1
	i64.rotl
	i64.const 0x579b30ec048d159d
	call $assert_eq_i64)
 (export "main" (func $main)))
