# Completeness critic

The critic is a deterministic citation scan over the 13,041-row claim ledger and the
fourteen lens findings. Its verdict is that the census is exhaustive but the lens
analysis is not: coverage is a measured property, not an assertion.

## Measured coverage

- 490 of 13,041 claim rows (3.76 percent) are cited by another lens finding.
- 296 of 331 open-doc families, 43 of 63 GitHub families and 140 of 247 stub families have zero cited rows.
- 277 of 415 distinct family names have zero cited rows in all three corpora (4,170 claim rows).
- 24 open families, 11 GitHub families and 31 stub families have exactly one cited row.
- 45 CRITICAL and 1,449 HIGH open documents belong to zero-cited open families.
- 17,671 fixed documents (76.7 percent of 23,026 in-repo documents) contribute zero claim rows.
- 2,232 open documents (41.7 percent) and 232 GitHub issues (40.6 percent) are filed under _unclassified.

## Classification defects

- The three family tables are not a canonical service partition. census.py derives a family from the last dot of an Issue-Shape-Id for documents, a title-prefix regex for GitHub issues, and the first hyphen of a crate name for stubs.
- 14 hyphen-normalized alias groups split 283 claim rows across names.
- 2,649 open documents carry more than one Issue-Key; 109 cite docs.aws.amazon.com.
- The fixed-doc index retains six fields and drops every AWS-side evidence field.

## Consequence

No lens claims parity. 1:1 parity is NOT achieved. The residual is enumerated
exactly in BACKLOG.json and counted in PARITY-CENSUS.md. Fidelity to AWS for
every open family requires a live AWS replay of each family trace id; the corpus
contains no such replay. The substrate delivered in fabric/ is separately witnessed

## Source

findings/completeness.json (COMPL-01 through COMPL-11), reproduced verbatim under docs/findings/.