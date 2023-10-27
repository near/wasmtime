;;! target = "x86_64"

(module
  (type (func (result i32)))  ;; type #0
  (import "a" "ef0" (func (result i32)))    ;; index 0
  (import "a" "ef1" (func (result i32)))
  (import "a" "ef2" (func (result i32)))
  (import "a" "ef3" (func (result i32)))
  (import "a" "ef4" (func (result i32)))    ;; index 4
  (table $t0 30 30 funcref)
  (table $t1 30 30 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 5))  ;; index 5
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))  ;; index 9
  (func (export "test")
    (table.init $t0 1 (i32.const 7) (i32.const 0) (i32.const 4))
         (elem.drop 1)
         (table.init $t0 3 (i32.const 15) (i32.const 1) (i32.const 3))
         (elem.drop 3)
         (table.copy $t0 0 (i32.const 20) (i32.const 15) (i32.const 5))
         (table.copy $t0 0 (i32.const 21) (i32.const 29) (i32.const 1))
         (table.copy $t0 0 (i32.const 24) (i32.const 10) (i32.const 1))
         (table.copy $t0 0 (i32.const 13) (i32.const 11) (i32.const 4))
         (table.copy $t0 0 (i32.const 19) (i32.const 20) (i32.const 5)))
  (func (export "check") (param i32) (result i32)
    (call_indirect $t0 (type 0) (local.get 0)))
)
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b805000000           	mov	eax, 5
;;   11:	 4883c408             	add	rsp, 8
;;   15:	 5d                   	pop	rbp
;;   16:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b806000000           	mov	eax, 6
;;   11:	 4883c408             	add	rsp, 8
;;   15:	 5d                   	pop	rbp
;;   16:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b807000000           	mov	eax, 7
;;   11:	 4883c408             	add	rsp, 8
;;   15:	 5d                   	pop	rbp
;;   16:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b808000000           	mov	eax, 8
;;   11:	 4883c408             	add	rsp, 8
;;   15:	 5d                   	pop	rbp
;;   16:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b809000000           	mov	eax, 9
;;   11:	 4883c408             	add	rsp, 8
;;   15:	 5d                   	pop	rbp
;;   16:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   10:	 498b4310             	mov	rax, qword ptr [r11 + 0x10]
;;   14:	 4156                 	push	r14
;;   16:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   1a:	 be00000000           	mov	esi, 0
;;   1f:	 ba01000000           	mov	edx, 1
;;   24:	 b907000000           	mov	ecx, 7
;;   29:	 41b800000000         	mov	r8d, 0
;;   2f:	 41b904000000         	mov	r9d, 4
;;   35:	 ffd0                 	call	rax
;;   37:	 4883c408             	add	rsp, 8
;;   3b:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   3f:	 498b4318             	mov	rax, qword ptr [r11 + 0x18]
;;   43:	 4156                 	push	r14
;;   45:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   49:	 be01000000           	mov	esi, 1
;;   4e:	 ffd0                 	call	rax
;;   50:	 4883c408             	add	rsp, 8
;;   54:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   58:	 498b4310             	mov	rax, qword ptr [r11 + 0x10]
;;   5c:	 4156                 	push	r14
;;   5e:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   62:	 be00000000           	mov	esi, 0
;;   67:	 ba03000000           	mov	edx, 3
;;   6c:	 b90f000000           	mov	ecx, 0xf
;;   71:	 41b801000000         	mov	r8d, 1
;;   77:	 41b903000000         	mov	r9d, 3
;;   7d:	 ffd0                 	call	rax
;;   7f:	 4883c408             	add	rsp, 8
;;   83:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   87:	 498b4318             	mov	rax, qword ptr [r11 + 0x18]
;;   8b:	 4156                 	push	r14
;;   8d:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   91:	 be03000000           	mov	esi, 3
;;   96:	 ffd0                 	call	rax
;;   98:	 4883c408             	add	rsp, 8
;;   9c:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   a0:	 498b4308             	mov	rax, qword ptr [r11 + 8]
;;   a4:	 4156                 	push	r14
;;   a6:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   aa:	 be00000000           	mov	esi, 0
;;   af:	 ba00000000           	mov	edx, 0
;;   b4:	 b914000000           	mov	ecx, 0x14
;;   b9:	 41b80f000000         	mov	r8d, 0xf
;;   bf:	 41b905000000         	mov	r9d, 5
;;   c5:	 ffd0                 	call	rax
;;   c7:	 4883c408             	add	rsp, 8
;;   cb:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   cf:	 498b4308             	mov	rax, qword ptr [r11 + 8]
;;   d3:	 4156                 	push	r14
;;   d5:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;   d9:	 be00000000           	mov	esi, 0
;;   de:	 ba00000000           	mov	edx, 0
;;   e3:	 b915000000           	mov	ecx, 0x15
;;   e8:	 41b81d000000         	mov	r8d, 0x1d
;;   ee:	 41b901000000         	mov	r9d, 1
;;   f4:	 ffd0                 	call	rax
;;   f6:	 4883c408             	add	rsp, 8
;;   fa:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   fe:	 498b4308             	mov	rax, qword ptr [r11 + 8]
;;  102:	 4156                 	push	r14
;;  104:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;  108:	 be00000000           	mov	esi, 0
;;  10d:	 ba00000000           	mov	edx, 0
;;  112:	 b918000000           	mov	ecx, 0x18
;;  117:	 41b80a000000         	mov	r8d, 0xa
;;  11d:	 41b901000000         	mov	r9d, 1
;;  123:	 ffd0                 	call	rax
;;  125:	 4883c408             	add	rsp, 8
;;  129:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;  12d:	 498b4308             	mov	rax, qword ptr [r11 + 8]
;;  131:	 4156                 	push	r14
;;  133:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;  137:	 be00000000           	mov	esi, 0
;;  13c:	 ba00000000           	mov	edx, 0
;;  141:	 b90d000000           	mov	ecx, 0xd
;;  146:	 41b80b000000         	mov	r8d, 0xb
;;  14c:	 41b904000000         	mov	r9d, 4
;;  152:	 ffd0                 	call	rax
;;  154:	 4883c408             	add	rsp, 8
;;  158:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;  15c:	 498b4308             	mov	rax, qword ptr [r11 + 8]
;;  160:	 4156                 	push	r14
;;  162:	 488b3c24             	mov	rdi, qword ptr [rsp]
;;  166:	 be00000000           	mov	esi, 0
;;  16b:	 ba00000000           	mov	edx, 0
;;  170:	 b913000000           	mov	ecx, 0x13
;;  175:	 41b814000000         	mov	r8d, 0x14
;;  17b:	 41b905000000         	mov	r9d, 5
;;  181:	 ffd0                 	call	rax
;;  183:	 4883c408             	add	rsp, 8
;;  187:	 4883c408             	add	rsp, 8
;;  18b:	 5d                   	pop	rbp
;;  18c:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec10             	sub	rsp, 0x10
;;    8:	 897c240c             	mov	dword ptr [rsp + 0xc], edi
;;    c:	 4c89742404           	mov	qword ptr [rsp + 4], r14
;;   11:	 8b4c240c             	mov	ecx, dword ptr [rsp + 0xc]
;;   15:	 4c89f2               	mov	rdx, r14
;;   18:	 8b9af0000000         	mov	ebx, dword ptr [rdx + 0xf0]
;;   1e:	 39d9                 	cmp	ecx, ebx
;;   20:	 0f8376000000         	jae	0x9c
;;   26:	 4189cb               	mov	r11d, ecx
;;   29:	 4d6bdb08             	imul	r11, r11, 8
;;   2d:	 488b92e8000000       	mov	rdx, qword ptr [rdx + 0xe8]
;;   34:	 4889d6               	mov	rsi, rdx
;;   37:	 4c01da               	add	rdx, r11
;;   3a:	 39d9                 	cmp	ecx, ebx
;;   3c:	 480f43d6             	cmovae	rdx, rsi
;;   40:	 488b02               	mov	rax, qword ptr [rdx]
;;   43:	 4885c0               	test	rax, rax
;;   46:	 0f8523000000         	jne	0x6f
;;   4c:	 4d8b5e38             	mov	r11, qword ptr [r14 + 0x38]
;;   50:	 498b5b48             	mov	rbx, qword ptr [r11 + 0x48]
;;   54:	 4156                 	push	r14
;;   56:	 51                   	push	rcx
;;   57:	 488b7c2408           	mov	rdi, qword ptr [rsp + 8]
;;   5c:	 be00000000           	mov	esi, 0
;;   61:	 8b1424               	mov	edx, dword ptr [rsp]
;;   64:	 ffd3                 	call	rbx
;;   66:	 4883c410             	add	rsp, 0x10
;;   6a:	 e904000000           	jmp	0x73
;;   6f:	 4883e0fe             	and	rax, 0xfffffffffffffffe
;;   73:	 4885c0               	test	rax, rax
;;   76:	 0f8422000000         	je	0x9e
;;   7c:	 4d8b5e40             	mov	r11, qword ptr [r14 + 0x40]
;;   80:	 418b0b               	mov	ecx, dword ptr [r11]
;;   83:	 8b5018               	mov	edx, dword ptr [rax + 0x18]
;;   86:	 39d1                 	cmp	ecx, edx
;;   88:	 0f8512000000         	jne	0xa0
;;   8e:	 50                   	push	rax
;;   8f:	 59                   	pop	rcx
;;   90:	 488b5110             	mov	rdx, qword ptr [rcx + 0x10]
;;   94:	 ffd2                 	call	rdx
;;   96:	 4883c410             	add	rsp, 0x10
;;   9a:	 5d                   	pop	rbp
;;   9b:	 c3                   	ret	
;;   9c:	 0f0b                 	ud2	
;;   9e:	 0f0b                 	ud2	
;;   a0:	 0f0b                 	ud2	
