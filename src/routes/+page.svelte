<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { grandStages } from "$lib/grandStages";
  import { completedLevelsStore, loadCompletedLevelsFromStorage } from "$lib/progress";

  let ready = false;

  onMount(async () => {
    loadCompletedLevelsFromStorage();
    ready = true;
  });

  function isStageCompleted(levelIds: string[]) {
    return levelIds.every((id) => $completedLevelsStore.has(id));
  }

  function isStageUnlocked(index: number) {
    if (index === 0) return true;
    const prev = grandStages[index - 1];
    return isStageCompleted(prev.levelIds);
  }

  function openStage(index: number) {
    if (!isStageUnlocked(index)) return;
    const stage = grandStages[index];
    goto(`/grand/${stage.id}`);
  }
</script>

<main class="screen">
  <div class="panel glass">
    <div class="brand">
      <div class="brand-title">OPCODE LOGIC</div>
      <div class="brand-sub">Grand Stage Select</div>
    </div>

    {#if !ready}
      <div class="loading">Initializing...</div>
    {:else}
      <div class="grid">
        {#each grandStages as stage, i}
          {@const unlocked = isStageUnlocked(i)}
          {@const completed = isStageCompleted(stage.levelIds)}
          <button
            class="card"
            class:locked={!unlocked}
            class:completed={completed}
            disabled={!unlocked}
            on:click={() => openStage(i)}
          >
            <div class="card-head">
              <div class="badge">{stage.badge}</div>
              <div class="status">
                {#if completed}
                  <span class="ok">CLEARED</span>
                {:else if !unlocked}
                  <span class="lock">LOCKED</span>
                {:else}
                  <span class="go">UNLOCKED</span>
                {/if}
              </div>
            </div>
            <div class="card-title">{stage.title}</div>
            <div class="card-desc">{stage.description}</div>
            <div class="card-foot">
              <span class="meta">{stage.levelIds.length} levels</span>
              <span class="meta">
                {#if completed}
                  ✓ 完了
                {:else}
                  {Array.from($completedLevelsStore).filter((id) => stage.levelIds.includes(id)).length}/{stage.levelIds.length}
                {/if}
              </span>
            </div>
          </button>
        {/each}
      </div>
      <div class="hint">
        グランドステージは順番に解放されます。次のグランドへ進むには、前のグランドを全てクリアしてください。
      </div>
    {/if}
  </div>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500&family=Inter:wght@400;600;700&display=swap");

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #0d0f17;
    background-image: radial-gradient(
        circle at 50% 0%,
        #1e2544 0%,
        #0d0f17 100%
      ),
      url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%239C92AC' fill-opacity='0.02'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2v-4h4v-2h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2v-4h4v-2H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    color: #e2e8f0;
    font-family: "Inter", sans-serif;
    overflow: hidden;
  }

  .screen {
    height: 100vh;
    width: 100vw;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    box-sizing: border-box;
  }

  .glass {
    background: rgba(255, 255, 255, 0.02);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.5);
  }

  .panel {
    width: min(980px, 96vw);
    padding: 2rem;
    box-sizing: border-box;
  }

  .brand {
    margin-bottom: 1.5rem;
  }

  .brand-title {
    font-weight: 900;
    font-size: 1.75rem;
    letter-spacing: -0.5px;
    color: #fff;
  }

  .brand-sub {
    margin-top: 0.35rem;
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
    letter-spacing: 2px;
    color: #3b82f6;
    text-transform: uppercase;
  }

  .loading {
    padding: 2rem 0;
    text-align: center;
    color: #94a3b8;
    font-size: 0.95rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }

  .card {
    background: rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 14px;
    padding: 1.25rem 1.25rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
    color: #e2e8f0;
  }

  .card:hover:not(:disabled) {
    transform: translateY(-2px);
    border-color: rgba(59, 130, 246, 0.35);
    box-shadow: 0 10px 30px -20px rgba(59, 130, 246, 0.7);
  }

  .card:disabled {
    cursor: not-allowed;
  }

  .card.locked {
    opacity: 0.45;
  }

  .card.completed {
    border-color: rgba(16, 185, 129, 0.3);
  }

  .card-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .badge {
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
    letter-spacing: 2px;
    color: #93c5fd;
  }

  .status span {
    font-family: "Fira Code", monospace;
    font-size: 0.7rem;
    font-weight: 800;
    letter-spacing: 1px;
  }

  .ok {
    color: #10b981;
  }
  .lock {
    color: #94a3b8;
  }
  .go {
    color: #60a5fa;
  }

  .card-title {
    font-size: 1.2rem;
    font-weight: 800;
    color: #fff;
    margin-bottom: 0.35rem;
  }

  .card-desc {
    color: #94a3b8;
    font-size: 0.95rem;
    line-height: 1.6;
    min-height: 3.2em;
  }

  .card-foot {
    margin-top: 1rem;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }

  .meta {
    color: #64748b;
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
  }

  .hint {
    margin-top: 1.25rem;
    color: #94a3b8;
    font-size: 0.9rem;
    line-height: 1.6;
  }

  @media (max-width: 840px) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
