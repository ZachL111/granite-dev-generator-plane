# granite-dev-generator-plane

`granite-dev-generator-plane` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies generator behavior through node-edge fixtures, with cycle and reachability reports and local-only command execution.

## Problem It Tries To Make Smaller

I want this repository to be useful as a quick reading exercise: fixtures first, implementation second, verifier last.

## Granite Dev Generator Plane Review Notes

`stale` and `baseline` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Working Pieces

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/granite-dev-generator-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Design Notes

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust code keeps the review rule close to the tests.

## Example Run

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The check exercises the source code and the review fixture. `stale` is the high score at 259; `baseline` is the low score at 171.

## Known Limits

No external service is required. A deeper version would add more negative cases and a clearer boundary around invalid input.
