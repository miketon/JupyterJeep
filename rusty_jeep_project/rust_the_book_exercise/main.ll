; ModuleID = 'main.b8591b61-cgu.0'
source_filename = "main.b8591b61-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hb7ff12a9f4fe140dE", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc66c25e2e25345fcE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE" }>, align 8
@alloc_09d11aa498739cbf0519d318f9792c9b = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_71b99a1804d93c013f301ec59bc480be = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_09d11aa498739cbf0519d318f9792c9b, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc_2bfeba76c1438a46208a034153050220 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc_e2222df79fea71f2539d68a3a6ae8b04 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs" }>, align 1
@alloc_ffaf8578af742fcd9190283b2acd56d3 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e2222df79fea71f2539d68a3a6ae8b04, [16 x i8] c"K\00\00\00\00\00\00\00\93\01\00\00\0D\00\00\00" }>, align 8
@alloc_16a429f18bc382a7367b7be880ae543c = private unnamed_addr constant <{ [19 x i8] }> <{ [19 x i8] c"[main.rs] on awake\0A" }>, align 1
@alloc_c4c9d68f37f104cd72fe358d462cc340 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_16a429f18bc382a7367b7be880ae543c, [8 x i8] c"\13\00\00\00\00\00\00\00" }>, align 8
@alloc_6edc2ae99c553c520f8c3f81b6ddd018 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"[on_main_print_greeting] Heyao World!!!\0A" }>, align 1
@alloc_26b04c951a29748b9fe1702bdeca0e3d = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_6edc2ae99c553c520f8c3f81b6ddd018, [8 x i8] c"(\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcc799a4561a0c52cE(ptr %f) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hf7a560ef3937f06fE(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !2
  br label %bb4

bb4:                                              ; preds = %start
  ret void

bb2:                                              ; No predecessors!
  %1 = load ptr, ptr %0, align 8, !noundef !3
  %2 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %3 = load i32, ptr %2, align 8, !noundef !3
  %4 = insertvalue { ptr, i32 } undef, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17hb914d2c8ed180aa5E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
start:
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h91996717d3eb1d2aE(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8, !noundef !3
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE"(ptr align 8 %_1) unnamed_addr #2 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hcc799a4561a0c52cE(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba0918006791e144E"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !3
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #2 {
start:
  %_15 = alloca { ptr, i64 }, align 8
  %_12 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_9 = add i64 %args.1, 1
  %_7 = icmp ugt i64 %pieces.1, %_9
  %1 = zext i1 %_7 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !4, !noundef !3
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  store ptr null, ptr %_15, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !5, !noundef !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_15, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E(ptr sret(%"core::fmt::Arguments<'_>") %_12, ptr align 8 @alloc_71b99a1804d93c013f301ec59bc480be, i64 1, ptr align 8 @alloc_2bfeba76c1438a46208a034153050220, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h7e47e10600a90221E(ptr %_12, ptr align 8 @alloc_ffaf8578af742fcd9190283b2acd56d3) #5
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hc66c25e2e25345fcE"(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h263851872fdfab25E(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h263851872fdfab25E(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef1c1ac2fa91f7baE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8, !noundef !3
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %5 = load i32, ptr %4, align 8, !noundef !3
  %6 = insertvalue { ptr, i32 } undef, ptr %3, 0
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1
  resume { ptr, i32 } %7

cleanup:                                          ; preds = %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %2
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hf7a560ef3937f06fE(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hb7ff12a9f4fe140dE"(ptr %_1) unnamed_addr #2 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hba0918006791e144E"() unnamed_addr #2 {
start:
  ret i8 0
}

; main::main
; Function Attrs: uwtable
define internal void @_ZN4main4main17hd0d97daef3a32cf2E() unnamed_addr #1 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E(ptr sret(%"core::fmt::Arguments<'_>") %_2, ptr align 8 @alloc_c4c9d68f37f104cd72fe358d462cc340, i64 1, ptr align 8 @alloc_2bfeba76c1438a46208a034153050220, i64 0)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hdf66a86f8ccb3187E(ptr %_2)
; call main::on_main_print_greeting
  call void @_ZN4main22on_main_print_greeting17h8d627335dcd70cdeE()
  ret void
}

; main::on_main_print_greeting
; Function Attrs: uwtable
define internal void @_ZN4main22on_main_print_greeting17h8d627335dcd70cdeE() unnamed_addr #1 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hfb455dc6e2532627E(ptr sret(%"core::fmt::Arguments<'_>") %_2, ptr align 8 @alloc_26b04c951a29748b9fe1702bdeca0e3d, i64 1, ptr align 8 @alloc_2bfeba76c1438a46208a034153050220, i64 0)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hdf66a86f8ccb3187E(ptr %_2)
  ret void
}

; Function Attrs: uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h91996717d3eb1d2aE(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17h7e47e10600a90221E(ptr, ptr align 8) unnamed_addr #3

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17hdf66a86f8ccb3187E(ptr) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #4 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hb914d2c8ed180aa5E(ptr @_ZN4main4main17hd0d97daef3a32cf2E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #4 = { "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 1594527}
!3 = !{}
!4 = !{i8 0, i8 2}
!5 = !{i64 8}
