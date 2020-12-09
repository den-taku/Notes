(* Coq syntax *)

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

(* 別ファイルの定義・定理を読み込める *)
(* 詳しくはIntroduction.vの冒頭を読むべし *)

(* 数学的帰納法を使った証明 *)
(* Induction Hypothesisを仮定して示す *)
(* IHarg'が帰納法の仮定につける名前 *)
induction arg as [|arg' IHarg'].

(* サブ定理(補題) *)
assert (H_name: hypothesis_exp). { (* tactics *) }
assert (hypothesis_exp) as H_name. { (* tactics *) }

(* 証明の置き換え *)
replace (A) with (B).

(* handout03 *)

(* 自然数のペア *)
Inductive natprod : Type :=
| pair (n1 n2 : nat).

Notation "( x , y )" := (pair x y).

(* リスト *)
Inductive natlist : Type :=
| nil
| cons (n: nat) (l: natlist).

cons -> n :: l
[n1; n2; n3]

(* 数学的帰納法を自然数の場合と同様に使える *)

(* 部分写像 *)
Inductive id : Type :=
| Id (n: nat).

Difinition eqb_id (x1 x2 : id) :=
    match x1, x2 with
    |Id n1, Id n2 => n1 =? n2
    end.

Inductive partial_map : Type :=
    | empty
    | record (i : id) (v : nat) (m : partial_map).

(* 写像の更新 *)
Difinition update (d: partial_map)
                  (x : id) (value : nat)
                  : partial_map :=
    record x value d.

(* 線形探索 *)
Fixpoint find (x : id) (d: partial_map)
    : natoption :=
    match d with
    | empty         => None
    | record y v d' => if eqb_id x y
                       then Some v
                       else find x d'
    end.

(* handout4 *)

(* 型パラメータを引数にとるリスト *)
(* 多相的型定義 *)
Inductive list (X:Type) : Type :=
    | nil
    | cons (x : X) (l : list X).

nil nat
cons nat 3 (nil nat)

X : Type で抽象化

(* 引数の暗黙化 *)

Arguments nil {X}.
Arguments cons {X} _ _.

(* 関数引数の暗黙化 *)

fixpoint repeat''' {X : Type} (x : X) (count : nat) : list X :=
    match count with
    | O        => nil
    | S count' => cons x (repeat''' x count')
    end.

(* 暗黙化を無効化するオペレータ@ *)

Difinition mynil := @nil nat.

(* 証明における型の全称量化 *)

forall X:Type

(* 高階関数も扱える *)

(* 匿名関数 *)

fun x => expression.

(* handout5 *)

(* tactics *)

apply
(* ゴールと仮定が等しければ終了 *)
(* 適用する仮定が仮定を持つならその過程まで遡る *)
(* 全称量子化されたものは具体化する *)

symmetry
(* 等式の左右が入れ替わる *)

apply with
(* 具体化できない中間の要素を処理系に知らせる *)

transitivity
(* 推移律に関してはこれでも良い *)



















































































