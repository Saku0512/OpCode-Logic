<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Editor from "$lib/components/Editor.svelte";
  import RegisterView from "$lib/components/RegisterView.svelte";

  // We can reuse IOView for visual feedback of "Input Arguments" and "Return Value"
  import IOView from "$lib/components/IOView.svelte";

  let syntax = "Intel";
  let code = "mov rax, rdi\nadd rax, rsi";
  let input = [10, 20];
  let expected = [30];

  // Initialize with 0s
  let registers: Record<string, number> = {};

  // For UI display, we'll strip 'output' to just show what RAX is, or keep the queue concept if we want
  // But since the new VM doesn't return an output queue, we'll map the RESULT (RAX) to output.
  let output: number[] = [];

  let status = "Ready";
  let error: string | null = null;

  function changeSyntax() {
    if (syntax === "Intel") {
      code = "mov rax, rdi\nadd rax, rsi";
    } else {
      code = "movq %rdi, %rax\naddq %rsi, %rax";
    }
  }

  async function runSimulation() {
    status = "Running...";
    error = null;
    output = [];

    try {
      const result: any = await invoke("run_simulation", {
        code,
        syntax,
        input,
      });

      // Update registers strictly from result
      registers = result.registers;

      // Determine "Output" as RAX value
      const resultVal = result.registers["RAX"] || 0;
      output = [resultVal];

      if (result.error) {
        error = result.error;
        status = "Error";
      } else {
        // Check correctness
        const isCorrect = output[0] === expected[0];
        status = isCorrect ? "SUCCESS!" : "Failed - Incorrect Output";
      }
    } catch (e) {
      error = String(e);
      status = "Error";
    }
  }
</script>

<main class="container">
  <div class="header">
    <div class="title-group">
      <h1>OpCode Logic</h1>
      <select
        bind:value={syntax}
        on:change={changeSyntax}
        class="syntax-select"
      >
        <option value="Intel">Intel Syntax</option>
        <option value="Att">AT&T Syntax</option>
      </select>
    </div>

    <div
      class="status"
      class:success={status === "SUCCESS!"}
      class:error={status === "Error" || status.startsWith("Failed")}
    >
      {status}
      {error ? `(${error})` : ""}
    </div>
    <button on:click={runSimulation}>RUN</button>
  </div>

  <div class="workspace">
    <div class="left-panel">
      <Editor bind:code />
    </div>
    <div class="right-panel">
      <RegisterView {registers} />
      <!-- We treat "Input" as the args passed, and "Output" as the return value -->
      <IOView {input} {output} {expected} />
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #1e1e1e;
    color: #fff;
    font-family: "Inter", sans-serif;
  }

  .container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 1rem;
    box-sizing: border-box;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    font-size: 1.5rem;
  }

  .syntax-select {
    background: #333;
    color: white;
    border: 1px solid #555;
    padding: 0.25rem;
    border-radius: 4px;
  }

  .status {
    font-weight: bold;
    color: #ccc;
  }

  .status.success {
    color: #4caf50;
  }

  .status.error {
    color: #f44336;
  }

  button {
    background-color: #007acc;
    color: white;
    border: none;
    padding: 0.5rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
  }

  button:hover {
    background-color: #0098ff;
  }

  .workspace {
    display: flex;
    flex: 1;
    gap: 1rem;
    min-height: 0;
  }

  .left-panel {
    flex: 1;
  }

  .right-panel {
    width: 350px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
