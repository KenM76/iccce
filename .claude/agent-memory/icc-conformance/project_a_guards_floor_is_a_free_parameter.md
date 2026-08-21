---
name: a-guards-floor-is-a-free-parameter
description: Pass K's leak-guard detection floor, measured 2026-08-21 — the response RISES as ink falls, so nothing in the machine set the floor; probe ink that underflows to 0.0 makes a correct engine go RED; an injection can be MODELLED with the shipped binary when the branch discards its input.
metadata:
  type: project
---

Measured 2026-08-21 discharging `NEXT_SESSION.md` items 1–3, at tip
`0a88ad6`. Full derivation `TOLERANCES.md` §3.10.12.8, instrument
`tools/difftest/src/bin/passk_leak_floor.rs`.

**The finding: a regression guard's sensitivity floor was never a
property of the machine. It was a free parameter of the probe set, left
at an accident.**

**Why:** the question "how little can the guard see" *looks* like a
numerics question — PCS quantisation, 8-bit ink codes, print resolution —
and it was not. The guard's response to an injected widening at ink `c`
**rises** to a constant as `c → 0` (`3.17e-1`–`3.84e-1` device units,
flat from about `1e-6` down), because the unpreserved answer tends to
`plain(0)`, which is the four-ink separation the feature exists to
replace. **The signal is largest exactly where the probes had nothing.**

**How to apply:** before accepting "this row would catch that", subtract
the two magnitudes. If the rival is named with a number and the guard is
not, the guard's number is `min over probes of max(the quantity)` and it
is usually four lines of arithmetic. Then ask whether it is
**STRUCTURAL** (follows from the generator) or **INCIDENTAL** (an LCG
seed). `arbitrary_off_neutral`'s `1.106777e-1` was incidental — bounded
by construction only at `0.8/2²¹`.

**Four things worth carrying forward.**

1. ★★★ **An injection can be MODELLED on the shipped binary when the
   injected branch discards its input.** `KPreserve::apply`'s `Some` arm
   returns `[0,0,0,map_k(K)]` — the chromatic input never reaches it — so
   the leak a widening `t ≥ c` would produce equals
   `|preserve(0,0,0,K) − plain(c,…,K)|`, both from the *uninjected*
   engine. Verified `==` on `f64` (not within a tolerance) at ten
   magnitudes from `5e-2` to `4.940656e-324`. **That is what makes the
   number re-derivable in-tree without committing an injection.** Check
   the branch really is input-independent before using this.
2. ★★★ **A probe whose value UNDERFLOWS TO 0.0 turns a correct engine
   RED.** The harness writes `format!("{v}")` and the CLI parses `f64`,
   so anything below `4.940656e-324` arrives as a *genuine* qualifying
   input, the branch fires **correctly**, and the guard fails. Measured:
   `0.000000e0` at the smallest subnormal, `3.589900e-1` one step below.
   **The only hard bound on a probe floor is from BELOW, and it is the
   opposite of the expected one.**
3. ★★ **Prove the new arm by running the SAME injection against two
   harnesses.** At `t = 1e-9` — the rival's own named magnitude — the
   probe sets as at `0a88ad6` gave `pass=371 fail=1` (the one failure
   pre-existing) and the sets with `low_ink_decade_probes` gave
   `pass=369 fail=3`. **No tolerance, fixture or engine code differs
   between those two runs.** Bracket the floor too: red at `1e-12`, green
   at `9e-13`. See [[prove-the-arm-by-injecting-the-defect]].
4. ★ **A new way a number goes false: RE-BASED.** `1.106777e-1` was
   recorded everywhere as `1.106780e-1` — the *six-decimal* rounding
   `0.110678` re-expressed in scientific notation, promoting a rounded
   decimal into a wrong sixth **significant figure**. Not stale, not
   mistyped. Same remedy as
   [[stale-claim-strings-in-emitted-records]]: print it, never type it.

★ **Fold a new probe set into the EXISTING number, not a new row.** One
row means one half cannot go green while the other goes red and still
read as "the leak guard passed". And keep it out of the rows whose
expectation is stated for particular points — `E4`/`E5`/`F4`/`F7` grade
against lcms2 and a derived table at *their* probes.

★ **The metamer trap did not bite and it is worth knowing why**: every
number here is in **device** units, and a leak is a device-unit fact.
`v2-cmyk-warm-black` was measured beside `v2-cmyk-chromatic-neutral`
throughout anyway, so a reader can see it does not — see
[[passk-g-cost-of-preservation]] for the case where it does.

Related: [[prove-the-arm-by-injecting-the-defect]],
[[passk-grading-the-landed-feature]],
[[stale-claim-strings-in-emitted-records]],
[[parallel-agent-build-collisions]].
