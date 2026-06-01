# Tasks — 001-mvp

> **30 ordered MVP tasks (TDD + dependencies + [P] parallel marks)** — originally from Spec Kit `/speckit.tasks`, mv to BMAD by [ADR-013](../02-planning/decision-log.md)
> **Status:** Draft → ready for /speckit.implement (i.e., 开始干活)
> **Based on:** [PRD.md](../02-planning/PRD.md) + [architecture.md](architecture.md)
> **Created:** 2026-05-18
>
> Spec Kit principle: tasks ordered to respect dependencies. `[P]` 标记 = 可并行执行 (no shared file conflict)。每个 task 引用具体文件路径。TDD：test 先于 implementation。
>
> 任务编号 `T001-T0NN`。跟 [BMAD dev story](../../bmad/03-solutioning/epics/) 的 `S-001` ~ `S-013` cross-ref。

---

## 0. Pre-tasks (BLOCKING)

这些**必须先完成**，否则 task 1+ 无法开始。

### T000 · Resolve open questions

- **T000.1** Manually `cat ~/.claude/projects/<sample>/*.jsonl` 确认 JSONL schema → 写到 `docs/spec/jsonl-schema.md` (待写)
  - Verifies: OQ-1, OQ-2 from [plan § 11](architecture.md)
  - Blocks: T002, T004
- **T000.2** Manually test Gatekeeper on macOS 12 / 14 / 15 → 写到 `docs/guides/install.md` (待写)
  - Verifies: OQ-3
  - Blocks: T030 (release prep)
  - **作者待办**

---

## 1. Phase 1: Discovery + Classification (backend foundation)

### Story: F-D (Discovery) + F-C (Classification)

### Setup tasks

- **T001** Add Rust dependencies to `src-tauri/Cargo.toml`: `sysinfo = "0.30"`, `serde = "1"`, `serde_json = "1"`, `log = "0.4"`, `flexi_logger = "0.27"`, `dirs = "5"`
  - Files: `src-tauri/Cargo.toml`
  - DoD: `cargo build` succeeds

### Test tasks (TDD - write first, fail, then implement)

- **T002** [P] Write unit tests for `tail_jsonl(path) -> Result<Option<String>, io::Error>`
  - Files: `src-tauri/tests/jsonl_tail.rs`
  - 5+ test cases (empty file, single line w/o `\n`, single line w/ `\n`, multi-line, 100MB synthetic)
  - DoD: Tests written, all fail (no impl yet)
  - Cross-ref: [BMAD S-003](../../bmad/03-solutioning/epics/story-003-jsonl-tail-reader.md)

- **T003** [P] Write unit tests for `classify(jsonl_last: Option<JsonlMessage>) -> SessionStatus`
  - Files: `src-tauri/tests/classify.rs`
  - All branches: None → Unknown; assistant only text → Waiting; assistant with tool_use → Working; user → Working; tool_result → Working
  - DoD: 5+ tests written and fail
  - Cross-ref: [BMAD S-004](../../bmad/03-solutioning/epics/story-004-status-classifier.md)

- **T004** [P] Write unit tests for `locate_jsonl(proc: &RawProcess) -> Result<PathBuf, LocateError>`
  - Files: `src-tauri/tests/jsonl_locator.rs`
  - Use `tempfile` crate to mock `~/.claude/projects/<encoded>/`
  - 5+ cases: happy / multi-jsonl / no jsonl / no dir / all stale
  - DoD: Tests written and fail
  - Cross-ref: [BMAD S-002](../../bmad/03-solutioning/epics/story-002-jsonl-locator.md)
  - Depends on: T000.1 (need to know real encoded-path rule)

- **T005** [P] Write unit tests for `list_processes() -> Vec<RawProcess>`
  - Files: `src-tauri/tests/process_enum.rs`
  - Cases: mock sysinfo with mix of claude/non-claude, different UIDs
  - DoD: Tests written
  - Cross-ref: [BMAD S-001](../../bmad/03-solutioning/epics/story-001-process-enumeration.md)

### Implementation tasks

- **T006** Implement `list_processes() -> Vec<RawProcess>` in `src-tauri/src/session.rs`
  - Use sysinfo `System::refresh_processes()`
  - Filter name == "claude" or cmd[0] basename == "claude"
  - Skip UID mismatch
  - Run T005 → tests pass
  - DoD: tests green
  - Cross-ref: [S-001](../../bmad/03-solutioning/epics/story-001-process-enumeration.md)

