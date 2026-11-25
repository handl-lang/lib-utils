# lib-utils

`utils` — stdlib utility modules (`bool`, `math`, `str`) plus the
aggregate capability surface.

## Layout

| path | role |
| --- | --- |
| `src/bool.hnl` | pure-handl boolean helpers |
| `src/math.hnl` | `math.*` documentation surface — atom spec in `atoms.yaml` (impls: stdlib `ops::math`) |
| `src/str.hnl` | `str.*` documentation surface — atom spec in `atoms-str.yaml` (impls: stdlib `ops::strs`) |
| `src/utils.hnl` | the aggregate package interface (`cap` declarations) |
| `src/math.test.hnl` | the `math` test suite |
| `atoms.yaml`, `atoms-str.yaml` | atom specs — one module per spec document (`module:`+`atoms:`); `atoms-<module>.yaml` siblings for extra modules |
| `pkg.hnl` | manifest — `modules: ["bool", "math", "str"]` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | the gate / bupha entry |

## Contract

- Spec-driven layout: `math.hnl`/`str.hnl` are documentation
  surfaces; the atom specs are `atoms.yaml`/`atoms-str.yaml`
  (P-013/T-151) and the implementations are native handls in
  `plugin-lib-stdlib` `ops::math` / `ops::strs`
  (`math_impls()`/`strs_impls()`); `utils.hnl` still declares the
  aggregate `cap` surface. Sibling native impls live in their own
  crates (e.g. `handl-lib-mood`) — there is no `plugin-lib-utils`.

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
