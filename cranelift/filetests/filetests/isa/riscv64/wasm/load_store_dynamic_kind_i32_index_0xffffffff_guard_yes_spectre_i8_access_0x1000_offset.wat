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
;;! offset_guard_size = 0xffffffff
;;! index_type = "i32"
;;! style = { kind = "dynamic", bound = "heap_bound" }

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i32 1)

  (func (export "do_store") (param i32 i32)
    local.get 0
    local.get 1
    i32.store8 offset=0x1000)

  (func (export "do_load") (param i32) (result i32)
    local.get 0
    i32.load8_u offset=0x1000))

;; function u0:0:
;; block0:
;;   slli a3,a0,32
;;   srli a4,a3,32
;;   ld a3,8(a2)
;;   sltu a3,a3,a4
;;   ld a2,0(a2)
;;   add a2,a2,a4
;;   lui a4,1
;;   add a2,a2,a4
;;   sub a0,zero,a3
;;   not a3,a0
;;   and a4,a2,a3
;;   sb a1,0(a4)
;;   j label1
;; block1:
;;   ret
;;
;; function u0:1:
;; block0:
;;   slli a2,a0,32
;;   srli a3,a2,32
;;   ld a2,8(a1)
;;   sltu a2,a2,a3
;;   ld a4,0(a1)
;;   add a3,a4,a3
;;   lui a4,1
;;   add a3,a3,a4
;;   sub a0,zero,a2
;;   not a2,a0
;;   and a4,a3,a2
;;   lbu a0,0(a4)
;;   j label1
;; block1:
;;   ret