- **T007** Implement `tail_jsonl(path)` in `src-tauri/src/session.rs`
  - Seek to EOF + reverse scan algorithm (per [research § 4](research-notes.md))
  - Run T002 → tests pass
  - DoD: tests green, 100MB benchmark < 10ms
  - Cross-ref: [S-003](../../bmad/03-solutioning/epics/story-003-jsonl-tail-reader.md)

- **T008** Implement `locate_jsonl(proc)` in `src-tauri/src/session.rs`
  - Uses `dirs::home_dir()` → `~/.claude/projects/<encoded>/`
  - Encoded rule per T000.1 result (`/` → `-` predicted)
  - mtime desc + 60s active threshold
  - Run T004 → tests pass
  - DoD: tests green
  - Cross-ref: [S-002](../../bmad/03-solutioning/epics/story-002-jsonl-locator.md)
  - Depends on: T000.1, T006

- **T009** Implement `JsonlMessage` / `Role` / `ContentBlock` types + `classify(Option<JsonlMessage>) -> SessionStatus`
  - Files: `src-tauri/src/session.rs`
  - Types per [data-model.md § 2.2](data-model.md)
  - `has_pending_tool_use` helper
  - Run T003 → tests pass
  - DoD: tests green, all branches covered
  - Cross-ref: [S-004](../../bmad/03-solutioning/epics/story-004-status-classifier.md)

- **T010** Implement `Session` struct + `list() -> Vec<Session>` orchestrator + sort
  - Files: `src-tauri/src/session.rs`
  - Calls T006 → T008 → T007 → T009 in pipeline
  - Per-session try with catch_unwind (impl in T015 polish)
  - Sort: waiting first by waiting_since asc
  - Cross-ref: [S-005 partial](../../bmad/03-solutioning/epics/story-005-list-sessions-command.md)

- **T011** Implement `list_sessions` Tauri command in `src-tauri/src/lib.rs`
  - Files: `src-tauri/src/lib.rs`
  - `#[tauri::command] fn list_sessions(app: AppHandle) -> Vec<Session>`
  - Tail of fn: count waiting, `app.tray_by_id("main").unwrap().set_title(Some(text))`
  - Register in `invoke_handler(generate_handler![list_sessions])`
  - Cross-ref: [S-005](../../bmad/03-solutioning/epics/story-005-list-sessions-command.md)
  - Depends on: T010

### Checkpoint 1

- Run `cargo test` → all green
- `cargo tauri dev` → console can call `invoke("list_sessions")` and print result
- 3 real claude sessions → backend returns 3 Session entries with correct status
- Tray title shows correct waiting count

---

## 2. Phase 2: Tray UI

### T012 · Tray icon + Quit menu

- Files: `src-tauri/src/lib.rs` (review + finalize scaffold)
- Already partially in scaffold; verify:
  - `TrayIconBuilder::with_id("main").icon(...).icon_as_template(true).menu(quit_menu).menu_on_left_click(false)`
  - Register `on_menu_event` handling "quit" → `app.exit(0)`
  - macOS: `set_activation_policy(ActivationPolicy::Accessory)`
- Manual test: tray icon visible, right-click → Quit, click Quit → app exits
- Cross-ref: [S-006](../../bmad/03-solutioning/epics/story-006-tray-icon-menu.md)

### T013 · Popup window create + show/hide

- Files: `src-tauri/src/lib.rs` (`on_tray_icon_event`)
- `tauri.conf.json` window config (already in scaffold)
- Match `TrayIconEvent::Click { button: Left, button_state: Up, .. }`
  - if `window.is_visible()?` → `hide()`
  - else `show()? + set_focus()?`
- Manual test: click toggle works, <200ms
- Cross-ref: [S-007](../../bmad/03-solutioning/epics/story-007-popup-window.md)

---

## 3. Phase 3: Frontend UI

### T014 · Session list render

