<script lang="ts">
    export let input: number[] = [];
    export let output: number[] = [];
    export let expected: number[] = [];
</script>

<div class="io-panel">
    <div class="section">
        <h3>Input Queue</h3>
        <div class="queue input-queue">
            {#each input as val}
                <div class="item">{val}</div>
            {/each}
        </div>
    </div>

    <div class="section">
        <h3>Output Queue</h3>
        <div class="queue output-queue">
            {#each output as val}
                <div class="item">{val}</div>
            {/each}
        </div>
    </div>

    <div class="section">
        <h3>Expected Output</h3>
        <div class="queue expected-queue">
            {#each expected as val}
                <div class="item expected">{val}</div>
            {/each}
        </div>
    </div>
</div>

<style>
    .io-panel {
        background-color: #252526;
        padding: 1rem;
        color: #fff;
        border-radius: 4px;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    h3 {
        margin-top: 0;
        margin-bottom: 0.5rem;
        font-size: 1rem;
        color: #ccc;
    }

    .queue {
        display: flex;
        gap: 0.5rem;
        overflow-x: auto;
        padding-bottom: 0.5rem;
    }

    .item {
        background-color: #333;
        padding: 0.5rem 1rem;
        border-radius: 2px;
        font-family: monospace;
        min-width: 2rem;
        text-align: center;
    }

    .item.expected {
        border: 1px dashed #569cd6;
    }
</style>
