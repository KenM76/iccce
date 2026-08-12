//! # iccce-measure — CGATS / IT8.7 measurement data
//!
//! Reads the text files a spectrophotometer produces and a profiler
//! consumes: a header of `KEYWORD value` properties, a declared field
//! list, and a table of measured patches.
//!
//! ## Why this crate exists before any profiler does
//!
//! Profile *creation* (`docs/ROADMAP.md` Pass 10) is blocked on
//! measurement hardware — but only for **one** claim: *"this profile
//! describes that printer."* Everything upstream of that claim is
//! parsing, colorimetry and fitting, none of which needs an
//! instrument. This crate is the first of those pieces, built now so
//! Pass 10 can start rather than stall.
//!
//! ## Contracts
//!
//! - **INVARIANT: no ICC.** A measurement file is not a profile. The
//!   split means a future profiler and a future measurement tool
//!   share one reader, and this crate's tests never need an ICC
//!   fixture.
//! - **INVARIANT: no colour maths.** Values are returned as declared
//!   and as parsed. Deciding that `LAB_L` means CIE L\* under some
//!   observer is the consumer's act, and a spectral→XYZ integration
//!   needs colour-matching functions this project has not yet sourced
//!   (a recorded corpus gap).
//! - **Reports, does not repair** — the same rule as the ICC parser.
//!   A field count that disagrees with `NUMBER_OF_FIELDS` is reported
//!   and the file is still readable.
//!
//! ## Sourcing, and one licence hazard worth naming
//!
//! Structure and keyword vocabulary are taken from **lcms2's
//! `cmscgats.c`** (MIT, vendored at `tools/difftest/vendor`) —
//! `impl_crosscheck` tier, exactly as the project's other
//! implementation-derived work. CGATS.17 itself is a paywalled
//! NPES/CGATS document and is **not** sourced; where lcms2's reader
//! is more permissive than the standard may be, this follows lcms2
//! and says so.
//!
//! ★ **Argyll CMS is AGPL-3.0 and must not be read or cited for this
//! work.** It is by far the most tempting reference in this subject
//! area and it is copyleft; this project is MIT (`docs/LEGAL.md` §1).
//! The hazard is recorded here because the temptation recurs.
//!
//! ## ★ Do not recompute `LAB_*` from `XYZ_*`. Use the file's own columns.
//!
//! A characterisation file typically prints **both** `XYZ_*` and
//! `LAB_*` for every patch, which invites a consumer to keep one and
//! derive the other. **Do not.** The two column groups are not
//! guaranteed to have been produced under the white point *you* would
//! use to convert between them, and in the most important real-world
//! dataset they were not.
//!
//! Measured on **FOGRA51** (1 617 patches, read from the `targ` tag of
//! `PSOcoated_v3.icc`, ISO 28178, `TARGET_TYPE "ISO12642-2"`, filter
//! M1), converting the file's `LAB_*` back to XYZ and comparing against
//! the file's own `XYZ_*`, counting a patch as agreeing within `0,005`
//! (the half-ULP of the file's 2-decimal XYZ printing):
//!
//! | White point used for Lab→XYZ | Patches agreeing | Max residual |
//! |---|---|---|
//! | ICC PCS D50 — `96,42 / 100 / 82,49` | **651 / 1 617** | `0,0332` |
//! | `96,422 / 100 / 82,521` | **1 617 / 1 617** | `0,0050` |
//!
//! The second row is a perfect fit at exactly the rounding limit, so
//! **FOGRA51's Lab columns were computed with a D50 that is not ICC's.**
//!
//! The cost of ignoring this is bounded and non-trivial: recomputing
//! Lab from the file's XYZ under ICC's D50 differs from the file's
//! printed Lab by up to **`0,2146` ΔE76** (mean `0,0326`).
//!
//! **A subtlety worth stating, because it decides how you test this:
//! only the Lab→XYZ direction has discriminating power.** Going
//! XYZ→Lab, the two white points differ by `0,2146` vs `0,2140` ΔE76 —
//! indistinguishable — because the 2-decimal quantisation of the `XYZ_*`
//! columns swamps the white-point difference. A check run in that
//! direction would have found nothing and concluded, wrongly, that the
//! white points agree.
//!
//! Consequences for a consumer of this crate:
//!
//! - **Prefer `LAB_*` as authoritative** when the file provides it.
//!   Those are the measured, published values.
//! - **A round-trip test against FOGRA51 has a floor of ~`0,03` ΔE76**
//!   from the data's own printed precision. A tolerance tighter than
//!   that is measuring the file's decimal places, not your engine.
//! - This crate does none of the above for you — it has **no colour
//!   maths** by invariant. It hands you both column groups exactly as
//!   parsed; which one is authoritative is the consumer's decision, and
//!   this note exists so that decision is made knowingly.
//!
//! Verified 2026-08-12 by extracting the `targ` tag and doing the
//! arithmetic **outside this crate**, so the finding does not depend on
//! this parser being correct. Corpus:
//! `ICC_Spec\cgats\cgats__ref__characterisation_data_sourcing.md`.
//!
//! ## Exercised against
//!
//! Besides the unit tests, this reader has been run over the real
//! FOGRA51 `targ` payload above — 123 455 bytes, 11 fields, 1 617 data
//! rows — and returned **1 617 rows with zero `Issue`s** and every
//! `LAB_L` cell parsed. The file is **not committed**: its licence
//! permits local use but its redistribution terms are unresolved
//! (`docs/LEGAL.md`), so the corpus holds the archive and the tests
//! here stay synthetic.

