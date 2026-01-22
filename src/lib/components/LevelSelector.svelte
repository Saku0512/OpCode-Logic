<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    export let selectedLevelId: string | null = null;
    export let completedLevels: Set<string> = new Set();
    export let onSelect: (level: any) => void;

    let levels: any[] = [];
    let loading = true;

    // Helper to check if level is unlocked
    // We treat "index" as order. Level[0] is always unlocked.
    // Level[i] is unlocked if Level[i-1] is in completedLevels.
    function isUnlocked(index: number) {
        if (index === 0) return true;
        if (levels.length === 0) return false;
        const prevLevel = levels[index - 1];
        return completedLevels.has(prevLevel.id);
    }

    onMount(async () => {
        try {
            levels = await invoke("get_levels");
            if (!selectedLevelId && levels.length > 0) {
                // Find highest unlocked level
                let highest = levels[0];
                for (let i = 1; i < levels.length; i++) {
                    if (isUnlocked(i)) highest = levels[i];
                    else break;
                }
                selectLevel(highest);
            }
        } catch (e) {
            console.error("Failed to load levels", e);
        } finally {
            loading = false;
        }
    });

    function selectLevel(level: any) {
        selectedLevelId = level.id;
        onSelect(level);
    }
</script>

<div class="level-selector">
    <div class="stage-title">GRAND STAGE 1</div>
    {#if loading}
        <div class="loading">Initializing...</div>
    {:else}
        <div class="list">
            {#each levels as level, i}
                {@const unlocked = isUnlocked(i)}
                {@const isCompleted = completedLevels.has(level.id)}
                <button
                    class="level-btn"
                    class:active={selectedLevelId === level.id}
                    class:completed={isCompleted}
                    class:locked={!unlocked}
                    disabled={!unlocked}
                    on:click={() => {
                        if (unlocked) {
                            selectLevel(level);
                        } else {
                            console.log(`Level ${level.id} is locked. Previous level must be completed first.`);
                        }
                    }}
                >
                    <span class="status-marker">
                        {#if isCompleted}
                            <span class="check">✓</span>
                        {:else if !unlocked}
                            <span class="lock">🔒</span>
                        {:else}
                            <span class="dot">•</span>
                        {/if}
                    </span>
                    <div class="level-meta">
                        <span class="level-id"
                            >STG_{String(i + 1).padStart(2, "0")}</span
                        >
                        <span class="level-name text-truncate"
                            >{level.name}</span
                        >
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .level-selector {
        height: 100%;
        display: flex;
        flex-direction: column;
        background: transparent;
    }

    .stage-title {
        padding: 1.5rem 1.25rem 1rem;
        font-size: 0.75rem;
        font-weight: 700;
        letter-spacing: 2px;
        color: #64748b;
        text-transform: uppercase;
    }

    .list {
        flex: 1;
        overflow-y: auto;
        padding: 0 0.75rem;
    }

    .level-btn {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        width: 100%;
        padding: 0.75rem 0.75rem;
        margin-bottom: 0.25rem;
        background: transparent;
        border: 1px solid transparent;
        border-radius: 8px;
        color: #94a3b8;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        text-align: left;
    }

    .level-btn:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.05);
        color: #e2e8f0;
    }

    .level-btn.active {
        background: rgba(86, 156, 214, 0.15);
        border: 1px solid rgba(86, 156, 214, 0.3);
        color: #fff;
    }

    .level-btn.locked {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .status-marker {
        width: 24px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 0.9rem;
    }

    .check {
        color: #4ec9b0;
    }
    .lock {
        font-size: 0.7rem;
    }
    .dot {
        color: #569cd6;
        font-size: 1.5rem;
    }

    .level-meta {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-width: 0;
    }

    .level-id {
        font-family: "Fira Code", monospace;
        font-size: 0.65rem;
        opacity: 0.6;
    }

    .level-name {
        font-size: 0.85rem;
        font-weight: 500;
    }

    .text-truncate {
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .loading {
        padding: 2rem;
        text-align: center;
        color: #64748b;
        font-size: 0.9rem;
    }
</style>
