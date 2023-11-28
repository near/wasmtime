(module
 (import "env" "assert_eq" (func $assert_eq (param i64) (param i64)))
 (func $main
	(local $counter i32)
	(local $fp i64)
	(local $f i64)
	(local.set $counter (i32.const 0))
	(local.set $fp (i64.const 0))
	(local.set $f (i64.const 1))
	(block
	 (loop
		(local.get $fp)
		(local.get $f)
		(local.set $fp (local.get $f))
		(local.set $f (i64.add))
		(local.set $counter
		 (i32.add
			(local.get $counter)
			(i32.const 1)))
		(br_if 1
		 (i32.eq
			(local.get $counter)
			(i32.const 10000)
		 )
		)
		(br 0)
	 )
	)
	(local.get $fp)
	(i64.const -2872092127636481573)
	call $assert_eq)
(start $main))
