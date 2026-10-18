# lib-utils

`utils` — stdlib utility modules (`bool`, `math`, `str`) plus the aggregate
capability surface.

Part of the stdlib → `lib-*` split (ADR-0128); moved from
`stdlib/handl-packages/utils/`.

- `src/bool.hnl` — pure-handl boolean helpers.
- `src/math.hnl` — `math.*` interface (handls in stdlib `ops::math`).
- `src/str.hnl` — `str.*` interface (handls in stdlib `ops::strs`).
- `src/utils.hnl` — the aggregate package interface (`cap` declarations).
- `src/math.test.hnl` — the `math` test suite.