use std::collections::BTreeMap;

/// A parsed measurement file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeasurementSet {
    /// Header properties in file order-independent form: `ORIGINATOR`,
    /// `CREATED`, `INSTRUMENTATION`, `MEASUREMENT_SOURCE`, … Values
    /// are stored with surrounding quotes stripped but otherwise
    /// verbatim.
    pub properties: BTreeMap<String, String>,
    /// The declared field names, in column order, from the
    /// `BEGIN_DATA_FORMAT` block — e.g. `SAMPLE_ID`, `CMYK_C`,
    /// `LAB_L`, `SPECTRAL_NM_380`.
    pub fields: Vec<String>,
    /// One row per patch, each with `fields.len()` values (or a
    /// reported [`Issue::FieldCountMismatch`] and whatever was
    /// present).
    pub rows: Vec<Vec<Value>>,
    /// Everything the file got wrong. Never corrected.
    pub issues: Vec<Issue>,
}

/// A single cell. CGATS is not typed per column, so a cell is a
/// number when it parses as one and text otherwise — the file's own
/// content decides, not a schema this crate invents.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Text(String),
}

impl Value {
    /// The numeric value, if this cell is one.
    #[must_use]
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Text(_) => None,
        }
    }

    /// The text, whichever variant this is (numbers render as parsed).
    #[must_use]
    pub fn as_text(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Text(s) => s.clone(),
        }
    }
}

