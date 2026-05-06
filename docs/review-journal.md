# Review Journal

The review surface for `granite-dev-generator-plane` is deliberately narrow: one fixture, one scoring rule, and one local check.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 171, lane `ship`
- `stress`: `diagnostic quality`, score 175, lane `ship`
- `edge`: `review cost`, score 214, lane `ship`
- `recovery`: `safe rewrite`, score 175, lane `ship`
- `stale`: `change width`, score 259, lane `ship`

## Note

This file is intentionally plain so the fixture remains the source of truth.
