Indective day : Type :=
    | monday
    | tuesday
    | wednesday
    | thursday
    | friday
    | satday
    | sunday.

Definition next_weekday (d: day) : day :=
    match d with
    | monday => tuesday
    | tuesday => wednesday
    | wednesday => thursday
    | thursday => friday
    | friday => monday
    | satday => monday
    | sunday => monday
    end .

Compute (next_weekday friday).
Compute (next_weekday (next_weekday saturday)).

Example test_next_weekday:
    (next_week (next_weekday saturday)) = tuesday.

Proof. simpl. reflesivity. Qed.
