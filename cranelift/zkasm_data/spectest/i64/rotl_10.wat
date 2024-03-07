(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xabcd7294ef567809
	i64.const 0xffffffffffffffed
	i64.rotl
	i64.const 0xcf013579ae529dea
	call $assert_eq_i64)
 (export "main" (func $main)))
