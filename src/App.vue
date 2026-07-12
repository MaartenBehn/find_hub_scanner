<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

interface TagState {
  name: string;
  present: boolean;
  last_seen: number;   // unix seconds, 0 = never
}

interface FindHubState {
  key: TagState;
  wallet: TagState;
}

const state = ref<FindHubState | null>(null);
const scanning = ref(false);
let unlisten: UnlistenFn | null = null;
let tickInterval: ReturnType<typeof setInterval> | null = null;

// Seconds-ago counter ticks locally so the UI stays fresh between scan events
const tick = ref(0);

function fmt(seconds_ago: number | null): string {
  if (seconds_ago === null) return "never seen";
  if (seconds_ago < 5) return "just now";
  if (seconds_ago < 60) return `${seconds_ago}s ago`;
  const m = Math.floor(seconds_ago / 60);
  if (m < 60) return `${m}m ago`;
  return `${Math.floor(m / 60)}h ago`;
}

function displayAge(tag: TagState): string {
  // Add tick.value so Vue re-evaluates every second
  void tick.value;
  if (!tag.last_seen) return "never seen";
  const now = Math.floor(Date.now() / 1000);
  return fmt(now - tag.last_seen);
}

async function fetchState() {
  try {
    state.value = await invoke<FindHubState>("get_state");
  } catch (e) {
    console.error("get_state failed", e);
  }
}

onMounted(async () => {
  await fetchState();
  scanning.value = true;

  // Listen for real-time events from the Rust scanner loop
  unlisten = await listen<FindHubState>("findhub://state", (event) => {
    state.value = event.payload;
  });

  // Tick every second to update the "X seconds ago" labels
  tickInterval = setInterval(() => { tick.value++; }, 1000);
});

onUnmounted(() => {
  unlisten?.();
  if (tickInterval) clearInterval(tickInterval);
});
</script>

<template>
  <div class="app">
    <header>
      <div class="logo">📡</div>
      <h1>FindHub</h1>
      <p class="subtitle">Bluetooth tag presence</p>
    </header>

    <div class="status-bar">
      <span :class="['dot', scanning ? 'dot--scanning' : '']"></span>
      <span>{{ scanning ? "Scanning continuously…" : "Idle" }}</span>
    </div>

    <main v-if="state">
      <div class="cards">
        <TagCard :tag="state.key"   icon="🔑" :age="displayAge(state.key)" />
        <TagCard :tag="state.wallet" icon="👛" :age="displayAge(state.wallet)" />
      </div>
    </main>

    <div v-else class="loading">Waiting for first scan…</div>
  </div>
</template>

<!-- ───────── sub-component ───────── -->
<script lang="ts">
import { defineComponent, type PropType } from "vue";
interface TagState { name: string; present: boolean; last_seen: number; seconds_ago: number | null; }

export const TagCard = defineComponent({
  props: {
    tag:  { type: Object as PropType<TagState>, required: true },
    icon: { type: String, required: true },
    age:  { type: String, required: true },
  },
  template: `
    <div :class="['card', tag.present ? 'card--present' : 'card--absent']">
      <div class="card__icon">{{ icon }}</div>
      <div class="card__body">
        <div class="card__name">{{ tag.name }}</div>
        <div :class="['card__badge', tag.present ? 'badge--in' : 'badge--out']">
          {{ tag.present ? "IN RANGE" : "OUT OF RANGE" }}
        </div>
        <div class="card__age">{{ age }}</div>
      </div>
      <div class="card__indicator" :aria-label="tag.present ? 'present' : 'absent'"></div>
    </div>
  `,
});
</script>

<style>
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

:root {
  --bg:        #0d1117;
  --surface:   #161b22;
  --border:    #30363d;
  --text:      #e6edf3;
  --muted:     #8b949e;
  --present:   #3fb950;
  --absent:    #484f58;
  --accent:    #58a6ff;
  --radius:    14px;
  --font:      "Inter", "SF Pro Display", system-ui, sans-serif;
}

body {
  background: var(--bg);
  color: var(--text);
  font-family: var(--font);
  -webkit-font-smoothing: antialiased;
  min-height: 100dvh;
}

.app {
  max-width: 420px;
  margin: 0 auto;
  padding: 2rem 1.25rem 4rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.35rem;
  padding-top: 1rem;
}
.logo { font-size: 2.5rem; line-height: 1; }
h1 {
  font-size: 1.75rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text);
}
.subtitle { font-size: 0.85rem; color: var(--muted); }

/* scanning status */
.status-bar {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.8rem;
  color: var(--muted);
  padding: 0.5rem 0.75rem;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 99px;
  width: fit-content;
  align-self: center;
}
.dot {
  width: 8px; height: 8px;
  border-radius: 50%;
  background: var(--absent);
  flex-shrink: 0;
}
.dot--scanning {
  background: var(--present);
  animation: pulse 2s ease-in-out infinite;
}
@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50%       { opacity: 0.4; transform: scale(0.8); }
}

.cards { display: flex; flex-direction: column; gap: 1rem; }

/* tag card */
.card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  transition: border-color 0.3s, box-shadow 0.3s;
  position: relative;
  overflow: hidden;
}
.card--present {
  border-color: var(--present);
  box-shadow: 0 0 0 1px var(--present), 0 4px 24px rgba(63,185,80,.12);
}
.card__icon { font-size: 2rem; flex-shrink: 0; }
.card__body { flex: 1; display: flex; flex-direction: column; gap: 0.3rem; }
.card__name { font-size: 1.1rem; font-weight: 600; }
.card__age  { font-size: 0.75rem; color: var(--muted); }

.card__badge {
  display: inline-block;
  font-size: 0.65rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  padding: 0.2em 0.6em;
  border-radius: 99px;
  width: fit-content;
}
.badge--in  { background: rgba(63,185,80,.18); color: var(--present); }
.badge--out { background: rgba(72,79,88,.3);   color: var(--muted); }

.card__indicator {
  width: 10px; height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
  background: var(--absent);
  transition: background 0.3s;
}
.card--present .card__indicator { background: var(--present); }

.loading {
  text-align: center;
  color: var(--muted);
  font-size: 0.9rem;
  padding: 3rem 0;
}
</style>
