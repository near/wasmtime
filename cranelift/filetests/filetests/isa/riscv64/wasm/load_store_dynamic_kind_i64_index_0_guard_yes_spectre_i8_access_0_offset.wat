;;! target = "riscv64"
;;!
;;! settings = ['enable_heap_access_spectre_mitigation=true']
;;!
;;! compile = true
;;!
;;! [globals.vmctx]
;;! type = "i64"
;;! vmctx = true
;;!
;;! [globals.heap_base]
;;! type = "i64"
;;! load = { base = "vmctx", offset = 0, readonly = true }
;;!
;;! [globals.heap_bound]
;;! type = "i64"
;;! load = { base = "vmctx", offset = 8, readonly = true }
;;!
;;! [[heaps]]
;;! base = "heap_base"
;;! min_size = 0x10000
;;! offset_guard_size = 0
;;! index_type = "i64"
;;! style = { kind = "dynamic", bound = "heap_bound" }

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i64 1)

  (func (export "do_store") (param i64 i32)
    local.get 0
    local.get 1
    i32.store8 offset=0)

  (func (export "do_load") (param i64) (result i32)
    local.get 0
    i32.load8_u offset=0))

;; function u0:0:
;; block0:
;;   ld a3,8(a2)
;;   uge a4,a0,a3##ty=i64
;;   ld a3,0(a2)
;;   add a3,a3,a0
;;   li a5,0
;;   andi a4,a4,255
;;   sltu a0,zero,a4
;;   sub a2,zero,a0
;;   and a4,a5,a2
;;   not a5,a2
;;   and a2,a3,a5
;;   or a3,a4,a2
;;   sb a1,0(a3)
;;   j label1
;; block1:
;;   ret
;;
;; function u0:1:
;; block0:
;;   ld a3,8(a1)
;;   uge a4,a0,a3##ty=i64
;;   ld a3,0(a1)
;;   add a3,a3,a0
;;   li a5,0
;;   andi a4,a4,255
;;   sltu a0,zero,a4
;;   sub a1,zero,a0
;;   and a4,a5,a1
;;   not a5,a1
;;   and a1,a3,a5
;;   or a3,a4,a1
;;   lbu a0,0(a3)
;;   j label1
;; block1:
;;   ret
