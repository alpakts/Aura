; Modül: aa_lang
declare i32 @printf(i8*, ...)
@fmt_num = private unnamed_addr constant [4 x i8] c"%d\0A\00"

define i32 @main() {
entry:
  %x_ptr = alloca i32
  store i32 5, i32* %x_ptr
  %y_ptr = alloca i32
  store i32 10, i32* %y_ptr
  %result_ptr = alloca i32
  %1 = load i32, i32* %x_ptr
  %2 = load i32, i32* %y_ptr
  %3 = add i32 %1, %2
  store i32 %3, i32* %result_ptr
  %4 = load i32, i32* %result_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %4)
  ret i32 0
}
