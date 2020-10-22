(* handout1 *)

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

(* 型を調べる *)
Check arg.

(* 既存の型を使った型定義 *)
Inductive type_name2 : Type :=
    | ele1
    | ele2
    | ele3 (ele : type_name).

(* パターンマッチのさせ方 *)
match arg with
| ele1 => exp1
| ele2 => exp2
| ele2 e => exp3
end.

(* 組 *)
Inductive type_name : Type :=
    | tuple_name (ele1 ele2 ele3 ele4 : type_name2).

Check (tuple_name (ele1 ele2 ele3 ele4).
    : type_name

(* 自然数の定義 *)
Inductive nat : Type :=
    | O
    | S (n: nat).

(* 再帰関数の定義 *)
Fixpoint function_name (arg:type_name) : value_type :=
    match arg with
    | ele1 => exp1
    | ele2 => exp2
    | ele3 (ele3')=> funtion_name ele3'
    end.

(* 定理 *)
Theorem theorem_name : forall arg:type_name, exp.

(* タクティック *)

(* 式の計算 *)
simpl. 

(* =の両辺は等しい.よって題意は示された*)
reflexivity. 

(* 文脈への仮定の導入 *)
(* 仮定は「文脈」contextに移動する *)
intros hypothesis_syntax. 

(* ならば *)
hypothesis_exp -> proposition_exp
(* intros H のように仮定を導入する *)

(* 仮定による等式の書き換え *)
(* すでに証明した定理も使える *)
rewrite -> H.
rewrite <- H.

(* 場合分け *)
(* -, +, *はbulletと呼ばれる *)
destruct arg as [arg_ele1|arg_ele2|arg_ele3] eqn: Eqn_name.
 - (* tactics *)
 - destruct arg2 as [|] eqn: Eqn_name2.
   + (* tactics *)
   + destruct qrg3 as [|] eqn: Eqn_name3.
    * (* tactics *)
    * (* tactics *)
 - (* tactics *)

(* introsとdestructは組み合わせることができる*)
intros x. destruct x as [|y]. => intros [|y].

(* handout2 *)




























































































