<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import Editor from "$lib/components/Editor.svelte";
  import RegisterView from "$lib/components/RegisterView.svelte";
  import IOView from "$lib/components/IOView.svelte";
  import LevelSelector from "$lib/components/LevelSelector.svelte";
  import ExplanationView from "$lib/components/ExplanationView.svelte";
  import { getGrandStage } from "$lib/grandStages";
  import {
    completedLevelsStore,
    loadCompletedLevelsFromStorage,
    markLevelComplete,
  } from "$lib/progress";

  let syntax = "Intel";
  let code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall`;

  let currentLevel: any = null;
  let selectedLevelId: string | null = null;
  let input: number[] = [0];
  let output: number[] = [];
  let expected: number[] = [];
  let registers: Record<string, number> = {};
  let status = "READY";
  let message = "Select a level to begin the mission.";
  let error: string | null = null;

  // Grand stage context
  $: grandId = $page.params.grandId ?? "";
  $: grandStage = getGrandStage(grandId);
  let stageLevels: any[] = [];
  let stageMissingIds: string[] = [];

  onMount(async () => {
    loadCompletedLevelsFromStorage();

    // Load levels and filter for this grand stage
    try {
      const allLevels: any[] = await invoke("get_levels");
      if (grandStage) {
        const byId = new Map(allLevels.map((l) => [l.id, l]));
        stageLevels = grandStage.levelIds
          .map((id) => byId.get(id))
          .filter(Boolean);
        stageMissingIds = grandStage.levelIds.filter((id) => !byId.has(id));
      } else {
        stageLevels = [];
        stageMissingIds = [];
      }
    } catch (e) {
      console.error("Failed to load levels", e);
      stageLevels = [];
      stageMissingIds = [];
    }

    // Ensure some initial code exists
    if (!currentLevel) {
      applyDefaultCode("01_Mov&Call");
    }
  });

  function handleLevelSelect(level: any) {
    currentLevel = level;
    selectedLevelId = level.id;
    status = "READY";
    message = level.description;
    error = null;
    registers = {};
    output = [];

    if (level.test_cases.length > 0) {
      input = level.test_cases[0][0];
      expected = level.test_cases[0][1];
    }

    applyDefaultCode(level.id);
  }

  function applyDefaultCode(levelId: string) {
    // すべてのステージで統一された初期コードを使用（後でレベル別テンプレを追加可能）
    code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: Mov & Call
    ; read from stdin (syscall 0), write to stdout (syscall 1)
    
    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall`;
  }

  function resetCode() {
    if (currentLevel) {
      applyDefaultCode(currentLevel.id);
    }
  }

  async function runSimulation() {
    status = "EXECUTING...";
    message = "Validating logic against test vectors...";
    error = null;
    output = [];

    try {
      const result: any = await invoke("run_simulation", {
        code,
        syntax,
        input,
        levelId: currentLevel ? currentLevel.id : null,
      });

      const vmState = result.vm_state;
      registers = vmState.registers;

      if (vmState.output.length > 0) {
        output = vmState.output;
      } else {
        // sys_exit (60) で終了した場合は RAX を出力として扱わない
        if (vmState.exited && vmState.registers["RAX"] === 60) {
          output = [];
        } else {
          output = [vmState.registers["RAX"] || 0];
        }
      }

      if (!result.success) {
        status = "FAILED";
        message = result.message;
      } else {
        status = "SUCCESS";
        message = "Mission accomplished. Level cleared.";
        if (currentLevel) {
          markLevelComplete(currentLevel.id);
        }
      }

      if (vmState.error) {
        status = "RUNTIME ERROR";
        error = vmState.error;
      }
    } catch (e) {
      error = String(e);
      status = "SYSTEM ERROR";
    }
  }

  function goBack() {
    goto("/");
  }
</script>

