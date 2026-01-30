<script lang="ts">
    import { t } from "svelte-i18n";
    import type { Section, Slide } from "$lib/curriculum";

    export let sectionId: string;
    export let slides: Slide[];
    export let onFinish: () => void;

    let currentIndex = 0;

    $: currentSlide = slides[currentIndex];

    function next() {
        if (currentIndex < slides.length - 1) {
            currentIndex++;
        } else {
            onFinish();
        }
    }

    function back() {
        if (currentIndex > 0) {
            currentIndex--;
        }
    }

    function parseMarkdown(text: string) {
        if (!text) return "";
        return text
            .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
            .replace(/`(.*?)`/g, "<code>$1</code>")
            .replace(/\n/g, "<br>");
    }
</script>

<div class="slide-viewer glass">
    <div class="slide-content">
        <div class="slide-header">
            <div class="slide-title">
                {$t(
                    `tutorial.sections.${sectionId}.slides.${currentSlide.id}.title`,
                )}
            </div>
            <div class="progress">
                {currentIndex + 1} / {slides.length}
            </div>
        </div>

        <div class="slide-body">
            {#if currentSlide.image}
                <div class="image-container">
                    <img src={currentSlide.image} alt="Slide representation" />
                </div>
            {/if}
            <div class="text-content">
                {@html parseMarkdown(
                    $t(
                        `tutorial.sections.${sectionId}.slides.${currentSlide.id}.content`,
                    ),
                )}
            </div>
        </div>
    </div>

    <div class="slide-footer">
        <button class="btn-nav" on:click={back} disabled={currentIndex === 0}>
            {$t("tutorial.common.back")}
        </button>
        <button class="btn-nav primary" on:click={next}>
            {currentIndex === slides.length - 1
                ? $t("tutorial.common.start_exercise")
                : $t("tutorial.common.next")}
        </button>
    </div>
</div>

<style>
    .slide-viewer {
        display: flex;
        flex-direction: column;
        height: 100%;
        width: 100%;
        max-width: 900px;
        margin: 0 auto;
        background: rgba(13, 15, 23, 0.8);
        border-radius: 16px;
        overflow: hidden;
    }

    .slide-content {
        flex: 1;
        padding: 3rem;
        overflow-y: auto;
    }

    .slide-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        padding-bottom: 1rem;
    }

    .slide-title {
        font-size: 2rem;
        font-weight: 800;
        color: #fff;
    }

    .progress {
        font-family: "Fira Code", monospace;
        color: #64748b;
        font-size: 0.9rem;
    }

    .slide-body {
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .text-content {
        font-size: 1.15rem;
        line-height: 1.8;
        color: #cbd5e1;
    }

    :global(.text-content strong) {
        color: #f59e0b;
        font-weight: 700;
    }

    :global(.text-content code) {
        background: rgba(0, 0, 0, 0.4);
        color: #38bdf8;
        padding: 0.2rem 0.5rem;
        border-radius: 4px;
        font-family: "Fira Code", monospace;
        font-size: 0.95em;
    }

    .image-container {
        width: 100%;
        height: 300px;
        background: rgba(0, 0, 0, 0.3);
        border-radius: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .slide-footer {
        padding: 1.5rem 3rem;
        display: flex;
        justify-content: space-between;
        background: rgba(0, 0, 0, 0.2);
        border-top: 1px solid rgba(255, 255, 255, 0.05);
    }

    .btn-nav {
        padding: 0.75rem 2rem;
        border-radius: 8px;
        font-weight: 700;
        cursor: pointer;
        transition: all 0.2s;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        color: #94a3b8;
    }

    .btn-nav:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .btn-nav.primary {
        background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
        color: #fff;
        border: none;
        box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3);
    }

    .btn-nav.primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(59, 130, 246, 0.4);
    }

    .btn-nav:disabled {
        opacity: 0.3;
        cursor: not-allowed;
    }
</style>