/// A rule violation the file carries. Reported; never repaired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// A data row has a different number of cells than there are
    /// declared fields. The row is kept as parsed.
    FieldCountMismatch {
        row: usize,
        expected: usize,
        actual: usize,
    },
    /// `NUMBER_OF_FIELDS` disagrees with the `DATA_FORMAT` block.
    /// Both numbers are reported; the DECLARED FIELDS win, because
    /// they are what the columns actually are.
    NumberOfFieldsDisagrees { declared: usize, counted: usize },
    /// `NUMBER_OF_SETS` disagrees with the row count.
    NumberOfSetsDisagrees { declared: usize, counted: usize },
    /// A `BEGIN_*` block was never closed by its `END_*`.
    UnterminatedBlock { block: &'static str },
    /// Data appeared before any `BEGIN_DATA_FORMAT` declared what the
    /// columns mean.
    DataBeforeFormat { line: usize },
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldCountMismatch {
                row,
                expected,
                actual,
            } => write!(f, "row {row}: {actual} values, {expected} fields declared"),
            Self::NumberOfFieldsDisagrees { declared, counted } => write!(
                f,
                "NUMBER_OF_FIELDS says {declared}, DATA_FORMAT lists {counted}"
            ),
            Self::NumberOfSetsDisagrees { declared, counted } => {
                write!(f, "NUMBER_OF_SETS says {declared}, file has {counted} rows")
            }
            Self::UnterminatedBlock { block } => write!(f, "{block} was never closed"),
            Self::DataBeforeFormat { line } => {
                write!(f, "line {line}: data before BEGIN_DATA_FORMAT")
            }
        }
    }
}

/// The parse could not proceed at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No `BEGIN_DATA` block anywhere — this is not a measurement
    /// file, whatever else it may be.
    NoDataBlock,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDataBlock => write!(f, "no BEGIN_DATA block: not a CGATS measurement file"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Split a CGATS line into tokens, honouring double-quoted strings
/// (which may contain spaces) and stripping `#` comments.
///
/// Quote handling is the one place a naive `split_whitespace` breaks
/// on real files: `ORIGINATOR "Some Company Ltd"` is two tokens, not
/// four.
fn tokenize(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut started = false;
    for ch in line.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                started = true;
            }
            '#' if !in_quotes => break, // comment to end of line
            c if c.is_whitespace() && !in_quotes => {
                if started {
                    out.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            c => {
                cur.push(c);
                started = true;
            }
        }
    }
    if started {
        out.push(cur);
    }
    out
}

/// Parse a CGATS/IT8.7 measurement file.
///
/// The shape, per lcms2's reader: free-form `KEYWORD value` lines,
/// then `BEGIN_DATA_FORMAT` … `END_DATA_FORMAT` declaring the
/// columns, then `BEGIN_DATA` … `END_DATA` holding one row per patch.
/// Field names may span several lines inside the format block, which
/// real files do use.
pub fn parse(text: &str) -> Result<MeasurementSet, ParseError> {
    let mut set = MeasurementSet::default();
    let mut in_format = false;
    let mut in_data = false;
    let mut saw_data_block = false;
    let mut declared_fields: Option<usize> = None;
    let mut declared_sets: Option<usize> = None;

    for (lineno, raw) in text.lines().enumerate() {
        let tokens = tokenize(raw);
        if tokens.is_empty() {
            continue;
        }
        match tokens[0].as_str() {
            "BEGIN_DATA_FORMAT" => in_format = true,
            "END_DATA_FORMAT" => in_format = false,
            "BEGIN_DATA" => {
                in_data = true;
                saw_data_block = true;
                if set.fields.is_empty() {
                    set.issues
                        .push(Issue::DataBeforeFormat { line: lineno + 1 });
                }
            }
            "END_DATA" => in_data = false,
            _ if in_format => {
                // Field names, possibly several per line.
                set.fields.extend(tokens);
            }
            _ if in_data => {
                let row: Vec<Value> = tokens
                    .into_iter()
                    .map(|t| match t.parse::<f64>() {
                        Ok(n) => Value::Number(n),
                        Err(_) => Value::Text(t),
                    })
                    .collect();
                if !set.fields.is_empty() && row.len() != set.fields.len() {
                    set.issues.push(Issue::FieldCountMismatch {
                        row: set.rows.len(),
                        expected: set.fields.len(),
                        actual: row.len(),
                    });
                }
                set.rows.push(row);
            }
            key => {
                // A header property: KEYWORD followed by its value.
                let value = tokens[1..].join(" ");
                if key == "NUMBER_OF_FIELDS" {
                    declared_fields = value.parse().ok();
                } else if key == "NUMBER_OF_SETS" {
                    declared_sets = value.parse().ok();
                }
                // `KEYWORD` declares a custom property name; record it
                // as a property like any other rather than acting on
                // it (lcms2 uses it to extend its own table).
                set.properties.insert(key.to_string(), value);
            }
        }
    }

    if in_format {
        set.issues.push(Issue::UnterminatedBlock {
            block: "BEGIN_DATA_FORMAT",
        });
    }
    if in_data {
        set.issues.push(Issue::UnterminatedBlock {
            block: "BEGIN_DATA",
        });
    }
    if !saw_data_block {
        return Err(ParseError::NoDataBlock);
    }
    // Declared counts are checked against reality and REPORTED — the
    // parsed structure always reflects what the file contains, not
    // what its header claims.
    if let Some(n) = declared_fields {
        if n != set.fields.len() {
            set.issues.push(Issue::NumberOfFieldsDisagrees {
                declared: n,
                counted: set.fields.len(),
            });
        }
    }
    if let Some(n) = declared_sets {
        if n != set.rows.len() {
            set.issues.push(Issue::NumberOfSetsDisagrees {
                declared: n,
                counted: set.rows.len(),
            });
        }
    }
    Ok(set)
}