{#if !grandStage}
  <main class="container">
    <div class="center glass">
      <div class="stage-badge">UNKNOWN STAGE</div>
      <h1>Unknown Grand Stage</h1>
      <p class="description">指定されたグランドステージが見つかりませんでした。</p>
      <button class="btn-primary" on:click={goBack}>ステージ選択へ戻る</button>
    </div>
  </main>
{:else if stageLevels.length === 0}
  <main class="container">
    <div class="center glass">
      <div class="stage-badge">{grandStage.badge}</div>
      <h1>{grandStage.title}</h1>
      <p class="description">
        このグランドステージのレベル定義はまだアプリに実装されていません。
      </p>
      {#if stageMissingIds.length > 0}
        <p class="subtle">未実装ID: {stageMissingIds.join(", ")}</p>
      {/if}
      <button class="btn-primary" on:click={goBack}>ステージ選択へ戻る</button>
    </div>
  </main>
{:else}
  <main class="container">
    <div class="sidebar">
      <LevelSelector
        bind:selectedLevelId
        completedLevels={$completedLevelsStore}
        levels={stageLevels}
        stageTitle={grandStage.badge}
        onSelect={handleLevelSelect}
      />
    </div>

    <div class="main-content">
      <div class="header glass">
        <div class="title-group">
          <div class="level-info">
            <div class="stage-badge">{grandStage.badge}</div>
            <h1>{currentLevel ? currentLevel.name : grandStage.title}</h1>
            <p class="description">{message}</p>
          </div>
        </div>

        <div class="action-group">
          <div class="controls">
            <button class="btn-secondary" on:click={goBack}>← STAGES</button>
            <select bind:value={syntax} class="syntax-select">
              <option value="Intel">INTEL SYNTAX</option>
              <option value="Att">AT&T SYNTAX</option>
            </select>
            <button class="btn-reset" on:click={resetCode}> RESET </button>
            <button class="btn-run" on:click={runSimulation}>
              <span class="btn-icon">▶</span> RUN & VERIFY
            </button>
          </div>
          <div
            class="status-indicator"
            class:success={status === "SUCCESS"}
            class:error={status === "FAILED" || status.includes("ERROR")}
          >
            <span class="status-dot"></span>
            <span class="status-text">{status}</span>
          </div>
        </div>
      </div>

      <div class="workspace">
        {#if error}
          <div class="error-banner glass-error">
            <span class="error-label">EXCEPTION:</span>
            {error}
          </div>
        {/if}

        <div class="panels">
          <div class="left-panel glass">
            <div class="panel-header">
              <span class="icon">📝</span> EDITOR
            </div>
            <Editor bind:code />
            {#if currentLevel}
              <ExplanationView
                levelId={currentLevel.id}
                isCompleted={$completedLevelsStore.has(currentLevel.id)}
              />
            {/if}
          </div>
          <div class="right-panel">
            <div class="glass panel-inner">
              <div class="panel-header">
                <span class="icon">📊</span> REGISTERS
              </div>
              <RegisterView {registers} />
            </div>
            <div class="glass panel-inner">
              <div class="panel-header">
                <span class="icon">🔌</span> I/O STREAM
              </div>
              <IOView {input} {output} {expected} />
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
{/if}

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

  .container {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .center {
    width: min(720px, 92vw);
    margin: auto;
    padding: 2rem;
    box-sizing: border-box;
  }

  .sidebar {
    width: 280px;
    flex-shrink: 0;
    background: rgba(13, 15, 23, 0.7);
    backdrop-filter: blur(20px);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    box-sizing: border-box;
    min-width: 0;
    gap: 1.25rem;
  }

  .glass {
    background: rgba(255, 255, 255, 0.02);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.5);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
  }

  .stage-badge {
    font-family: "Fira Code", monospace;
    font-size: 0.7rem;
    color: #3b82f6;
    letter-spacing: 2px;
    margin-bottom: 0.25rem;
  }

  h1 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 800;
    letter-spacing: -0.5px;
    color: #fff;
  }

  .description {
    margin: 0.5rem 0 0;
    color: #94a3b8;
    font-size: 0.95rem;
    max-width: 720px;
    line-height: 1.6;
  }

  .subtle {
    margin: 1rem 0 0;
    color: #64748b;
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
    line-height: 1.5;
    word-break: break-word;
  }

  .action-group {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 1rem;
  }

  .controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .syntax-select {
    background: rgba(0, 0, 0, 0.3);
    color: #94a3b8;
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 0.5rem 1rem;
    border-radius: 10px;
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
    cursor: pointer;
    outline: none;
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.04);
    color: #cbd5e1;
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 0.6rem 1rem;
    border-radius: 10px;
    font-weight: 700;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .btn-primary {
    margin-top: 1.25rem;
    background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
    color: #fff;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 10px;
    font-weight: 800;
    font-size: 0.9rem;
    cursor: pointer;
  }

  .btn-reset {
    background: rgba(255, 255, 255, 0.05);
    color: #94a3b8;
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 0.6rem 1.25rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-run {
    background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
    color: #fff;
    border: none;
    padding: 0.6rem 2rem;
    border-radius: 10px;
    font-weight: 700;
    font-size: 0.9rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s;
    box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(0, 0, 0, 0.2);
    padding: 0.35rem 1rem;
    border-radius: 20px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #64748b;
  }

  .status-text {
    font-family: "Fira Code", monospace;
    font-size: 0.7rem;
    font-weight: 600;
    color: #64748b;
  }

  .status-indicator.success .status-dot {
    background: #10b981;
    box-shadow: 0 0 10px #10b981;
  }

  .status-indicator.success .status-text {
    color: #10b981;
  }

  .status-indicator.error .status-dot {
    background: #ef4444;
    box-shadow: 0 0 10px #ef4444;
  }

  .status-indicator.error .status-text {
    color: #ef4444;
  }

  .workspace {
    display: flex;
    flex-direction: column;
    flex: 1;
    gap: 1rem;
    min-height: 0;
  }

  .glass-error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #fca5a5;
    padding: 1rem 1.5rem;
    border-radius: 12px;
    font-family: "Fira Code", monospace;
    font-size: 0.85rem;
  }

  .error-label {
    font-weight: 700;
    color: #ef4444;
    margin-right: 0.5rem;
  }

  .panels {
    display: flex;
    flex: 1;
    gap: 1.25rem;
    min-height: 0;
  }

  .left-panel {
    flex: 1.2;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .right-panel {
    flex: 0.8;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    min-width: 400px;
  }

  .panel-header {
    padding: 1rem 1.25rem;
    font-size: 0.7rem;
    font-weight: 700;
    color: #64748b;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .panel-inner {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>

