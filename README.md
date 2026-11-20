# lib-utils

`utils` — stdlib utility modules (`bool`, `math`, `str`) plus the aggregate
capability surface.

- `src/bool.hnl` — pure-handl boolean helpers.
- `src/math.hnl` — `math.*` interface (handls in stdlib `ops::math`).
- `src/str.hnl` — `str.*` interface (handls in stdlib `ops::strs`).
- `src/utils.hnl` — the aggregate package interface (`cap` declarations).
- `src/math.test.hnl` — the `math` test suite.

## Layout

```text
src/        the .hnl sources
pkg.hnl     package manifest — modules, deps, description, license
scripts/hnl-gate.sh   the gate
ci.sh       bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
