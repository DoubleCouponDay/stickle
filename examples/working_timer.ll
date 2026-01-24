; ModuleID = 'C:\Users\charisse\AppData\Local\Temp\.tmpRPs0IV\Users\charisse\Documents\GitHub\stickle\examples\working_timer.st.ll'
source_filename = "examples\\working_timer.st"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%__vtable_TON = type { ptr }
%TON = type { ptr, i8, i64, i8, i64, i8, i8, [24 x i8] }
%__vtable_main = type { ptr }
%main = type { ptr, %TON }

@latched = global i8 0
@state = global i32 0
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___working_timer_st, ptr null }]
@__vtable_TON_instance = global %__vtable_TON zeroinitializer
@____vtable_TON__init = unnamed_addr constant %__vtable_TON zeroinitializer
@__TON__init = external unnamed_addr constant %TON
@____vtable_main__init = unnamed_addr constant %__vtable_main zeroinitializer
@__main__init = unnamed_addr constant %main zeroinitializer
@__vtable_main_instance = global %__vtable_main zeroinitializer

declare void @TON(ptr)

define void @main(ptr %0) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %__vtable = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
  %timer = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
  %1 = getelementptr inbounds %TON, ptr %timer, i32 0, i32 1
  store i8 1, ptr %1, align 1
  %2 = getelementptr inbounds %TON, ptr %timer, i32 0, i32 2
  store i64 3000000000, ptr %2, align 8
  call void @TON(ptr %timer)
  %Q = getelementptr inbounds nuw %TON, ptr %timer, i32 0, i32 3
  %load_Q = load i8, ptr %Q, align 1
  %3 = icmp ne i8 %load_Q, 0
  br i1 %3, label %condition_body, label %else

condition_body:                                   ; preds = %entry
  %4 = getelementptr inbounds %TON, ptr %timer, i32 0, i32 1
  store i8 0, ptr %4, align 1
  %5 = getelementptr inbounds %TON, ptr %timer, i32 0, i32 2
  store i64 3000000000, ptr %5, align 8
  call void @TON(ptr %timer)
  store i32 10, ptr @state, align 4
  store i8 1, ptr @latched, align 1
  br label %continue

else:                                             ; preds = %entry
  store i32 5, ptr @state, align 4
  br label %continue

continue:                                         ; preds = %else, %condition_body
  ret void
}

define i32 @GetState() {
entry:
  %GetState = alloca i32, align 4
  store i32 0, ptr %GetState, align 4
  %load_state = load i32, ptr @state, align 4
  store i32 %load_state, ptr %GetState, align 4
  %GetState_ret = load i32, ptr %GetState, align 4
  ret i32 %GetState_ret
}

define void @__init___working_timer_st() {
entry:
  call void @__init___vtable_ton(ptr @__vtable_TON_instance)
  call void @__init___vtable_main(ptr @__vtable_main_instance)
  call void @__user_init___vtable_TON(ptr @__vtable_TON_instance)
  call void @__user_init___vtable_main(ptr @__vtable_main_instance)
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

define void @__init___vtable_main(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %__body = getelementptr inbounds nuw %__vtable_TON, ptr %deref, i32 0, i32 0
  store ptr @main, ptr %__body, align 8
  ret void
}

define void @__init_main(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %__vtable = getelementptr inbounds nuw %main, ptr %deref, i32 0, i32 0
  store ptr @__vtable_main_instance, ptr %__vtable, align 8
  ret void
}

define void @__user_init___vtable_TON(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__user_init___vtable_main(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}

define void @__user_init_main(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  %deref = load ptr, ptr %self, align 8
  %timer = getelementptr inbounds nuw %main, ptr %deref, i32 0, i32 1
  call void @__user_init_TON(ptr %timer)
  ret void
}

define void @__user_init_TON(ptr %0) {
entry:
  %self = alloca ptr, align 8
  store ptr %0, ptr %self, align 8
  ret void
}
