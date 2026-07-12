<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

interface TagState {
  name: string;
  present: boolean;
  last_seen: number;
}

interface FindHubState {
  key: TagState;
  wallet: TagState;
}

const state = ref<FindHubState | null>(null);
const tick = ref(0);
let unlisten: UnlistenFn | null = null;
let tickInterval: ReturnType<typeof setInterval> | null = null;

function getAgeString(lastSeen: number): string {
  void tick.value; // Keeps the string reactive to the 1s clock tick
  if (!lastSeen) return "never";
  
  const diff = Math.floor(Date.now() / 1000) - lastSeen;
  if (diff < 5) return "just now";
  if (diff < 60) return `${diff}s ago`;
  
  const mins = Math.floor(diff / 60);
  return `${mins}m ago`;
}

async function fetchState() {
  try {
    state.value = await invoke<FindHubState>("get_state");
  } catch (e) {
    console.error("Failed to fetch state:", e);
  }
}

onMounted(async () => {
  await fetchState();
  
  unlisten = await listen<FindHubState>("findhub://state", (event) => {
    state.value = event.payload;
  });

  tickInterval = setInterval(() => { tick.value++; }, 1000);
});

onUnmounted(() => {
  unlisten?.();
  if (tickInterval) clearInterval(tickInterval);
});
</script>

<template>
  <div class="findhub-container">
    <h2>📡 FindHub Status</h2>

    <div v-if="state" class="tag-list">
      <div class="tag-row" :class="{ 'is-present': state.key.present }">
        <span class="icon">🔑</span>
        <div class="info">
          <strong>{{ state.key.name }}</strong>
          <span class="age">Seen: {{ getAgeString(state.key.last_seen) }}</span>
        </div>
        <span class="status-pill">{{ state.key.present ? 'Nearby' : 'Away' }}</span>
      </div>

      <div class="tag-row" :class="{ 'is-present': state.wallet.present }">
        <span class="icon">👛</span>
        <div class="info">
          <strong>{{ state.wallet.name }}</strong>
          <span class="age">Seen: {{ getAgeString(state.wallet.last_seen) }}</span>
        </div>
        <span class="status-pill">{{ state.wallet.present ? 'Nearby' : 'Away' }}</span>
      </div>
    </div>

    <div v-else class="loading">
      Loading tags...
    </div>
  </div>
</template>

