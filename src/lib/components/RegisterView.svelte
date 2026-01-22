<script lang="ts">
    export let registers: Record<string, number> = {};

    const registerOrder = [
        "RAX",
        "RBX",
        "RCX",
        "RDX",
        "RSI",
        "RDI",
        "RSP",
        "RBP",
        "R8",
        "R9",
        "R10",
        "R11",
        "R12",
        "R13",
        "R14",
        "R15",
    ];
</script>

<div class="registers">
    <h3>Registers (x86_64)</h3>
    <div class="grid">
        {#each registerOrder as name}
            <div class="register">
                <span class="label">{name}</span>
                <span class="value">{registers[name] ?? 0}</span>
            </div>
        {/each}
    </div>
</div>

<style>
    .registers {
        background-color: #252526;
        padding: 1rem;
        color: #fff;
        border-radius: 4px;
        overflow-y: auto;
        max-height: 400px;
    }

    h3 {
        margin-top: 0;
        margin-bottom: 0.5rem;
        font-size: 1rem;
        color: #ccc;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 0.5rem;
    }

    .register {
        display: flex;
        justify-content: space-between;
        background-color: #333;
        padding: 0.5rem;
        border-radius: 2px;
        font-size: 0.9rem;
    }

    .label {
        font-weight: bold;
        color: #ce9178;
    }

    .value {
        font-family: monospace;
    }
</style>
