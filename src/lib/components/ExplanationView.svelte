<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  export let levelId: string | null = null;
  export let isCompleted: boolean = false;
  export let syntax: "Intel" | "Att" = "Intel";

  let explanation: string | null = null;
  let collectIntel: string | null = null;
  let collectAtt: string | null = null;
  let loading = false;
  let loadingCollect = false;
  let error: string | null = null;
  let errorCollect: string | null = null;
  let showExplanation = false;
  let showSolution = false;
  let solutionSyntax: "Intel" | "Att" = "Intel";
  let lastLevelId: string | null = null;

  $: currentCollect = solutionSyntax === "Att" ? collectAtt : collectIntel;

  // レベルが切り替わったらキャッシュや表示状態をクリア（前のレベルの内容が残るのを防ぐ）
  $: if (levelId !== lastLevelId) {
    lastLevelId = levelId;
    explanation = null;
    collectIntel = null;
    collectAtt = null;
    error = null;
    errorCollect = null;
    loading = false;
    loadingCollect = false;
    showExplanation = false;
    showSolution = false;
    solutionSyntax = syntax;
  }

  async function loadExplanation() {
    if (!levelId || !isCompleted) return;

    loading = true;
    error = null;
    try {
      explanation = await invoke("get_level_explanation", { levelId });
    } catch (e) {
      error = String(e);
      console.error("Failed to load explanation:", e);
    } finally {
      loading = false;
    }
  }

  function toggleExplanation() {
    if (!showExplanation && !explanation) {
      loadExplanation();
    }
    showExplanation = !showExplanation;
  }

  async function loadCollect(targetSyntax: "Intel" | "Att") {
    if (!levelId || !isCompleted) return;
    loadingCollect = true;
    errorCollect = null;
    try {
      const code: string = await invoke("get_level_collect", {
        levelId,
        syntax: targetSyntax,
      });
      if (targetSyntax === "Att") collectAtt = code;
      else collectIntel = code;
    } catch (e) {
      errorCollect = String(e);
      console.error("Failed to load collect.asm:", e);
    } finally {
      loadingCollect = false;
    }
  }

  function toggleSolution() {
    if (!showSolution) {
      solutionSyntax = syntax;
      if (!currentCollect) {
        loadCollect(solutionSyntax);
      }
    }
    showSolution = !showSolution;
  }

  function onChangeSolutionSyntax(next: "Intel" | "Att") {
    solutionSyntax = next;
    if (!currentCollect) {
      loadCollect(solutionSyntax);
    }
  }

  // Markdownを簡単にレンダリング（基本的な処理）
  function formatMarkdown(text: string): string {
    let formatted = text;

    // コードブロックを処理（```で囲まれた部分）
    formatted = formatted.replace(
      /```(\w+)?\n([\s\S]*?)```/g,
      (match, lang, code) => {
        return `<pre class="code-block"><code>${code.trim()}</code></pre>`;
      },
    );

    // 見出しを処理
    formatted = formatted.replace(/^### (.*$)/gim, "<h3>$1</h3>");
    formatted = formatted.replace(/^## (.*$)/gim, "<h2>$1</h2>");
    formatted = formatted.replace(/^# (.*$)/gim, "<h1>$1</h1>");

    // 太字を処理
    formatted = formatted.replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>");

    // 段落を処理（空行で区切る）
    const paragraphs = formatted.split(/\n\n+/);
    formatted = paragraphs
      .map((p) => {
        p = p.trim();
        if (!p || p.startsWith("<")) return p; // 既にHTMLタグがある場合はそのまま
        return `<p>${p.replace(/\n/g, "<br>")}</p>`;
      })
      .join("");

    return formatted;
  }

  import { t } from "svelte-i18n";
</script>

{#if isCompleted}
  <div class="explanation-container">
    <div class="toggle-row">
      <button class="explanation-toggle" on:click={toggleExplanation}>
        <span class="icon">📖</span>
        <span
          >{showExplanation
            ? $t("explanation.close")
            : $t("explanation.read")}</span
        >
      </button>
      <button class="explanation-toggle" on:click={toggleSolution}>
        <span class="icon">✅</span>
        <span
          >{showSolution
            ? $t("explanation.close_solution")
            : $t("explanation.view_solution")}</span
        >
      </button>
    </div>

    {#if showExplanation}
      <div class="explanation-panel glass">
        {#if loading}
          <div class="loading">{$t("explanation.loading")}</div>
        {:else if error}
          <div class="error">
            {$t("explanation.error", { values: { error } })}
          </div>
        {:else if explanation}
          <div class="explanation-content">
            {@html formatMarkdown(explanation)}
          </div>
        {/if}
      </div>
    {/if}

    {#if showSolution}
      <div class="explanation-panel glass">
        {#if loadingCollect}
          <div class="loading">{$t("explanation.loading_solution")}</div>
        {:else if errorCollect}
          <div class="error">
            {$t("explanation.error_solution", {
              values: { error: errorCollect },
            })}
          </div>
        {:else if currentCollect}
          <div class="explanation-content">
            <div class="solution-head">
              <h2>{$t("explanation.solution_title")}</h2>
              <select
                class="syntax-select"
                bind:value={solutionSyntax}
                on:change={(e) =>
                  onChangeSolutionSyntax(
                    (e.currentTarget as HTMLSelectElement).value as any,
                  )}
              >
                <option value="Intel">Intel</option>
                <option value="Att">AT&amp;T</option>
              </select>
            </div>
            <pre class="code-block"><code>{currentCollect.trim()}</code></pre>
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .explanation-container {
    margin-top: 1rem;
  }

  .toggle-row {
    display: flex;
    gap: 0.75rem;
  }

  .solution-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .syntax-select {
    background: rgba(0, 0, 0, 0.25);
    color: #94a3b8;
    border: 1px solid rgba(255, 255, 255, 0.12);
    padding: 0.45rem 0.75rem;
    border-radius: 10px;
    font-family: "Fira Code", monospace;
    font-size: 0.75rem;
    cursor: pointer;
    outline: none;
  }

  .explanation-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    color: #60a5fa;
    padding: 0.75rem 1.25rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
    width: 100%;
    justify-content: center;
  }

  .explanation-toggle:hover {
    background: rgba(59, 130, 246, 0.2);
    border-color: rgba(59, 130, 246, 0.5);
    color: #93c5fd;
  }

  .explanation-panel {
    margin-top: 1rem;
    padding: 1.5rem;
    max-height: 500px;
    overflow-y: auto;
  }

  .explanation-content {
    color: #e2e8f0;
    line-height: 1.8;
    font-size: 0.95rem;
  }

  .explanation-content :global(h1) {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 1.5rem 0 1rem;
    color: #fff;
    border-bottom: 2px solid rgba(59, 130, 246, 0.3);
    padding-bottom: 0.5rem;
  }

  .explanation-content :global(h2) {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 1.25rem 0 0.75rem;
    color: #cbd5e1;
  }

  .explanation-content :global(h3) {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 1rem 0 0.5rem;
    color: #94a3b8;
  }

  .explanation-content :global(p) {
    margin: 0.75rem 0;
  }

  .explanation-content :global(strong) {
    color: #60a5fa;
    font-weight: 600;
  }

  .explanation-content :global(.code-block) {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    overflow-x: auto;
    font-family: "Fira Code", monospace;
    font-size: 0.85rem;
    line-height: 1.6;
  }

  .explanation-content :global(.code-block code) {
    color: #4ec9b0;
    white-space: pre;
  }

  .loading {
    text-align: center;
    color: #94a3b8;
    padding: 2rem;
  }

  .error {
    color: #fca5a5;
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
  }

  .icon {
    font-size: 1.1rem;
  }
</style>
