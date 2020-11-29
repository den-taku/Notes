Definition f (x y : nat) := y + x.
Eval compute [f] in (fun y:nat => f (y*y)).
