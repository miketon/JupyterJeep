	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcc799a4561a0c52cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17hf7a560ef3937f06fE
	; InlineAsm Start
	; InlineAsm End
	b	LBB0_1
LBB0_1:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17hb914d2c8ed180aa5E
	.globl	__ZN3std2rt10lang_start17hb914d2c8ed180aa5E
	.p2align	2
__ZN3std2rt10lang_start17hb914d2c8ed180aa5E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp]
	mov	x0, x2
	ldr	x2, [sp]
	str	x0, [sp, #8]
	mov	x4, x3
	ldr	x3, [sp, #8]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h91996717d3eb1d2aE
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcc799a4561a0c52cE
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba0918006791e144E
	sturb	w0, [x29, #-1]
	ldurb	w0, [x29, #-1]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E:
	.cfi_startproc
	sub	sp, sp, #144
	.cfi_def_cfa_offset 144
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #24]
	str	x1, [sp, #32]
	str	x2, [sp, #40]
	str	x3, [sp, #48]
	subs	x8, x1, x3
	cset	w8, lo
	tbnz	w8, #0, LBB3_2
	b	LBB3_1
LBB3_1:
	ldr	x8, [sp, #32]
	ldr	x9, [sp, #48]
	add	x9, x9, #1
	subs	x8, x8, x9
	cset	w8, hi
	and	w8, w8, #0x1
	strb	w8, [sp, #63]
	b	LBB3_3
LBB3_2:
	mov	w8, #1
	strb	w8, [sp, #63]
	b	LBB3_3
LBB3_3:
	ldrb	w8, [sp, #63]
	tbnz	w8, #0, LBB3_5
	b	LBB3_4
LBB3_4:
	ldr	x8, [sp, #48]
	ldr	x9, [sp, #16]
	ldr	x10, [sp, #40]
	ldr	x11, [sp, #32]
	ldr	x12, [sp, #24]
	stur	xzr, [x29, #-16]
	str	x12, [x9, #16]
	str	x11, [x9, #24]
	ldur	x12, [x29, #-16]
	ldur	x11, [x29, #-8]
	str	x12, [x9]
	str	x11, [x9, #8]
	str	x10, [x9, #32]
	str	x8, [x9, #40]
	.cfi_def_cfa wsp, 144
	ldp	x29, x30, [sp, #128]
	add	sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB3_5:
	.cfi_restore_state
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_2@PAGE
	add	x0, x0, l___unnamed_2@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	adrp	x2, l___unnamed_3@PAGE
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_4@PAGE
	add	x1, x1, l___unnamed_4@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h7e47e10600a90221E
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc66c25e2e25345fcE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17h263851872fdfab25E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h263851872fdfab25E:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp0:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE
	str	w0, [sp, #12]
Ltmp1:
	b	LBB5_3
LBB5_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB5_2:
Ltmp2:
	mov	x8, x1
	stur	x0, [x29, #-16]
	stur	w8, [x29, #-8]
	b	LBB5_1
LBB5_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table5:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17hf7a560ef3937f06fE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hb7ff12a9f4fe140dE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba0918006791e144E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main4main17hd0d97daef3a32cf2E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x8, sp, #16
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_5@PAGE
	add	x0, x0, l___unnamed_5@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	adrp	x2, l___unnamed_3@PAGE
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E
	ldr	x0, [sp, #8]
	bl	__ZN3std2io5stdio6_print17hdf66a86f8ccb3187E
	bl	__ZN4main22on_main_print_greeting17h8d627335dcd70cdeE
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main22on_main_print_greeting17h8d627335dcd70cdeE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x8, sp, #16
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_6@PAGE
	add	x0, x0, l___unnamed_6@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	adrp	x2, l___unnamed_3@PAGE
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E
	ldr	x0, [sp, #8]
	bl	__ZN3std2io5stdio6_print17hdf66a86f8ccb3187E
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17hd0d97daef3a32cf2E@PAGE
	add	x0, x0, __ZN4main4main17hd0d97daef3a32cf2E@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17hb914d2c8ed180aa5E
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hb7ff12a9f4fe140dE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc66c25e2e25345fcE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE

	.section	__TEXT,__const
l___unnamed_7:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	l___unnamed_7
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3
l___unnamed_3:
	.byte	0

l___unnamed_8:
	.ascii	"/rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_4:
	.quad	l___unnamed_8
	.asciz	"K\000\000\000\000\000\000\000\223\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_9:
	.ascii	"[main.rs] on awake\n"

	.section	__DATA,__const
	.p2align	3
l___unnamed_5:
	.quad	l___unnamed_9
	.asciz	"\023\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.ascii	"[on_main_print_greeting] Heyao World!!!\n"

	.section	__DATA,__const
	.p2align	3
l___unnamed_6:
	.quad	l___unnamed_10
	.asciz	"(\000\000\000\000\000\000"

.subsections_via_symbols
