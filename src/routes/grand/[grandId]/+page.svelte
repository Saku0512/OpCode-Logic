<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import Editor from "$lib/components/Editor.svelte";
  import RegisterView from "$lib/components/RegisterView.svelte";
  import IOView from "$lib/components/IOView.svelte";
  import LevelSelector from "$lib/components/LevelSelector.svelte";
  import ExplanationView from "$lib/components/ExplanationView.svelte";
  import LanguageSelector from "$lib/components/LanguageSelector.svelte";
  import { getGrandStage } from "$lib/grandStages";
  import { t } from "svelte-i18n";
  import {
    completedLevelsStore,
    loadCompletedLevelsFromStorage,
    markLevelComplete,
  } from "$lib/progress";

  let syntax: "Intel" | "Att" = "Intel";
  let code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: (select a level)
    ; TODO: Write your code here.

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
  let statusKey = "status.ready";
  let messageKey = "status.select_to_begin";
  let error: string | null = null;

  // Grand stage context
  $: grandId = $page.params.grandId ?? "";
  $: grandStage = getGrandStage(grandId);
  let stageLevels: any[] = [];
  let stageMissingIds: string[] = [];

  function pickHighestUnlocked(levels: any[]) {
    if (!levels || levels.length === 0) return null;
    const completed = get(completedLevelsStore);
    let highest = levels[0];
    for (let i = 1; i < levels.length; i++) {
      const prev = levels[i - 1];
      if (completed.has(prev.id)) highest = levels[i];
      else break;
    }
    return highest;
  }

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

    // Ensure a level is actually selected (otherwise RUN can "succeed" without saving progress)
    if (!currentLevel && stageLevels.length > 0) {
      const initial = pickHighestUnlocked(stageLevels) ?? stageLevels[0];
      await handleLevelSelect(initial);
    }

    // Ensure some initial code exists
    if (!currentLevel) {
      await applyDefaultCode("01_Mov&Call");
    }
  });

  async function handleLevelSelect(level: any) {
    currentLevel = level;
    selectedLevelId = level.id;
    statusKey = "status.ready";
    messageKey = `levels.${level.id}.description`;
    error = null;
    registers = {};
    output = [];

    if (level.test_cases.length > 0) {
      input = level.test_cases[0][0];
      expected = level.test_cases[0][1];
    }

    await applyDefaultCode(level.id);
  }

  async function applyDefaultCode(levelId: string) {
    // 可能ならステージごとの ini.asm を初期コードとして読み込む
    try {
      const ini: string = await invoke("get_level_ini", { levelId, syntax });
      code = ini;
      return;
    } catch (e) {
      // fallback: 以前のテンプレ
      const level = stageLevels.find((l) => l.id === levelId) ?? currentLevel;
      const mission = $t(`levels.${levelId}.name`);
      const desc = $t(`levels.${levelId}.description`);
      code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; MISSION: ${mission}
    ${desc ? `; ${desc}` : ";"}

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall`;
    }
  }

  function resetCode() {
    if (currentLevel) {
      void applyDefaultCode(currentLevel.id);
    }
  }

  let simulationMessage: string | null = null;

  async function runSimulation() {
    if (!currentLevel) {
      statusKey = "status.failed";
      messageKey = "status.select_level";
      error = null;
      return;
    }
    statusKey = "status.executing";
    messageKey = "status.validating";
    error = null;
    output = [];

    try {
      const result: any = await invoke("run_simulation", {
        code,
        syntax,
        input,
        levelId: currentLevel.id,
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
        statusKey = "status.failed";
        simulationMessage = result.message;
      } else {
        statusKey = "status.success";
        messageKey = "status.mission_accomplished";
        simulationMessage = null;
        markLevelComplete(currentLevel.id);
      }

      if (vmState.error) {
        statusKey = "status.runtime_error";
        error = vmState.error;
      }
    } catch (e) {
      error = String(e);
      statusKey = "status.system_error";
    }
  }

  function goBack() {
    goto("/");
  }
</script>

{#if !grandStage}
  <main class="container">
    <div class="center glass">
      <div class="stage-badge">{$t("common.unknown_stage")}</div>
      <h1>{$t("common.unknown_stage_title")}</h1>
      <p class="description">{$t("common.unknown_stage_desc")}</p>
      <button class="btn-primary" on:click={goBack}
        >{$t("common.back_to_stages")}</button
      >
    </div>
  </main>
{:else if stageLevels.length === 0}
  <main class="container">
    <div class="center glass">
      <div class="stage-badge">{grandStage.badge}</div>
      <h1>{$t(`grand_stages.${grandStage.id}.title`)}</h1>
      <p class="description">
        {$t("common.stage_not_implemented")}
      </p>
      {#if stageMissingIds.length > 0}
        <p class="subtle">
          {$t("common.missing_id")}{stageMissingIds.join(", ")}
        </p>
      {/if}
      <button class="btn-primary" on:click={goBack}
        >{$t("common.back_to_stages")}</button
      >
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
            <h1>
              {currentLevel
                ? $t(`levels.${currentLevel.id}.name`)
                : $t(`grand_stages.${grandStage.id}.title`)}
            </h1>
            <p class="description">{simulationMessage || $t(messageKey)}</p>
          </div>
        </div>

        <div class="action-group">
          <div class="controls">
            <LanguageSelector />
            <button class="btn-secondary" on:click={goBack}
              >{$t("common.back_to_stages_short")}</button
            >
            <select bind:value={syntax} class="syntax-select">
              <option value="Intel">{$t("common.intel_syntax")}</option>
              <option value="Att">{$t("common.att_syntax")}</option>
            </select>
            <button class="btn-reset" on:click={resetCode}>
              {$t("common.reset")}
            </button>
            <button class="btn-run" on:click={runSimulation}>
              <span class="btn-icon">▶</span>
              {$t("common.run_verify")}
            </button>
          </div>
          <div
            class="status-indicator"
            class:success={statusKey === "status.success"}
            class:error={statusKey === "status.failed" ||
              statusKey.includes("error")}
          >
            <span class="status-dot"></span>
            <span class="status-text">{$t(statusKey)}</span>
          </div>
        </div>
      </div>

      <div class="workspace">
        {#if error}
          <div class="error-banner glass-error">
            <span class="error-label">{$t("common.exception")}</span>
            {error}
          </div>
        {/if}

        <div class="panels">
          <div class="left-panel glass">
            <div class="panel-header">
              <span class="icon">📝</span>
              {$t("common.editor")}
            </div>
            <Editor bind:code />
            {#if currentLevel}
              <ExplanationView
                levelId={currentLevel.id}
                isCompleted={$completedLevelsStore.has(currentLevel.id)}
                {syntax}
              />
            {/if}
          </div>
          <div class="right-panel">
            <div class="glass panel-inner">
              <div class="panel-header">
                <span class="icon">📊</span>
                {$t("common.registers")}
              </div>
              <RegisterView {registers} />
            </div>
            <div class="glass panel-inner">
              <div class="panel-header">
                <span class="icon">🔌</span>
                {$t("common.io_stream")}
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
