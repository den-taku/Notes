require import Bool DBool.

module A = {
  proc main() = {
    var: x;
    x <$ {0, 1};
    x <- x ^^ x;
    return x;
  }
}.

module B = {
  proc main() = {
    var y <- false;
    return y;
  }
}

lemma test &m :
  Pr[A.main() @ &m : res] = Pr[B.main() @ &m : res].
  proof. byequiv =>//.
  proc. by auto; smt.
qed.