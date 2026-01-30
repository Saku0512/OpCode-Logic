<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { t } from "svelte-i18n";
    import { goto } from "$app/navigation";
    import { tutorialProgress } from "$lib/stores/tutorialProgress";
    import Editor from "$lib/components/Editor.svelte";
    import RegisterView from "$lib/components/RegisterView.svelte";
    import IOView from "$lib/components/IOView.svelte";
    import type { Section, Exercise } from "$lib/curriculum";

    export let courseId: string;
    export let sectionId: string;
    export let exercise: Exercise;
    export let onBack: () => void;
    export let onComplete: () => void; // Used for "Next Section" navigation

    let code = exercise.initialCode || "";
    let syntax: "Intel" | "Att" = "Intel";
    let registers: Record<string, number> = {};
    let input: number[] = [];
    let output: number[] = [];
    let expected: number[] = [];
    let statusKey = "status.ready";
    let error: string | null = null;
    let simulationMessage: string | null = null;
    let levelData: any = null;
    let showSuccessModal = false;

    onMount(async () => {
        try {
            const allLevels: any[] = await invoke("get_levels");
            levelData = allLevels.find((l) => l.id === exercise.levelId);
            if (levelData && levelData.test_cases.length > 0) {
                input = levelData.test_cases[0][0];
                expected = levelData.test_cases[0][1];
            }
        } catch (e) {
            console.error("Failed to load level data", e);
        }
    });

    function parseMarkdown(text: string) {
        if (!text) return "";
        return text
            .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
            .replace(/`(.*?)`/g, "<code>$1</code>")
            .replace(/\n/g, "<br>");
    }

    async function runSimulation() {
        statusKey = "status.executing";
        error = null;
        simulationMessage = null;

        try {
            const result: any = await invoke("run_simulation", {
                code,
                syntax,
                input,
                levelId: exercise.levelId,
            });

            const vmState = result.vm_state;
            registers = vmState.registers;

            // Fix: Only fallback to RAX if we expect exactly one value and no stream output was produced.
            // If expected is empty, output should be empty.
            if (vmState.output.length > 0) {
                output = vmState.output;
            } else if (expected.length === 1) {
                output = [vmState.registers["RAX"] || 0];
            } else {
                output = [];
            }

            if (!result.success) {
                statusKey = "status.failed";
                simulationMessage = result.message;
            } else {
                statusKey = "status.success";
                // Don't call onComplete() immediately. Show modal first.
                tutorialProgress.markCleared(courseId, sectionId); // Use sectionId to match course list tracking
                showSuccessModal = true;
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

    function handleNext() {
        showSuccessModal = false;
        onComplete();
    }

    function handleStay() {
        showSuccessModal = false;
    }

    function goHome() {
        goto("/learn");
    }

    function resetCode() {
        code = exercise.initialCode || "";
    }
</script>

<div class="exercise-view">
    <div class="left-panel glass">
        <div class="panel-header">
            <button class="btn-back" on:click={onBack}
                >← {$t("tutorial.common.back_to_slides")}</button
            >
            <button class="btn-home" on:click={goHome}
                >🏠 {$t("tutorial.common.home")}</button
            >
        </div>
        <div class="content">
            <div class="badge">{$t("tutorial.common.mission")}</div>
            <h1>{$t(`tutorial.sections.${sectionId}.exercise.title`)}</h1>
            <p class="desc">
                {@html parseMarkdown(
                    $t(`tutorial.sections.${sectionId}.exercise.description`),
                )}
            </p>

            <div class="instruction-box">
                <div class="box-label">{$t("tutorial.common.instruction")}</div>
                <div class="instruction-text">
                    {@html parseMarkdown(
                        $t(
                            `tutorial.sections.${sectionId}.exercise.instruction`,
                        ),
                    )}
                </div>
            </div>
        </div>
    </div>

    <div class="right-panel">
        <div class="editor-section glass">
            <div class="section-header">
                <div class="title-group">
                    <span class="icon">📝</span>
                    {$t("common.editor")}
                </div>
                <div class="controls">
                    <select bind:value={syntax} class="syntax-select">
                        <option value="Intel"
                            >{$t("common.intel_syntax")}</option
                        >
                        <option value="Att">{$t("common.att_syntax")}</option>
                    </select>
                    <button class="btn-reset" on:click={resetCode}
                        >{$t("common.reset")}</button
                    >
                    <button class="btn-run" on:click={runSimulation}>
                        <span class="btn-icon">▶</span>
                        {$t("common.run_verify")}
                    </button>
                </div>
            </div>
            <div class="editor-wrapper">
                <Editor bind:code />
            </div>
            <div
                class="status-bar"
                class:success={statusKey === "status.success"}
                class:error={statusKey === "status.failed" ||
                    statusKey.includes("error")}
            >
                <span class="status-text">{$t(statusKey)}</span>
                {#if simulationMessage}
                    <span class="status-msg">: {simulationMessage}</span>
                {/if}
            </div>
        </div>

        <div class="bottom-panels">
            <div class="glass panel-inner">
                <div class="panel-header-small">
                    <span class="icon">📊</span>
                    {$t("common.registers")}
                </div>
                <RegisterView {registers} />
            </div>
            <div class="glass panel-inner">
                <div class="panel-header-small">
                    <span class="icon">🔌</span>
                    {$t("common.io_stream")}
                </div>
                <IOView {input} {output} {expected} />
            </div>
        </div>
    </div>
</div>

{#if showSuccessModal}
    <div class="modal-overlay">
        <div class="modal glass">
            <div class="modal-content">
                <div class="level-clear-icon">🎉</div>
                <h2>{$t("tutorial.common.level_cleared")}</h2>
                <div class="modal-actions">
                    <button class="btn-secondary" on:click={handleStay}
                        >{$t("tutorial.common.stay_here")}</button
                    >
                    <button class="btn-primary" on:click={handleNext}
                        >{$t("tutorial.common.next_section")} →</button
                    >
                </div>
            </div>
        </div>
    </div>
{/if}

{#if error}
    <div class="error-toast glass">
        <span class="error-label">{$t("common.exception")}</span>
        {error}
    </div>
{/if}

<style>
    .exercise-view {
        display: flex;
        height: 100%;
        width: 100%;
        gap: 1.5rem;
        padding: 1.5rem;
        box-sizing: border-box;
    }

    .left-panel {
        flex: 0.8;
        display: flex;
        flex-direction: column;
        padding: 2rem;
        overflow-y: auto;
    }

    .btn-back {
        background: none;
        border: none;
        color: #3b82f6;
        font-weight: 600;
        cursor: pointer;
        font-size: 0.9rem;
        margin-bottom: 2rem;
        padding: 0;
    }

    .badge {
        font-family: "Fira Code", monospace;
        font-size: 0.7rem;
        color: #3b82f6;
        letter-spacing: 2px;
        margin-bottom: 0.5rem;
    }

    h1 {
        font-size: 1.75rem;
        font-weight: 800;
        margin: 0 0 1.5rem;
    }

    .desc {
        color: #cbd5e1;
        line-height: 1.6;
        margin-bottom: 2rem;
    }

    .instruction-box {
        background: rgba(59, 130, 246, 0.05);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: 12px;
        padding: 1.5rem;
    }

    .box-label {
        font-family: "Fira Code", monospace;
        font-size: 0.7rem;
        font-weight: 700;
        color: #60a5fa;
        margin-bottom: 1rem;
        letter-spacing: 1px;
    }

    .instruction-text {
        line-height: 1.8;
        color: #e2e8f0;
    }

    .right-panel {
        flex: 1.2;
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
        min-width: 0;
    }

    .editor-section {
        flex: 1.2;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 1.25rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        background: rgba(0, 0, 0, 0.2);
    }

    .title-group {
        font-size: 0.7rem;
        font-weight: 700;
        color: #64748b;
    }

    .controls {
        display: flex;
        gap: 0.75rem;
    }

    .editor-wrapper {
        flex: 1;
        overflow: hidden;
    }

    .status-bar {
        padding: 0.75rem 1.25rem;
        font-family: "Fira Code", monospace;
        font-size: 0.8rem;
        background: rgba(0, 0, 0, 0.3);
        border-top: 1px solid rgba(255, 255, 255, 0.05);
    }

    .status-text {
        font-weight: 700;
        color: #94a3b8;
    }

    .success .status-text {
        color: #10b981;
    }
    .error .status-text {
        color: #ef4444;
    }

    .status-msg {
        color: #64748b;
    }

    .bottom-panels {
        flex: 0.8;
        display: flex;
        gap: 1.25rem;
        min-height: 0;
    }

    .panel-inner {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .panel-header-small {
        padding: 0.75rem 1rem;
        font-size: 0.65rem;
        font-weight: 700;
        color: #64748b;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    }

    .syntax-select {
        background: rgba(0, 0, 0, 0.3);
        color: #94a3b8;
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.4rem 0.75rem;
        border-radius: 6px;
        font-family: "Fira Code", monospace;
        font-size: 0.65rem;
        cursor: pointer;
    }

    .btn-reset {
        background: rgba(255, 255, 255, 0.05);
        color: #94a3b8;
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.4rem 1rem;
        border-radius: 6px;
        font-size: 0.7rem;
        font-weight: 600;
        cursor: pointer;
    }

    .btn-run {
        background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
        color: #fff;
        border: none;
        padding: 0.4rem 1.25rem;
        border-radius: 6px;
        font-weight: 700;
        font-size: 0.75rem;
        cursor: pointer;
        box-shadow: 0 4px 10px rgba(59, 130, 246, 0.2);
    }

    .error-toast {
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
        color: #fca5a5;
        padding: 1rem 1.5rem;
        border-radius: 12px;
        z-index: 1000;
        font-family: "Fira Code", monospace;
        animation: slideIn 0.3s ease-out;
    }

    .btn-home {
        background: none;
        border: none;
        color: #64748b;
        font-weight: 600;
        cursor: pointer;
        font-size: 0.9rem;
        padding: 0;
        margin-left: 1rem;
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
        animation: fadeIn 0.2s ease-out;
    }

    .modal {
        background: #1e293b;
        border: 1px solid rgba(59, 130, 246, 0.3);
        padding: 3rem;
        border-radius: 24px;
        text-align: center;
        box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
        animation: scaleIn 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    .level-clear-icon {
        font-size: 4rem;
        margin-bottom: 1rem;
    }

    .modal h2 {
        font-size: 2rem;
        color: #fff;
        margin: 0 0 2rem;
        text-shadow: 0 0 20px rgba(59, 130, 246, 0.5);
    }

    .modal-actions {
        display: flex;
        gap: 1rem;
        justify-content: center;
    }

    .btn-primary {
        background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
        color: white;
        border: none;
        padding: 0.8rem 2rem;
        border-radius: 12px;
        font-weight: 700;
        font-size: 1rem;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .btn-primary:hover {
        transform: scale(1.05);
    }

    .btn-secondary {
        background: rgba(255, 255, 255, 0.1);
        color: #94a3b8;
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 0.8rem 1.5rem;
        border-radius: 12px;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
    }

    .btn-secondary:hover {
        background: rgba(255, 255, 255, 0.15);
        color: #fff;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    @keyframes scaleIn {
        from {
            transform: scale(0.8);
            opacity: 0;
        }
        to {
            transform: scale(1);
            opacity: 1;
        }
    }
</style>
