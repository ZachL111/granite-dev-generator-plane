# Granite Dev Generator Plane Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 171 | ship |
| stress | diagnostic quality | 175 | ship |
| edge | review cost | 214 | ship |
| recovery | safe rewrite | 175 | ship |
| stale | change width | 259 | ship |

Start with `stale` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The next useful expansion would be a malformed fixture around diagnostic quality and safe rewrite.
