import { writable } from "svelte/store";

const STORAGE_KEY = "opcode_completed_levels";

export const completedLevelsStore = writable<Set<string>>(new Set());

export function loadCompletedLevelsFromStorage() {
  if (typeof localStorage === "undefined") return;
  const stored = localStorage.getItem(STORAGE_KEY);
  if (!stored) {
    completedLevelsStore.set(new Set());
    return;
  }
  try {
    const arr = JSON.parse(stored);
    if (Array.isArray(arr)) {
      completedLevelsStore.set(new Set(arr.map(String)));
    } else {
      completedLevelsStore.set(new Set());
    }
  } catch {
    completedLevelsStore.set(new Set());
  }
}

export function markLevelComplete(id: string) {
  completedLevelsStore.update((set) => {
    const next = new Set(set);
    next.add(id);
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(Array.from(next)));
    }
    return next;
  });
}

