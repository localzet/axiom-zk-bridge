# axiom-zk-bridge

A backend-neutral envelope for the **second proof layer**: proving that an already-approved program actually executed
with committed inputs and produced a committed output.

> **Maturity:** research prototype v0.1. The default verifier proves properties by exhaustive evaluation over an
> explicitly finite input domain. A VALID receipt is therefore a theorem about that bounded model, not a claim of
> unbounded program correctness.


v0.1 implements deterministic execution commitments and verifier envelopes, but deliberately ships **no fake
zero-knowledge prover**. Adapters for RISC Zero/SP1/Jolt or another zkVM can populate the opaque proof bytes later.

```bash
cargo run -- commit --program candidate.axp --input "x=-7" --output "7"
```
