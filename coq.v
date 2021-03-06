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

injection H as Hnm.
(* S m = S n -> n = m *)
(* HをinjectしてHnmとする *)
(* 名前をつけないとゴール中に仮定としてつく *)

discriminate
(* 矛盾を指摘してサブゴールを解消する *)
(* 爆発則 *)

Theorem f_equal :
forall (A B : Type) (f: A -> B) (x y : A),
    x = y -> f x = f y.

(* 仮定に対してtacticsを使う *)

tactics in H.
(* Hに対してtacticsを使う *)

generalize dependent
(* 文脈で仮定した変数を再び全少量化する *)

unfold D.
(* 定義を展開する *)

destruct (exp) eqn: H.
(* expがtrueの時とfalseの時で場合分け *)

(* handout6 *)

(* 命題の型 *)
Prop

(* conjunction *)
/\

(* ゴールを分けるtactics *)
split

(* disjunction *)
\/

(* tactics *)
left
right

(* 仮定にFalseが出てきたとき *)
(* 爆発則より *)
destruct hoge.

(* 爆発則を適応するtactics *)
exfalso.

(* 同値関係 *)
iff

(* 存在量化 *)
exists n.

(* tactics *)
exists 2.

(* 再帰的命題 *)
Fixpoint In {A : Type} (x : A) (l : list A) : Prop :=
    match l with
    | [] => False
    | x' :: l' => x' = x \/ In x l'
    end.

(* 証明の型は命題 *)

(* 証明なしで命題を追加する *)

Axiom

(* 使った公理を示す *)

Print Assamptions

(* Coqでは排中律を示せない(構成的論理) *)

(* handout7 *)

(* 帰納的定義は規則について閉じ、最小性を有す *)

Inductive ev : nat -> Prop :=
| ev_0 : ev 0
|ev_SS (n: nat) (H : ev n) : ev (S (S n)).

(* ev_0などはapplyできる *)

(* tactic inversion *)
(* 定理を遡る *)
(* destructよりも強力 *)
inversinon E as [|].

(* 導出についての帰納法 *)
(* 証明オブジェクトに対するinductionをすれば良い *)








































































