	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 3
	.globl	_wsel_reset                     ; -- Begin function wsel_reset
	.p2align	2
_wsel_reset:                            ; @wsel_reset
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_enqueue                   ; -- Begin function wsel_enqueue
	.p2align	2
_wsel_enqueue:                          ; @wsel_enqueue
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.2@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.2@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_commit                    ; -- Begin function wsel_commit
	.p2align	2
_wsel_commit:                           ; @wsel_commit
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.4@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.4@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_waitUntilScheduled        ; -- Begin function wsel_waitUntilScheduled
	.p2align	2
_wsel_waitUntilScheduled:               ; @wsel_waitUntilScheduled
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.6@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.6@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_wsel_waitUntilCompleted        ; -- Begin function wsel_waitUntilCompleted
	.p2align	2
_wsel_waitUntilCompleted:               ; @wsel_waitUntilCompleted
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	adrp	x8, _OBJC_SELECTOR_REFERENCES_.8@PAGE
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_.8@PAGEOFF]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_:                  ; @OBJC_METH_VAR_NAME_
	.asciz	"reset"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_
_OBJC_SELECTOR_REFERENCES_:
	.quad	l_OBJC_METH_VAR_NAME_

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.1:                ; @OBJC_METH_VAR_NAME_.1
	.asciz	"enqueue"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.2
_OBJC_SELECTOR_REFERENCES_.2:
	.quad	l_OBJC_METH_VAR_NAME_.1

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.3:                ; @OBJC_METH_VAR_NAME_.3
	.asciz	"commit"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.4
_OBJC_SELECTOR_REFERENCES_.4:
	.quad	l_OBJC_METH_VAR_NAME_.3

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.5:                ; @OBJC_METH_VAR_NAME_.5
	.asciz	"waitUntilScheduled"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.6
_OBJC_SELECTOR_REFERENCES_.6:
	.quad	l_OBJC_METH_VAR_NAME_.5

	.section	__TEXT,__objc_methname,cstring_literals
l_OBJC_METH_VAR_NAME_.7:                ; @OBJC_METH_VAR_NAME_.7
	.asciz	"waitUntilCompleted"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3                               ; @OBJC_SELECTOR_REFERENCES_.8
_OBJC_SELECTOR_REFERENCES_.8:
	.quad	l_OBJC_METH_VAR_NAME_.7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
	.long	0
	.long	64

.subsections_via_symbols
