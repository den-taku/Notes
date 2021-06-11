; ModuleID = 'struct.c'
source_filename = "struct.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.13.0"

%struct.munger_struct = type { i32, i32 }

; Function Attrs: norecurse nounwind ssp uwtable
define void @munge(%struct.munger_struct* nocapture) local_unnamed_addr #0 {
  %2 = getelementptr inbounds %struct.munger_struct, %struct.munger_struct* %0, i64 1, i32 0
  %3 = load i32, i32* %2, align 4, !tbaa !3
  %4 = getelementptr inbounds %struct.munger_struct, %struct.munger_struct* %0, i64 2, i32 1
  %5 = load i32, i32* %4, align 4, !tbaa !8
  %6 = add nsw i32 %5, %3
  %7 = getelementptr inbounds %struct.munger_struct, %struct.munger_struct* %0, i64 0, i32 0
  store i32 %6, i32* %7, align 4, !tbaa !3
  ret void
}

; Function Attrs: nounwind readnone ssp uwtable
define i32 @main() local_unnamed_addr #1 {
  ret i32 0
}

attributes #0 = { norecurse nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nounwind readnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"Apple LLVM version 10.0.0 (clang-1000.11.45.5)"}
!3 = !{!4, !5, i64 0}
!4 = !{!"munger_struct", !5, i64 0, !5, i64 4}
!5 = !{!"int", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
!8 = !{!4, !5, i64 4}
