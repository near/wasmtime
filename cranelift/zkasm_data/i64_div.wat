(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 214748364
	i64.const 107374183
    i64.const 2
	i64.mul
    i64.mul
    i64.const 214748364
    i64.div_s
    i64.const 214748366
	call $assert_eq_i64)
 (export "main" (func $main)))
