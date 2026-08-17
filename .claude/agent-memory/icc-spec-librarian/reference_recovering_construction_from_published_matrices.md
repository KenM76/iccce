---
name: recovering-construction-from-published-matrices
description: Techniques that recovered a published matrix's provenance — eigendecomposition to extract a CAT's cone matrix, and inverting a printed matrix to identify which rounded ancestor it descends from
metadata:
  type: reference
---

**Two cheap procedures that turned "where did these numbers come from?" from
unanswerable into settled, on ICC's `srgb.pdf` 2026-08-17. Reach for them
whenever a document prints a matrix without saying how it was built.**

### 1. Eigendecompose a chromatic-adaptation matrix to recover its CAT

A von-Kries-form matrix is `M = M_A⁻¹ · D · M_A` with `D` diagonal. Therefore
**the rows of `M_A` are `M`'s LEFT eigenvectors and the cone ratios are its
eigenvalues.** So `M_A` falls out of the published `chad` **alone**, assuming
nothing about source or destination white.

```python
w, V = numpy.linalg.eig(CHAD)      # w = cone ratios
Ma = numpy.linalg.inv(V)           # rows of Ma, up to per-row scale
row = Ma[i] / Ma[i].sum()          # normalise to sum 1 to compare with published Bradford
```

**Result:** ICC's published `chad` uses Bradford `M_A[0][0] = 0,8950` where
ICC.1:2022 Annex E.3 Eq. (E.1) prints `0,8951`. **Confirm with an exact forward
reconstruction in `Fraction`** — `0,8951` reproduces the tag to `5,7×10⁻⁶`,
`0,8950` to `5,7×10⁻¹⁶`. Two routes, and the second is the one that makes it a
finding rather than a numerical coincidence.

Also: **`chad⁻¹ · destination_white` recovers the source white exactly**,
independent of CAT. That identified `0,9505/1/1,0890` (a 4-dp rounding) as what
ICC's `chad` actually adapts from.

### 2. Invert a printed matrix to find its rounded ancestor

`inv(ICC srgb.pdf §A.7)` reproduced the **W3C-1996 4-decimal-place** sRGB→XYZ
matrix to `4,7×10⁻⁸`, and `inv(that 4-dp matrix)` rounded to 7 dp reproduced
**all nine cells** of §A.7. **⟹ §A.7 descends from a 4-dp print, not from
BT.709's chromaticities** — which explained every downstream residual.

**Diagnostic that flags this before you invert:** take the **column sums** of an
RGB→XYZ matrix (or row sums of its inverse's inverse). They are the implied
white. If they come out at a suspiciously round `0,9505 / 1 / 1,0890` rather
than the chromaticity-derived `0,950 455 927 / 1 / 1,089 057 751`, **the matrix
descends from rounded input.**

### Discipline
- **Exact `Fraction`, never `f64`**, for anything reported in ULP. `numpy` is
  fine for the eigendecomposition (a search), not for the verdict (a proof).
- **Report residuals in ULP of `s15Fixed16` (`1/65536 = 1,52588×10⁻⁵`)** — it is
  the unit that decides whether a difference can exist in a written profile at
  all. `0,37` ULP means *the bytes are identical and the claim is still false*.

Related: [[srgb-colorant-gap-routes-tried]], [[derived-values-need-a-second-pass]].
