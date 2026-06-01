import { invoke } from "@tauri-apps/api/core";

interface Session {
  pid: number;
  cwd: string;
  status: "waiting" | "working" | "unknown";
  last_message: string | null;
  last_update_unix: number | null;
  waiting_since_unix: number | null;
}

const listEl = document.getElementById("sessions") as HTMLUListElement;
const refreshBtn = document.getElementById("refresh") as HTMLButtonElement;

function render(sessions: Session[]): void {
  listEl.innerHTML = "";
  if (sessions.length === 0) {
    const li = document.createElement("li");
    li.textContent = "no claude sessions running";
    li.className = "empty";
    listEl.appendChild(li);
    return;
  }
  for (const s of sessions) {
    const li = document.createElement("li");
    li.className = `session ${s.status}`;
    const name = s.cwd.split("/").filter(Boolean).pop() ?? `pid ${s.pid}`;
    const row = document.createElement("div");
    row.className = "row";
    const nameEl = document.createElement("span");
    nameEl.className = "name";
    nameEl.textContent = name;
    const statusEl = document.createElement("span");
    statusEl.className = "status";
    statusEl.textContent = s.status;
    row.append(nameEl, statusEl);
    const msgEl = document.createElement("div");
    msgEl.className = "msg";
    msgEl.textContent = s.last_message ?? "";
    li.append(row, msgEl);
    listEl.appendChild(li);
  }
}

async function refresh(): Promise<void> {
  try {
    const sessions = await invoke<Session[]>("list_sessions");
    // S-005 smoke: emit raw payload for manual verification until S-008
    // brings the proper renderer online.
    console.log("[list_sessions]", sessions);
    render(sessions);
  } catch (e) {
    console.error("list_sessions failed", e);
  }
}

refreshBtn.addEventListener("click", () => {
  void refresh();
});

void refresh();
setInterval(() => {
  void refresh();
}, 2000);
