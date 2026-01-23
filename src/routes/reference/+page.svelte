<script lang="ts">
  import RegisterReference from "$lib/components/RegisterReference.svelte";
  import SyscallReference from "$lib/components/SyscallReference.svelte";

  let activeTab: "registers" | "syscalls" = "registers";
</script>

<main class="reference-container">
  <div class="nav-bar">
    <a href="/" class="back-btn">
      <span class="back-icon">←</span> Back to Levels
    </a>
  </div>

  <div class="reference-header glass">
    <h1>x86_64 REFERENCE</h1>
    <p class="subtitle">Assembly Language Architecture Guide</p>
  </div>

  <div class="tabs">
    <button
      class="tab-btn"
      class:active={activeTab === "registers"}
      on:click={() => (activeTab = "registers")}
    >
      <span class="tab-icon">📋</span> REGISTERS
    </button>
    <button
      class="tab-btn"
      class:active={activeTab === "syscalls"}
      on:click={() => (activeTab = "syscalls")}
    >
      <span class="tab-icon">🔌</span> SYSCALLS
    </button>
  </div>

  <div class="content glass">
    {#if activeTab === "registers"}
      <RegisterReference />
    {:else if activeTab === "syscalls"}
      <SyscallReference />
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
  }

  .reference-container {
    height: 100vh;
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .nav-bar {
    margin-bottom: 2rem;
  }

  .back-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    color: #94a3b8;
    text-decoration: none;
    font-weight: 600;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
    color: #e2e8f0;
  }

  .back-icon {
    font-size: 1.2rem;
  }

  .glass {
    background: rgba(255, 255, 255, 0.02);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.5);
  }

  .reference-header {
    padding: 3rem 2rem;
    margin-bottom: 2rem;
    text-align: center;
  }

  .reference-header h1 {
    margin: 0;
    font-size: 2.5rem;
    font-weight: 800;
    letter-spacing: -0.5px;
    color: #fff;
  }

  .subtitle {
    margin: 0.5rem 0 0;
    color: #94a3b8;
    font-size: 1.1rem;
  }

  .tabs {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .tab-btn {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #94a3b8;
    padding: 0.75rem 1.5rem;
    border-radius: 10px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.95rem;
  }

  .tab-btn:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.2);
    color: #fff;
  }

  .tab-btn.active {
    background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
    border-color: #3b82f6;
    color: #fff;
    box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3);
  }

  .content {
    padding: 2rem;
    overflow-y: auto;
    flex: 1;
    min-height: 0;
  }

  .content::-webkit-scrollbar {
    width: 8px;
  }

  .content::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }

  .content::-webkit-scrollbar-thumb {
    background: rgba(59, 130, 246, 0.4);
    border-radius: 10px;
  }

  .content::-webkit-scrollbar-thumb:hover {
    background: rgba(59, 130, 246, 0.6);
  }
</style>
