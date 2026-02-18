; ─── Scope definitions ──────────────────────────────────────────────────────

(function_def) @local.scope
(if_stmt) @local.scope
(match_case) @local.scope
(try_stmt) @local.scope
(conc_stmt) @local.scope
(task_def) @local.scope

; ─── Definitions ────────────────────────────────────────────────────────────

(let_stmt
  name: (identifier) @local.definition)

(param
  name: (identifier) @local.definition)

(function_def
  name: (identifier) @local.definition)

(try_stmt
  catch_param: (identifier) @local.definition)

(variable_pattern
  name: (identifier) @local.definition)

; ─── References ─────────────────────────────────────────────────────────────

(variable
  name: (identifier) @local.reference)