impl MeasurementSet {
    /// Column index of a named field, if present.
    #[must_use]
    pub fn field_index(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f == name)
    }

    /// All values of one named field, as numbers. `None` for rows
    /// where the field is absent or non-numeric — a hole is reported
    /// as a hole, not filled in.
    #[must_use]
    pub fn column(&self, name: &str) -> Option<Vec<Option<f64>>> {
        let idx = self.field_index(name)?;
        Some(
            self.rows
                .iter()
                .map(|r| r.get(idx).and_then(Value::as_number))
                .collect(),
        )
    }

    /// The spectral field names present, in wavelength order, with
    /// their wavelengths in nm — e.g. `SPECTRAL_NM_380` → 380.
    ///
    /// WHY parsed rather than assumed: real files use several
    /// spellings and several wavelength ranges/intervals, and a
    /// profiler must adapt to the file rather than demand a shape.
    /// Both `SPECTRAL_NM_400` and `nm400` are recognised because both
    /// occur; anything else is left alone rather than guessed at.
    #[must_use]
    pub fn spectral_fields(&self) -> Vec<(f64, usize)> {
        let mut out: Vec<(f64, usize)> = self
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                let up = f.to_ascii_uppercase();
                let digits = up
                    .strip_prefix("SPECTRAL_NM_")
                    .or_else(|| up.strip_prefix("NM"))?;
                digits.parse::<f64>().ok().map(|nm| (nm, i))
            })
            .collect();
        out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"IT8.7/4
