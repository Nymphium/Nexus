# FIXME: Documentation vs Implementation Discrepancies

このファイルは、`docs/` の仕様記述と現在の `src/` と `tests/` の実装との間で確認された矛盾点や未実装項目を管理します。

## Implementation Gaps relative to Spec
- **Concurrency**: 実際の並行実行ランタイムが未実装です（`docs/spec/syntax.md` には順次実行である旨が注釈済み）。
