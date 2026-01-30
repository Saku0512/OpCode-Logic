<script lang="ts">
    export let code: string = "";

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Tab") {
            e.preventDefault();
            const textarea = e.target as HTMLTextAreaElement;
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;

            // Insert 4 spaces
            code = code.substring(0, start) + "    " + code.substring(end);

            // Move cursor
            setTimeout(() => {
                textarea.selectionStart = textarea.selectionEnd = start + 4;
            }, 0);
        }
    }
</script>

<div class="editor-container">
    <textarea
        bind:value={code}
        class="editor"
        placeholder="Enter assembly code here..."
        on:keydown={handleKeydown}
    ></textarea>
</div>

<style>
    .editor-container {
        width: 100%;
        height: 100%;
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .editor {
        width: 100%;
        height: 100%;
        background-color: #1e1e1e;
        color: #d4d4d4;
        font-family: "Fira Code", monospace;
        font-size: 14px;
        border: 1px solid #333;
        padding: 1rem;
        box-sizing: border-box;
        resize: none;
    }

    .editor:focus {
        outline: none;
        border-color: #007acc;
    }
</style>
