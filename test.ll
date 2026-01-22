; Modül: aa_lang
declare i32 @printf(i8*, ...)
@fmt_num = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00"
@str.0 = private unnamed_addr constant [14 x i8] c"Ogrenci Notu:\00"
@str.1 = private unnamed_addr constant [13 x i8] c"Durum: Gecti\00"
@str.2 = private unnamed_addr constant [15 x i8] c"Derece: Pekiyi\00"
@str.3 = private unnamed_addr constant [13 x i8] c"Durum: Kaldi\00"
@str.4 = private unnamed_addr constant [20 x i8] c"Butunlemeye girmeli\00"
@str.5 = private unnamed_addr constant [13 x i8] c"X kucuktur Y\00"
@str.6 = private unnamed_addr constant [13 x i8] c"X buyuktur Y\00"

define i32 @main() {
entry:
  %not_ptr = alloca i32
  store i32 40, i32* %not_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @str.0, i32 0, i32 0))
  %tmp1 = load i32, i32* %not_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp1)
  %tmp2 = load i32, i32* %not_ptr
  %tmp3 = icmp sgt i32 %tmp2, 50
  br i1 %tmp3, label %L0, label %L1
L0:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.1, i32 0, i32 0))
  %tmp4 = load i32, i32* %not_ptr
  %tmp5 = icmp sgt i32 %tmp4, 90
  br i1 %tmp5, label %L3, label %L5
L3:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([15 x i8], [15 x i8]* @str.2, i32 0, i32 0))
  br label %L5
L5:
  br label %L2
L1:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.3, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([20 x i8], [20 x i8]* @str.4, i32 0, i32 0))
  br label %L2
L2:
  %x_ptr = alloca i32
  store i32 10, i32* %x_ptr
  %y_ptr = alloca i32
  store i32 5, i32* %y_ptr
  %tmp6 = load i32, i32* %x_ptr
  %tmp7 = load i32, i32* %y_ptr
  %tmp8 = icmp slt i32 %tmp6, %tmp7
  br i1 %tmp8, label %L6, label %L7
L6:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.5, i32 0, i32 0))
  br label %L8
L7:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str.6, i32 0, i32 0))
  br label %L8
L8:
  ret i32 0
}
