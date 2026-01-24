; ModuleID = 'C:\Users\charisse\AppData\Local\Temp\.tmphi1TSg\Users\charisse\Documents\GitHub\stickle\source\clampandsaw.st.ll'
source_filename = "source\\clampandsaw.st"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%__vtable_TON = type { ptr }
%TON = type { ptr, i8, i64, i8, i64, i8, i8, [24 x i8] }
%__vtable_FB_ClampAndSaw = type { ptr }
%FB_ClampAndSaw = type { ptr, %TON }
%TestStruct = type { [81 x i8] }
%PRG_ClampAndSaw = type { %FB_ClampAndSaw }

@currentState = global i32 0
@State.AllStopped = unnamed_addr constant i32 0
@THNTD = global i8 0
@reset = global i8 0
@testString = global [257 x i8] zeroinitializer
@State.ClampsExtending = unnamed_addr constant i32 1
@State.ClampsExtended = unnamed_addr constant i32 2
@State.SawExtending = unnamed_addr constant i32 3
@State.SawExtended = unnamed_addr constant i32 4
@State.SawRetracting = unnamed_addr constant i32 5
@State.SawRetracted = unnamed_addr constant i32 6
@State.ClampsRetracting = unnamed_addr constant i32 7
@State.ClampsRetracted = unnamed_addr constant i32 8
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___clampandsaw_st, ptr null }]
@__vtable_TON_instance = global %__vtable_TON zeroinitializer
@____vtable_TON__init = unnamed_addr constant %__vtable_TON zeroinitializer
@__TON__init = external unnamed_addr constant %TON
@____vtable_FB_ClampAndSaw__init = unnamed_addr constant %__vtable_FB_ClampAndSaw zeroinitializer
@__FB_ClampAndSaw__init = unnamed_addr constant %FB_ClampAndSaw zeroinitializer
@__TestStruct__init = unnamed_addr constant %TestStruct zeroinitializer
@PRG_ClampAndSaw_instance = global %PRG_ClampAndSaw zeroinitializer
@__vtable_FB_ClampAndSaw_instance = global %__vtable_FB_ClampAndSaw zeroinitializer

declare void @TON(ptr)

define void @FB_ClampAndSaw(ptr %0) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %__vtable = getelementptr inbounds nuw %FB_ClampAndSaw, ptr %0, i32 0, i32 0
  %threeseconds = getelementptr inbounds nuw %FB_ClampAndSaw, ptr %0, i32 0, i32 1
  %load_reset = load i8, ptr @reset, align 1
  %1 = icmp ne i8 %load_reset, 0
  br i1 %1, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  store i8 0, ptr @reset, align 1
  store i32 0, ptr @currentState, align 4
  ret void

buffer_block:                                     ; No predecessors!
  br label %continue

continue:                                         ; preds = %buffer_block, %entry
  %load_currentState = load i32, ptr @currentState, align 4
  switch i32 %load_currentState, label %else [
    i32 0, label %case
    i32 1, label %case4
    i32 2, label %case5
    i32 3, label %case6
    i32 4, label %case7
    i32 5, label %case10
    i32 6, label %case11
    i32 7, label %case12
    i32 8, label %case13
  ]

case:                                             ; preds = %continue
  %load_THNTD = load i8, ptr @THNTD, align 1
  %2 = icmp ne i8 %load_THNTD, 0
  br i1 %2, label %condition_body3, label %continue2

case4:                                            ; preds = %continue
  store i32 2, ptr @currentState, align 4
  br label %continue1

case5:                                            ; preds = %continue
  store i32 3, ptr @currentState, align 4
  br label %continue1

case6:                                            ; preds = %continue
  store i32 4, ptr @currentState, align 4
  br label %continue1

case7:                                            ; preds = %continue
  %3 = getelementptr inbounds %TON, ptr %threeseconds, i32 0, i32 1
  store i8 1, ptr %3, align 1
  %4 = getelementptr inbounds %TON, ptr %threeseconds, i32 0, i32 2
  store i64 3000000000, ptr %4, align 8
  call void @TON(ptr %threeseconds)
  %Q = getelementptr inbounds nuw %TON, ptr %threeseconds, i32 0, i32 3
  %load_Q = load i8, ptr %Q, align 1
  %5 = icmp ne i8 %load_Q, 0
  br i1 %5, label %condition_body9, label %continue8

case10:                                           ; preds = %continue
  store i32 6, ptr @currentState, align 4
  br label %continue1

case11:                                           ; preds = %continue
  store i32 7, ptr @currentState, align 4
  br label %continue1