- Files: `src/main.ts` (rewrite), `src/style.css` (rewrite)
- Per [ux-design § 5](../../bmad/02-planning/ux-design.md):
  - Header (title + refresh button)
  - List item: cwd name / status badge / duration / message preview
  - Sort displayed as backend returns (don't re-sort)
  - Scroll after 6 items
- `setInterval(refresh, 2000)` triggering `invoke<Session[]>("list_sessions")`
- Cross-ref: [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md)

### T015 · Expand/collapse message

- Files: `src/main.ts`
- State: `let expandedPid: number | null = null`
- Click on row → toggle expandedPid
- Expanded view: full last_message in SF Mono 12pt
- tool_use rendered as `[ToolName] short args`
- Close popup → reset `expandedPid = null`
- Cross-ref: [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md)

### T016 · Empty state

- Files: `src/main.ts`, `src/style.css`
- `if (sessions.length === 0)` render empty markup
- "no claude sessions running" + hint "start a session with `claude` in your terminal"
- Vertical + horizontal center
- Cross-ref: [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md)

### Checkpoint 2

- `cargo tauri dev` → 3 session 真实场景 dogfood
- light + dark mode 都 OK
- 滚动 > 6 项正常

---

## 4. Phase 4: Robustness

### T017 [P] · Logging init

- Files: `src-tauri/src/lib.rs` (early in `run()`)
- Use `flexi_logger`, path `~/Library/Logs/com.caiyiwen.claude-code-monitor/main.log`
- Format: `{ISO8601} [{LEVEL}] {msg}`
- Default level: INFO; `RUST_LOG=debug` → DEBUG
- silent fail on dir create
- Cross-ref: [S-012](../../bmad/03-solutioning/epics/story-012-logging.md)

### T018 · Per-session panic isolation

- Files: `src-tauri/src/session.rs` (`list()` function)
- Wrap per-session processing in `std::panic::catch_unwind`
- On Err: log + return `Session::unknown(pid, cwd)`
- Add `Session::unknown(...)` constructor
- Cross-ref: [S-011 part 1](../../bmad/03-solutioning/epics/story-011-error-handling.md)

### T019 · JSONL parse fallback

- Files: `src-tauri/src/session.rs` (`process_one_session()`)
- All `Result::Err` paths → fall through to `Session::unknown(pid, cwd)` + log
- `last_message = Some("(unable to read transcript)")`
- Cross-ref: [S-011 part 2](../../bmad/03-solutioning/epics/story-011-error-handling.md), [S-013](../../bmad/03-solutioning/epics/story-013-startup-race.md)

### Checkpoint 3

- Corrupt 1 JSONL → that session shows Unknown, others normal
- Inject panic in `process_one_session` → app doesn't crash, that session Unknown
- log file written correctly

---

## 5. Phase 5: Performance benchmarks

### T020 [P] · Benchmark: list_sessions

- Files: `src-tauri/benches/perf.rs` (new), update `Cargo.toml` `[[bench]]`
- Use criterion crate
- Cases: 5 / 10 / 15 / 25 fake sessions
- Asserts: 10 sessions < 50ms, 15 sessions < 100ms

### T021 [P] · Benchmark: tail_jsonl

- Files: same as T020
- Cases: 1KB / 1MB / 100MB synthetic JSONL
- Asserts: 100MB < 10ms

### T022 · CI: bench in workflow

- Files: `.github/workflows/ci.yml` (new)
- Run `cargo bench` (or selected benchmarks) on each PR
- Fail PR if performance regression > 20%

### T023 · CI: forbidden pattern grep

- Files: `.github/workflows/lint.yml` (new)
- Grep patterns per [project-context § 9](../../bmad/03-solutioning/project-context.md) / [constitution § II.5](../../constitution.md)
- Fail on match: notification / network / fs watcher / osascript / framework imports

---

## 6. Phase 6: Manual UAT (dogfood)

### T024 · Run quickstart acceptance tests

- Follow [quickstart.md § 2](quickstart.md) acceptance plan
- Check off each test
- File issues for any failure

### T025 · 14-day continuous dogfood

- Daily check (per quickstart § 3)
- End-of-period retrospective.md

---

## 7. Phase 7: Release prep (post-MVP works as scope)

Not part of dev tasks per se but enumerate:

### T026 · Write `guides/install.md`

- Cover DMG / Homebrew / source build
- Gatekeeper bypass with screenshots (depends on T000.2)
- Cross-ref: [F4](../../product/user-stories.md#f4--gatekeeper-拦截)

### T027 · Write `spec/jsonl-schema.md`

- Document actual Claude Code JSONL format
- Lock our dependencies on specific fields
- Note version compatibility
- Depends on: T000.1

### T028 · Update main `README.md`

- Quick start
- Screenshots
- Link to install.md / spec
- Status: alpha → beta → v1.0 sections

### T029 · Build & sign DMG (no sign in MVP)

- `cargo tauri build --target universal-apple-darwin`
- Bundle to DMG
- Test install on clean Mac (or VM)
- Verify DMG < 15MB (NFR-D1)

### T030 · GitHub release

- Tag `v0.1.0`
- Upload DMG to release page
- Release notes (link to user-stories / scenarios for"what it does")
- Depends on: T000.2

---

## 8. Task dependency graph

```
T000.1 → T002, T004, T008, T009, T027
T000.2 → T026, T030
T001 → T002...T011

T002, T003, T004, T005 (parallel, can do together)
T006 (after T005)
T007 (after T002)
T008 (after T004, T006)
T009 (after T003)
T010 (after T006, T007, T008, T009)
T011 (after T010)
[Checkpoint 1]

T012, T013 (after T011, parallel)
T014, T015, T016 (after T013, parallel)
[Checkpoint 2]

T017, T018, T019 (parallel, can do together)
[Checkpoint 3]

T020, T021 (parallel)
T022, T023 (after T020/T021)

T024 (after Checkpoint 3 + T022/T023)
T025 (after T024)

T026, T027, T028 (parallel, after Checkpoint 3)
T029 (after T028)
T030 (after T026, T029)
```

---

## 9. Story → task mapping

| Story | Tasks |
|---|---|
| [S-001](../../bmad/03-solutioning/epics/story-001-process-enumeration.md) | T005, T006 |
| [S-002](../../bmad/03-solutioning/epics/story-002-jsonl-locator.md) | T004, T008 |
| [S-003](../../bmad/03-solutioning/epics/story-003-jsonl-tail-reader.md) | T002, T007 |
| [S-004](../../bmad/03-solutioning/epics/story-004-status-classifier.md) | T003, T009 |
| [S-005](../../bmad/03-solutioning/epics/story-005-list-sessions-command.md) | T010, T011 |
| [S-006](../../bmad/03-solutioning/epics/story-006-tray-icon-menu.md) | T012 |
| [S-007](../../bmad/03-solutioning/epics/story-007-popup-window.md) | T013 |
| [S-008](../../bmad/03-solutioning/epics/story-008-session-list-render.md) | T014 |
| [S-009](../../bmad/03-solutioning/epics/story-009-expand-message.md) | T015 |
| [S-010](../../bmad/03-solutioning/epics/story-010-empty-state.md) | T016 |
| [S-011](../../bmad/03-solutioning/epics/story-011-error-handling.md) | T018, T019 |
| [S-012](../../bmad/03-solutioning/epics/story-012-logging.md) | T017 |
| [S-013](../../bmad/03-solutioning/epics/story-013-startup-race.md) | T019 |

---

## 10. Estimated timeline

| Phase | Tasks | Estimate (single dev) |
|---|---|---|
| Pre | T000 | 1 day (manual investigation) |
| Phase 1 | T001-T011 | 8-10 days |
| Phase 2 | T012-T013 | 2 days |
| Phase 3 | T014-T016 | 4-5 days |
| Phase 4 | T017-T019 | 2-3 days |
| Phase 5 | T020-T023 | 2 days |
| Phase 6 | T024-T025 | 14 days (calendar, dogfood not full-time) |
| Phase 7 | T026-T030 | 3-5 days |
| **MVP total** | T001-T025 | ~4 calendar weeks of focused work + 14d dogfood |
| **Release** | + T026-T030 | +1 week |

---

## 11. Cross-reference

| Topic | Where |
|---|---|
| Constitution | [memory/constitution.md](../../constitution.md) |
| Spec | [PRD.md](../02-planning/PRD.md) |
| Plan | [architecture.md](architecture.md) |
| Research | [research-notes.md](research-notes.md) |
| Data model | [data-model.md](data-model.md) |
| IPC contract | [../../spec/ipc-contract.md](../../spec/ipc-contract.md) |
| Quickstart | [quickstart.md](quickstart.md) |
| BMAD epics | [bmad/03-solutioning/epics/README.md](../../bmad/03-solutioning/epics/README.md) |
| BMAD stories | [bmad/03-solutioning/epics/](../../bmad/03-solutioning/epics/) |
| User stories | [product/user-stories.md](../../product/user-stories.md) |
