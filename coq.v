(* 型を定義*)
Inductive type_name : Type :=
    | ele1
    | ele2
    | ele3.

(* 関数を定義 *)
Definition function_name (arg:type_name) : value_type :=
    match arg with
        | ele1 => ele1
        | ele2 => ele2
        | ele3 => ele3
        end.

(* 関数を実行 *)
Compute function_name arg.

(* 言明 *)
Example test_name:
    function_name arg = ele.

(* 証明 *)
Proof. (* such as *) simpl. reflexivity. (* *) Qed.

(* 構文解析時に展開されるマクロ *)
(* such as *)
Natation "x && y" := (andb x y).
