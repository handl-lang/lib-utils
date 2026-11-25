# lib-utils

`utils` — stdlib utility modules (`bool`, `math`, `str`) plus the
aggregate capability surface.

## Layout

| path | role |
| --- | --- |
| `src/bool.hnl` | pure-handl boolean helpers |
| `src/math.hnl` | `math.*` interface (handls in stdlib `ops::math`) |
| `src/str.hnl` | `str.*` interface (handls in stdlib `ops::strs`) |
| `src/utils.hnl` | the aggregate package interface (`cap` declarations) |
| `src/math.test.hnl` | the `math` test suite |
| `atoms.yaml` | atom declarations |
| `pkg.hnl` | manifest — `modules: ["bool", "math", "str"]` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | the gate / bupha entry |

## Contract

- Interface/impl split: `math.hnl`/`str.hnl` declare `cap` surfaces;
  implementations are native handls in stdlib `ops::math` /
  `ops::strs`; `utils.hnl` aggregates them. Note the native
  `mood`-style impls live in `plugin-lib-utils`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