case12:                                           ; preds = %continue
  store i32 8, ptr @currentState, align 4
  br label %continue1

case13:                                           ; preds = %continue
  store i32 0, ptr @currentState, align 4
  br label %continue1

else:                                             ; preds = %continue
  br label %continue1

continue1:                                        ; preds = %continue8, %continue2, %else, %case13, %case12, %case11, %case10, %case6, %case5, %case4
  ret void

condition_body3:                                  ; preds = %case
  store i32 1, ptr @currentState, align 4
  br label %continue2

continue2:                                        ; preds = %condition_body3, %case
  br label %continue1

condition_body9:                                  ; preds = %case7
  %6 = getelementptr inbounds %TON, ptr %threeseconds, i32 0, i32 1
  store i8 0, ptr %6, align 1
  %7 = getelementptr inbounds %TON, ptr %threeseconds, i32 0, i32 2
  store i64 3000000000, ptr %7, align 8
  call void @TON(ptr %threeseconds)
  store i32 5, ptr @currentState, align 4
  br label %continue8

continue8:                                        ; preds = %condition_body9, %case7
  br label %continue1
}

declare void @puts(ptr)

declare i32 @REAL_TO_DINT(float)

declare void @DINT_TO_STRING(ptr, i32)

define void @PRG_ClampAndSaw(ptr %0) {
entry:
  %stateMachine = getelementptr inbounds nuw %PRG_ClampAndSaw, ptr %0, i32 0, i32 0
  call void @FB_ClampAndSaw(ptr %stateMachine)
  ret void
}

define i32 @GetState() {
entry:
  %GetState = alloca i32, align 4
  store i32 0, ptr %GetState, align 4
  %load_currentState = load i32, ptr @currentState, align 4
  store i32 %load_currentState, ptr %GetState, align 4
  %GetState_ret = load i32, ptr %GetState, align 4
  ret i32 %GetState_ret
}

define void @SetThntd(i8 %0) {
entry:
  %input = alloca i8, align 1
  store i8 %0, ptr %input, align 1
  %load_input = load i8, ptr %input, align 1
  store i8 %load_input, ptr @THNTD, align 1
  ret void
}

define void @ResetState() {
entry:
  store i8 1, ptr @reset, align 1
  ret void
}

define void @__init___clampandsaw_st() {
entry:
  call void @__init_prg_clampandsaw(ptr @PRG_ClampAndSaw_instance)
  call void @__init___vtable_ton(ptr @__vtable_TON_instance)
  call void @__init___vtable_fb_clampandsaw(ptr @__vtable_FB_ClampAndSaw_instance)
  call void @__user_init_PRG_ClampAndSaw(ptr @PRG_ClampAndSaw_instance)
  call void @__user_init___vtable_TON(ptr @__vtable_TON_instance)
  call void @__user_init___vtable_FB_ClampAndSaw(ptr @__vtable_FB_ClampAndSaw_instance)
  ret void
}

define void @__init___vtable_ton(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %__body = getelementptr inbounds nuw %__vtable_TON, ptr %deref, i32 0, i32 0
  store ptr @TON, ptr %__body, align 8
  ret void
}

define void @__init___vtable_fb_clampandsaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %__body = getelementptr inbounds nuw %__vtable_TON, ptr %deref, i32 0, i32 0
  store ptr @FB_ClampAndSaw, ptr %__body, align 8
  ret void
}

define void @__init_teststruct(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__init_prg_clampandsaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %stateMachine = getelementptr inbounds nuw %PRG_ClampAndSaw, ptr %deref, i32 0, i32 0
  call void @__init_fb_clampandsaw(ptr %stateMachine)
  ret void
}

define void @__init_fb_clampandsaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %__vtable = getelementptr inbounds nuw %FB_ClampAndSaw, ptr %deref, i32 0, i32 0
  store ptr @__vtable_FB_ClampAndSaw_instance, ptr %__vtable, align 8
  ret void
}

define void @__user_init_TestStruct(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__user_init___vtable_TON(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__user_init_PRG_ClampAndSaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %stateMachine = getelementptr inbounds nuw %PRG_ClampAndSaw, ptr %deref, i32 0, i32 0
  call void @__user_init_FB_ClampAndSaw(ptr %stateMachine)
  ret void
}

define void @__user_init_FB_ClampAndSaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %threeseconds = getelementptr inbounds nuw %FB_ClampAndSaw, ptr %deref, i32 0, i32 1
  call void @__user_init_TON(ptr %threeseconds)
  ret void
}

define void @__user_init_TON(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__user_init___vtable_FB_ClampAndSaw(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}
