import { invoke } from "@tauri-apps/api/core";

// Mirrors the SessionStatus enum + Session DTO in src-tauri/src/session.rs.
// snake_case kept across the IPC boundary intentionally (see data-model.md).
interface Session {
  pid: number;
  cwd: string;
  status: "waiting" | "working" | "unknown";
  last_message: string | null;
  last_update_unix: number | null;
  waiting_since_unix: number | null;
}

const PREVIEW_MAX_LEN = 80; // single-line preview, ~50 visible chars
const POLL_INTERVAL_MS = 2000;
const BACKEND_PREVIEW_CHAR_LIMIT = 200; // matches PREVIEW_MAX_CHARS in session.rs

const listEl = document.getElementById("sessions") as HTMLUListElement;
const mainEl = document.querySelector("main") as HTMLElement;
const refreshBtn = document.getElementById("refresh") as HTMLButtonElement;

// In-memory UI state. Reset on every reload (cheap for popup-style apps).
let expandedPid: number | null = null;

// =============================================================================
// Render helpers
// =============================================================================

/**
 * Extract the last path segment from a cwd for display. Falls back to PID
 * when cwd is empty / root-only.
 */
function cwdShortName(cwd: string, pid: number): string {
  const last = cwd.split("/").filter(Boolean).pop();
  return last ?? `pid ${pid}`;
}

/**
 * Format "minutes since" for the waiting duration label. Spec § 5.2:
 *   < 60s  → "just now"
 *   1-59m  → "Nmin"
 *   ≥ 60m  → "Nh" (optional v0.2; for now keep as Nmin even if 100min)
 */
function formatDuration(unixSeconds: number): string {
  const elapsedSec = Math.floor(Date.now() / 1000 - unixSeconds);
  if (elapsedSec < 60) return "just now";
  const min = Math.floor(elapsedSec / 60);
  if (min < 60) return `${min}min`;
  const hours = Math.floor(min / 60);
  return `${hours}h`;
}

/**
 * Single-line preview: first non-empty line, capped at PREVIEW_MAX_LEN chars.
 * Used for the collapsed row 2.
 */
function singleLinePreview(message: string): string {
  const firstLine =
    message
      .split("\n")
      .map((l) => l.trim())
      .find((l) => l.length > 0) ?? "";
  if (firstLine.length <= PREVIEW_MAX_LEN) return firstLine;
  return firstLine.slice(0, PREVIEW_MAX_LEN) + "…";
}

// =============================================================================
// Empty state (S-010)
// =============================================================================

function renderEmptyState(): void {
  mainEl.innerHTML = `
    <div class="empty-state">
      <div class="empty-title">no claude sessions running</div>
      <div class="empty-hint">start a session with <code>claude</code> in your terminal</div>
    </div>
  `;
}

// =============================================================================
// Session list (S-008 + S-009)
// =============================================================================

function renderSessions(sessions: Session[]): void {
  // Replace whatever is in <main> with the <ul> (in case we came from empty).
  if (!mainEl.contains(listEl)) {
    mainEl.innerHTML = "";
    mainEl.appendChild(listEl);
  }
  listEl.innerHTML = "";

  for (const s of sessions) {
    const li = document.createElement("li");
    li.className = `session ${s.status}`;
    if (s.pid === expandedPid) li.classList.add("expanded");
    li.dataset.pid = String(s.pid);
    li.setAttribute("role", "button");
    li.setAttribute("aria-expanded", String(s.pid === expandedPid));

    const row = document.createElement("div");
    row.className = "row";

    const nameEl = document.createElement("span");
    nameEl.className = "name";
    nameEl.textContent = cwdShortName(s.cwd, s.pid);
    nameEl.title = s.cwd; // hover full path

    const sepEl = document.createElement("span");
    sepEl.className = "separator";
    sepEl.textContent = "·";

    const statusEl = document.createElement("span");
    statusEl.className = "status";
    statusEl.textContent = s.status;

    row.append(nameEl, sepEl, statusEl);

    // Duration only meaningful for `waiting`.
    if (s.status === "waiting" && s.waiting_since_unix != null) {
      const durEl = document.createElement("span");
      durEl.className = "duration";
      durEl.textContent = formatDuration(s.waiting_since_unix);
      row.append(durEl);
    }

    li.appendChild(row);

    // Row 2: single-line preview (skip if empty — keeps row tight).
    if (s.last_message && s.last_message.trim().length > 0) {
      const msgEl = document.createElement("div");
      msgEl.className = "msg";
      msgEl.textContent = singleLinePreview(s.last_message);
      li.appendChild(msgEl);
    }

    // Expanded body (S-009).
    if (s.pid === expandedPid) {
      const expandEl = document.createElement("div");
      expandEl.className = "expand";
      const content = s.last_message ?? "(no message available)";
      expandEl.textContent = content;

      // Note when backend already truncated. Honest UX > pretending we have all.
      if (content.length >= BACKEND_PREVIEW_CHAR_LIMIT) {
        const note = document.createElement("div");
        note.className = "preview-truncation-note";
        note.textContent = `preview truncated at ${BACKEND_PREVIEW_CHAR_LIMIT} chars — full transcript in your terminal`;
        expandEl.appendChild(note);
      }

      li.appendChild(expandEl);
    }

    li.addEventListener("click", () => {
      expandedPid = expandedPid === s.pid ? null : s.pid;
      // Re-render the list (cheap for ≤ 10 sessions per architecture budget).
      renderSessions(sessions);
    });

    listEl.appendChild(li);
  }
}

function render(sessions: Session[]): void {
  if (sessions.length === 0) {
    expandedPid = null; // drop any stale expand state
    renderEmptyState();
    return;
  }
  // Guard: if previously-expanded PID is no longer in the list (session ended),
  // collapse it cleanly.
  if (expandedPid != null && !sessions.some((s) => s.pid === expandedPid)) {
    expandedPid = null;
  }
  renderSessions(sessions);
}

// =============================================================================
// Polling
// =============================================================================

let refreshInFlight = false;

async function refresh(): Promise<void> {
  if (refreshInFlight) return; // collapse overlapping refreshes
  refreshInFlight = true;
  refreshBtn.textContent = "…";
  refreshBtn.disabled = true;
  try {
    const sessions = await invoke<Session[]>("list_sessions");
    render(sessions);
  } catch (e) {
    console.error("list_sessions failed", e);
  } finally {
    refreshInFlight = false;
    refreshBtn.textContent = "refresh";
    refreshBtn.disabled = false;
  }
}

refreshBtn.addEventListener("click", () => {
  void refresh();
});

void refresh();
setInterval(() => {
  void refresh();
}, POLL_INTERVAL_MS);
