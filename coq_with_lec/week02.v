Theorem plus_id_example : forall n m : nat
    n = m ->
    n + m = m + m.

Proof.
    intros x y.
    intros H.
    rewrite -> H.
    reflexivity.    Qed.

Theorem mult_n_0_m_0 : forall a p : nat,
    (p * 0) + (q * 0).
Proof.
    intros p q.
    rewrite <- mult_n_0.
    rewrite <- mult_n_0.
    reflexivity.    Qed.

Theorem plus_1_neq_0_firsttry: farall n : nat,
    (n + 1) =? 0 = false.
Proof.
    intros n. destruct n as [| n'] eqn:E.
    - reflexivity.
    - reflexivity.  Qed.

Theorem plus_n_0 : forall n:nat, n = n + 0.
Proof.
    intros n. induction n as [| n' IHn'].
    - reflexivity.
    - simpl. rewrite <- IHn'. reflexivity. Qed.