ORIGINATOR "Some Company Ltd"
DESCRIPTOR "Test chart"
CREATED "2026-08-12"
INSTRUMENTATION "i1Pro 2"
NUMBER_OF_FIELDS 8
NUMBER_OF_SETS 3
BEGIN_DATA_FORMAT
SAMPLE_ID CMYK_C CMYK_M CMYK_Y CMYK_K LAB_L LAB_A LAB_B
END_DATA_FORMAT
BEGIN_DATA
A1 0.00 0.00 0.00 0.00 95.12 0.51 -2.31
A2 100.00 0.00 0.00 0.00 55.10 -37.20 -50.11   # cyan
A3 0.00 0.00 0.00 100.00 16.49 0.02 0.01
END_DATA
"#;

    #[test]
    fn parses_a_well_formed_file() {
        let m = parse(SAMPLE).unwrap();
        assert_eq!(m.fields.len(), 8);
        assert_eq!(m.rows.len(), 3);
        assert_eq!(m.issues, vec![]);
        // Quoted values keep their spaces and lose their quotes.
        assert_eq!(
            m.properties.get("ORIGINATOR").map(String::as_str),
            Some("Some Company Ltd")
        );
        assert_eq!(
            m.properties.get("INSTRUMENTATION").map(String::as_str),
            Some("i1Pro 2")
        );
        // Sample IDs are text; measurements are numbers.
        assert_eq!(m.rows[0][0], Value::Text("A1".into()));
        assert_eq!(m.rows[2][4], Value::Number(100.0));
    }

    /// A trailing `#` comment must not become a data column — the
    /// cyan row has a comment and still has exactly 8 values.
    #[test]
    fn comments_are_stripped_not_counted() {
        let m = parse(SAMPLE).unwrap();
        assert_eq!(m.rows[1].len(), 8);
        assert_eq!(m.rows[1][7], Value::Number(-50.11));
    }

    /// Named-column access, and holes reported as holes.
    #[test]
    fn columns_by_name() {
        let m = parse(SAMPLE).unwrap();
        let l = m.column("LAB_L").unwrap();
        assert_eq!(l, vec![Some(95.12), Some(55.10), Some(16.49)]);
        // SAMPLE_ID is text: numbers are None, not zero.
        assert_eq!(m.column("SAMPLE_ID").unwrap(), vec![None, None, None]);
        assert!(m.column("NO_SUCH_FIELD").is_none());
    }

    /// Header counts that disagree with reality are REPORTED and the
    /// parsed structure still reflects the file (report, don't
    /// repair — the same rule as the ICC parser).
    #[test]
    fn declared_counts_are_checked_and_reported() {
        let bad = SAMPLE.replace("NUMBER_OF_SETS 3", "NUMBER_OF_SETS 5");
        let m = parse(&bad).unwrap();
        assert!(m.issues.contains(&Issue::NumberOfSetsDisagrees {
            declared: 5,
            counted: 3,
        }));
        assert_eq!(m.rows.len(), 3, "the file's actual rows, not its claim");
    }

    /// A short row is reported by index and kept as parsed.
    #[test]
    fn short_row_reported_and_kept() {
        let bad = SAMPLE.replace("A3 0.00 0.00 0.00 100.00 16.49 0.02 0.01", "A3 0.00 0.00");
        let m = parse(&bad).unwrap();
        assert!(m.issues.iter().any(|i| matches!(
            i,
            Issue::FieldCountMismatch {
                row: 2,
                expected: 8,
                actual: 3
            }
        )));
        assert_eq!(m.rows[2].len(), 3);
    }

    /// Not a measurement file at all: refused by name.
    #[test]
    fn non_measurement_text_refused() {
        assert_eq!(parse("hello\nworld\n"), Err(ParseError::NoDataBlock));
    }

    /// Spectral columns are discovered from the file, in wavelength
    /// order, in either spelling that occurs in the wild.
    #[test]
    fn spectral_fields_discovered_in_order() {
        let spec = "NUMBER_OF_FIELDS 4\nBEGIN_DATA_FORMAT\n\
                    SAMPLE_ID SPECTRAL_NM_420 SPECTRAL_NM_400 nm410\n\
                    END_DATA_FORMAT\nBEGIN_DATA\nA1 0.1 0.2 0.3\nEND_DATA\n";
        let m = parse(spec).unwrap();
        let s = m.spectral_fields();
        assert_eq!(s, vec![(400.0, 2), (410.0, 3), (420.0, 1)]);
    }

    /// Field names may span several lines inside the format block —
    /// real files do this, and lcms2 accepts it.
    #[test]
    fn field_names_may_span_lines() {
        let split = SAMPLE.replace(
            "SAMPLE_ID CMYK_C CMYK_M CMYK_Y CMYK_K LAB_L LAB_A LAB_B",
            "SAMPLE_ID CMYK_C CMYK_M CMYK_Y\nCMYK_K LAB_L LAB_A LAB_B",
        );
        let m = parse(&split).unwrap();
        assert_eq!(m.fields.len(), 8);
        assert_eq!(m.issues, vec![]);
    }
}
