# Adversarial census verdict ledger

Live vote results already produced by the refuters of `wf_b49262e6-0ba`. Each claim is judged by three independent lenses: correctness, spec and repro. A claim survives only when all three votes are recorded and at least two refuters fail to refute it.

| measure | value |
|---|---:|
| journal_rows | 527 |
| unique_verdict_keys | 212 |
| refuted_votes | 194 |
| not_refuted_votes | 18 |
| failed_keys | 7 |
| claims_seen | 52 |
| claims_complete | 48 |
| claims_pending | 4 |

## Votes per claim

| claim | correctness | spec | repro | not-refuted | complete | survived |
|---|---|---|---|---:|---|---|
| EF-01 | refuted | refuted | refuted | 0 | True | False |
| EF-02 | refuted | refuted | refuted | 0 | True | False |
| EF-03 | refuted | refuted | refuted | 0 | True | False |
| EF-04 | refuted | refuted | refuted | 0 | True | False |
| EF-05 | not refuted | not refuted | not refuted | 3 | True | True |
| EF-06 | not refuted | refuted | refuted | 1 | True | False |
| EF-07 | refuted | refuted | refuted | 0 | True | False |
| EF-08 | refuted | refuted | refuted | 0 | True | False |
| EF-09 | refuted | refuted | refuted | 0 | True | False |
| EF-10 | refuted | refuted | refuted | 0 | True | False |
| EF-11 | refuted | refuted | refuted | 0 | True | False |
| F-01 | not refuted | refuted | refuted | 1 | True | False |
| F-02 | refuted | refuted | refuted | 0 | True | False |
| F-03 | refuted | refuted | refuted | 0 | True | False |
| F-04 | refuted | refuted | refuted | 0 | True | False |
| F-05 | not refuted | not refuted | not refuted | 3 | True | True |
| F-06 | refuted | refuted | refuted | 0 | True | False |
| F-07 | not refuted | refuted | refuted | 1 | True | False |
| F-08 | not refuted | refuted | refuted | 1 | True | False |
| F-09 | refuted | refuted | refuted | 0 | True | False |
| F-10 | refuted | refuted | refuted | 0 | True | False |
| F-11 | refuted | refuted | refuted | 0 | True | False |
| F-12 | not refuted | pending | not refuted | 2 | False | False |
| F-13 | refuted | refuted | refuted | 0 | True | False |
| F-14 | refuted | refuted | pending | 0 | False | False |
| F-15 | refuted | refuted | refuted | 0 | True | False |
| F-16 | pending | pending | refuted | 0 | False | False |
| F-17 | pending | refuted | refuted | 0 | False | False |
| L5-01 | refuted | refuted | refuted | 0 | True | False |
| L5-02 | refuted | refuted | refuted | 0 | True | False |
| L5-03 | refuted | refuted | refuted | 0 | True | False |
| L5-04 | refuted | refuted | refuted | 0 | True | False |
| L5-05 | refuted | refuted | refuted | 0 | True | False |
| L5-06 | refuted | refuted | not refuted | 1 | True | False |
| L5-07 | refuted | refuted | refuted | 0 | True | False |
| L5-08 | refuted | refuted | refuted | 0 | True | False |
| L5-09 | refuted | refuted | refuted | 0 | True | False |
| L5-10 | refuted | refuted | refuted | 0 | True | False |
| L5-11 | refuted | refuted | refuted | 0 | True | False |
| L5-12 | refuted | refuted | refuted | 0 | True | False |
| LR-01 | refuted | refuted | refuted | 0 | True | False |
| LR-02 | refuted | refuted | refuted | 0 | True | False |
| LR-03 | refuted | refuted | refuted | 0 | True | False |
| LR-04 | refuted | refuted | refuted | 0 | True | False |
| LR-05 | refuted | refuted | refuted | 0 | True | False |
| LR-06 | refuted | refuted | refuted | 0 | True | False |
| LR-07 | refuted | refuted | refuted | 0 | True | False |
| LR-08 | refuted | refuted | refuted | 0 | True | False |
| LR-09 | refuted | refuted | refuted | 0 | True | False |
| LR-10 | refuted | refuted | refuted | 0 | True | False |
| LR-11 | refuted | refuted | refuted | 0 | True | False |
| LR-12 | refuted | refuted | refuted | 0 | True | False |

## Status

Survivors: EF-05, F-05.
The Synthesize phase has not completed; the ranked backlog and completeness critic are recorded separately in ADVERSARIAL-CENSUS.md when its receipt lands. This ledger is a vote tally, not a synthesis.

1:1 AWS parity is NOT achieved. Exact residual: 5,355 open in-repo gap documents, 518 open GitHub issues, 7,011 true marker stubs, 440 service crates, 13,041 census claim rows.
