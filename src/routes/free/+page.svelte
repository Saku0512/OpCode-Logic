<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import Editor from "$lib/components/Editor.svelte";
  import RegisterView from "$lib/components/RegisterView.svelte";
  import IOView from "$lib/components/IOView.svelte";

  let code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; Free x86-64 Editor
    ; Write your assembly code here

    ; Example: read from stdin
    mov rax, 0
    mov rdi, 0
    mov rsi, buf
    mov rdx, 16
    syscall

    ; Example: write to stdout
    mov rdx, rax
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    syscall

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall`;

  let syntax: "Intel" | "Att" = "Intel";
  let inputStr = "0";
  let registers: Record<string, number> = {};
  let output: number[] = [];
  let status = "READY";
  let message = "Write your x86-64 assembly code and click RUN.";
  let error: string | null = null;

  function parseInput(s: string): number[] {
    if (!s.trim()) return [];
    return s
      .split(/[,\s]+/)
      .map((x) => x.trim())
      .filter((x) => x.length > 0)
      .map((x) => {
        if (x.startsWith("0x") || x.startsWith("0X")) return parseInt(x, 16);
        return parseInt(x, 10);
      })
      .filter((n) => !isNaN(n));
  }

  async function run() {
    status = "EXECUTING...";
    message = "Running assembly code...";
    error = null;
    output = [];
    registers = {};
    const inputArr = parseInput(inputStr);

    try {
      const result: any = await invoke("run_simulation", {
        code,
        syntax,
        input: inputArr,
        levelId: null,
      });
      const vm = result.vm_state;
      registers = vm.registers;

      if (vm.output.length > 0) {
        output = vm.output;
      } else {
        if (vm.exited && vm.registers["RAX"] === 60) {
          output = [];
        } else {
          output = [vm.registers["RAX"] ?? 0];
        }
      }
      status = "SUCCESS";
      message = "Code executed successfully.";
      if (vm.error) {
        status = "RUNTIME ERROR";
        error = vm.error;
      }
    } catch (e) {
      error = String(e);
      status = "SYSTEM ERROR";
      message = "Failed to execute code.";
    }
  }

  function reset() {
    code = `section .bss
    buf resb 16

section .text
    global _start

_start:
    ; Free x86-64 Editor
    ; Write your assembly code here

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall`;
    inputStr = "0";
    registers = {};
    output = [];
    status = "READY";
    message = "Write your x86-64 assembly code and click RUN.";
    error = null;
  }

  function goBack() {
    goto("/");
  }
</script>

<main class="container">
  <div class="main-content">
    <div class="header glass">
      <div class="title-group">
        <div class="level-info">
          <div class="stage-badge">FREE</div>
          <h1>Free x86-64 Editor</h1>
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
          <button class="btn-reset" on:click={reset}>RESET</button>
          <button class="btn-run" on:click={run}>
            <span class="btn-icon">▶</span> RUN
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
          <div class="input-section">
            <label for="free-input" class="input-label">Input (comma or space separated):</label>
            <input
              id="free-input"
              type="text"
              bind:value={inputStr}
              class="input-field"
              placeholder="0, 1, 2 or 0x10 0x20"
            />
          </div>
        </div>
        <div class="right-panel">
          <div class="glass panel-inner">
            <div class="panel-header">
              <span class="icon">📊</span> REGISTERS
            </div>
            <RegisterView registers={registers} />
          </div>
          <div class="glass panel-inner">
            <div class="panel-header">
              <span class="icon">🔌</span> I/O STREAM
            </div>
            <IOView input={parseInput(inputStr)} output={output} expected={[]} />
          </div>
        </div>
      </div>
    </div>
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

  .container {
    display: flex;
    height: 100vh;
    width: 100vw;
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

  .btn-icon {
    font-size: 0.9rem;
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

  .input-section {
    padding: 1rem 1.25rem;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }

  .input-label {
    display: block;
    color: #94a3b8;
    font-size: 0.8rem;
    margin-bottom: 0.5rem;
    font-weight: 600;
  }

  .input-field {
    width: 100%;
    background: rgba(0, 0, 0, 0.3);
    color: #e2e8f0;
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 0.6rem 1rem;
    border-radius: 8px;
    font-family: "Fira Code", monospace;
    font-size: 0.85rem;
    outline: none;
    box-sizing: border-box;
  }

  .input-field:focus {
    border-color: rgba(59, 130, 246, 0.5);
    background: rgba(0, 0, 0, 0.4);
  }

  @media (max-width: 1200px) {
    .panels {
      flex-direction: column;
    }

    .right-panel {
      min-width: auto;
    }
  }
</style>
