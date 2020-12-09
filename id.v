Definition id :=
    fun X : Type => fun x : X => x.

Check id (forall Y : Type, Y -> Y) id.
