; Modül: aa_lang
declare i32 @printf(i8*, ...)
declare i32 @system(i8*)
@fmt_num = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00"
@cmd_chcp = private unnamed_addr constant [17 x i8] c"chcp 65001 > nul\00"
@str.0 = private unnamed_addr constant [14 x i8] c"Ogrenci Notu:\00"
@str.1 = private unnamed_addr constant [11 x i8] c"Derece: AA\00"
@str.2 = private unnamed_addr constant [11 x i8] c"Derece: BB\00"
@str.3 = private unnamed_addr constant [11 x i8] c"Derece: CC\00"
@str.4 = private unnamed_addr constant [16 x i8] c"Durum: Kaldiniz\00"
@str.5 = private unnamed_addr constant [30 x i8] c"Program basariyla tamamlandi.\00"
@str.6 = private unnamed_addr constant [21 x i8] c"Dizinin ilk elemani:\00"
@str.7 = private unnamed_addr constant [24 x i8] c"Dizinin ikinci elemani:\00"
@str.8 = private unnamed_addr constant [8 x i8] c"Toplam:\00"
@str.9 = private unnamed_addr constant [28 x i8] c"--- While Dongusu (0-4) ---\00"
@str.10 = private unnamed_addr constant [26 x i8] c"--- For Dongusu (0-4) ---\00"
@str.11 = private unnamed_addr constant [17 x i8] c" D\C3\B6ng\C3\BC sonu\C3\A7:\00"
@str.12 = private unnamed_addr constant [8 x i8] c"Merhaba\00"
@str.13 = private unnamed_addr constant [20 x i8] c"Merhaba, Nas\C4\B1ls\C4\B1n\00"


define i32 @topla(i32 %arg0, i32 %arg1) {
entry:
  %a_ptr = alloca i32
  store i32 %arg0, i32* %a_ptr
  %b_ptr = alloca i32
  store i32 %arg1, i32* %b_ptr
  %tmp33 = load i32, i32* %a_ptr
  %tmp34 = load i32, i32* %b_ptr
  %tmp35 = add i32 %tmp33, %tmp34
  ret i32 %tmp35
}

define i32 @main() {
entry:
  call i32 @system(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @cmd_chcp, i32 0, i32 0))
  %not_ptr = alloca i32
  store i32 75, i32* %not_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @str.0, i32 0, i32 0))
  %tmp1 = load i32, i32* %not_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp1)
  %tmp2 = load i32, i32* %not_ptr
  %tmp3 = icmp sgt i32 %tmp2, 90
  br i1 %tmp3, label %L0, label %L1
L0:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([11 x i8], [11 x i8]* @str.1, i32 0, i32 0))
  br label %L2
L1:
  %tmp4 = load i32, i32* %not_ptr
  %tmp5 = icmp sgt i32 %tmp4, 70
  br i1 %tmp5, label %L3, label %L4
L3:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([11 x i8], [11 x i8]* @str.2, i32 0, i32 0))
  br label %L5
L4:
  %tmp6 = load i32, i32* %not_ptr
  %tmp7 = icmp sgt i32 %tmp6, 50
  br i1 %tmp7, label %L6, label %L7
L6:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([11 x i8], [11 x i8]* @str.3, i32 0, i32 0))
  br label %L8
L7:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([16 x i8], [16 x i8]* @str.4, i32 0, i32 0))
  br label %L8
L8:
  br label %L5
L5:
  br label %L2
L2:
  %kontrol_ptr = alloca i32
  store i32 1, i32* %kontrol_ptr
  %tmp8 = load i32, i32* %kontrol_ptr
  %tmp9 = icmp eq i32 %tmp8, 1
  br i1 %tmp9, label %L9, label %L11
L9:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([30 x i8], [30 x i8]* @str.5, i32 0, i32 0))
  br label %L11
L11:
  %sayilar_ptr = alloca [3 x i32]
  %tmp10 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 0
  store i32 10, i32* %tmp10
  %tmp11 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 1
  store i32 20, i32* %tmp11
  %tmp12 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 2
  store i32 30, i32* %tmp12
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([21 x i8], [21 x i8]* @str.6, i32 0, i32 0))
  %tmp13 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 0
  %tmp14 = load i32, i32* %tmp13
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp14)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([24 x i8], [24 x i8]* @str.7, i32 0, i32 0))
  %tmp15 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 1
  %tmp16 = load i32, i32* %tmp15
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp16)
  %tmp17 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 0
  %tmp18 = load i32, i32* %tmp17
  %tmp19 = getelementptr inbounds [3 x i32], [3 x i32]* %sayilar_ptr, i32 0, i32 1
  %tmp20 = load i32, i32* %tmp19
  %tmp21 = add i32 %tmp18, %tmp20
  %toplam_ptr = alloca i32
  store i32 %tmp21, i32* %toplam_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([8 x i8], [8 x i8]* @str.8, i32 0, i32 0))
  %tmp22 = load i32, i32* %toplam_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp22)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([28 x i8], [28 x i8]* @str.9, i32 0, i32 0))
  %counter_ptr = alloca i32
  store i32 0, i32* %counter_ptr
  br label %L12
L12:
  %tmp23 = load i32, i32* %counter_ptr
  %tmp24 = icmp slt i32 %tmp23, 5
  br i1 %tmp24, label %L13, label %L14
L13:
  %tmp25 = load i32, i32* %counter_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp25)
  %tmp26 = load i32, i32* %counter_ptr
  %tmp27 = add i32 %tmp26, 1
  store i32 %tmp27, i32* %counter_ptr
  br label %L12
L14:
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([26 x i8], [26 x i8]* @str.10, i32 0, i32 0))
  %i_ptr = alloca i32
  store i32 0, i32* %i_ptr
  br label %L15
L15:
  %tmp28 = load i32, i32* %i_ptr
  %tmp29 = icmp slt i32 %tmp28, 5
  br i1 %tmp29, label %L16, label %L17
L16:
  %tmp30 = load i32, i32* %i_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp30)
  %tmp31 = load i32, i32* %i_ptr
  %tmp32 = add i32 %tmp31, 1
  store i32 %tmp32, i32* %i_ptr
  br label %L15
L17:
  %x_ptr = alloca i32
  store i32 10, i32* %x_ptr
  %y_ptr = alloca i32
  store i32 20, i32* %y_ptr
  %tmp36 = load i32, i32* %x_ptr
  %tmp37 = load i32, i32* %y_ptr
  %tmp38 = call i32 @topla(i32 %tmp36, i32 %tmp37)
  %sonuc_ptr = alloca i32
  store i32 %tmp38, i32* %sonuc_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([17 x i8], [17 x i8]* @str.11, i32 0, i32 0))
  %tmp39 = load i32, i32* %sonuc_ptr
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 %tmp39)
  %tmp40 = getelementptr inbounds [8 x i8], [8 x i8]* @str.12, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* %tmp40)
  %tmp41 = getelementptr inbounds [20 x i8], [20 x i8]* @str.13, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* %tmp41)
  ret i32 0
}
