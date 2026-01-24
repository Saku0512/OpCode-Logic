<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  export let levelId: string | null = null;
  export let isCompleted: boolean = false;

  let explanation: string | null = null;
  let collectCode: string | null = null;
  let loading = false;
  let loadingCollect = false;
  let error: string | null = null;
  let errorCollect: string | null = null;
  let showExplanation = false;
  let showSolution = false;

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

  async function loadCollect() {
    if (!levelId || !isCompleted) return;
    loadingCollect = true;
    errorCollect = null;
    try {
      collectCode = await invoke("get_level_collect", { levelId });
    } catch (e) {
      errorCollect = String(e);
      console.error("Failed to load collect.asm:", e);
    } finally {
      loadingCollect = false;
    }
  }

  function toggleSolution() {
    if (!showSolution && !collectCode) {
      loadCollect();
    }
    showSolution = !showSolution;
  }

  // Markdownを簡単にレンダリング（基本的な処理）
  function formatMarkdown(text: string): string {
    let formatted = text;
    
    // コードブロックを処理（```で囲まれた部分）
    formatted = formatted.replace(/```(\w+)?\n([\s\S]*?)```/g, (match, lang, code) => {
      return `<pre class="code-block"><code>${code.trim()}</code></pre>`;
    });
    
    // 見出しを処理
    formatted = formatted.replace(/^### (.*$)/gim, '<h3>$1</h3>');
    formatted = formatted.replace(/^## (.*$)/gim, '<h2>$1</h2>');
    formatted = formatted.replace(/^# (.*$)/gim, '<h1>$1</h1>');
    
    // 太字を処理
    formatted = formatted.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
    
    // 段落を処理（空行で区切る）
    const paragraphs = formatted.split(/\n\n+/);
    formatted = paragraphs.map(p => {
      p = p.trim();
      if (!p || p.startsWith('<')) return p; // 既にHTMLタグがある場合はそのまま
      return `<p>${p.replace(/\n/g, '<br>')}</p>`;
    }).join('');
    
    return formatted;
  }
</script>

{#if isCompleted}
  <div class="explanation-container">
    <div class="toggle-row">
      <button class="explanation-toggle" on:click={toggleExplanation}>
        <span class="icon">📖</span>
        <span>{showExplanation ? "解説を閉じる" : "解説を読む"}</span>
      </button>
      <button class="explanation-toggle" on:click={toggleSolution}>
        <span class="icon">✅</span>
        <span>{showSolution ? "模範解答を閉じる" : "模範解答を見る"}</span>
      </button>
    </div>

    {#if showExplanation}
      <div class="explanation-panel glass">
        {#if loading}
          <div class="loading">解説を読み込んでいます...</div>
        {:else if error}
          <div class="error">解説の読み込みに失敗しました: {error}</div>
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
          <div class="loading">模範解答を読み込んでいます...</div>
        {:else if errorCollect}
          <div class="error">模範解答の読み込みに失敗しました: {errorCollect}</div>
        {:else if collectCode}
          <div class="explanation-content">
            <h2>模範解答（collect.asm）</h2>
            <pre class="code-block"><code>{collectCode.trim()}</code></pre>
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
