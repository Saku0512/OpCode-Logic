<script lang="ts">
    import { page } from "$app/stores";
    import { getCourse } from "$lib/curriculum";
    import { tutorialProgress } from "$lib/stores/tutorialProgress";
    import { goto } from "$app/navigation";
    import { t } from "svelte-i18n";

    $: courseId = $page.params.courseId ?? "";
    $: course = getCourse(courseId);

    function isSectionLocked(index: number): boolean {
        if (index === 0) return false;
        const prevSection = course?.sections[index - 1];
        if (!prevSection) return true;
        return !tutorialProgress.isCleared(
            $tutorialProgress,
            courseId,
            prevSection.id,
        );
    }

    function handleSectionClick(sectionId: string, index: number) {
        if (isSectionLocked(index)) return;
        goto(`/learn/${courseId}/${sectionId}`);
    }

    function goBack() {
        goto("/learn");
    }
</script>

<div class="course-detail-page">
    <div class="header glass">
        <button class="btn-back" on:click={goBack}
            >← {$t("tutorial.common.back_to_courses")}</button
        >
        {#if course}
            <h1>{$t(`tutorial.courses.${course.id}.title`)}</h1>
            <div class="progress-bar-container">
                <!-- Optional: Visual progress bar could go here -->
            </div>
        {/if}
    </div>

    <div class="sections-list">
        {#if course}
            {#each course.sections as section, index}
                {@const locked = isSectionLocked(index)}
                {@const cleared = tutorialProgress.isCleared(
                    $tutorialProgress,
                    courseId,
                    section.id,
                )}

                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div
                    class="section-card glass"
                    class:locked
                    class:cleared
                    on:click={() => handleSectionClick(section.id, index)}
                    role="button"
                    tabindex={locked ? -1 : 0}
                >
                    <div class="status-icon">
                        {#if locked}
                            🔒
                        {:else if cleared}
                            ✅
                        {:else}
                            ▶
                        {/if}
                    </div>
                    <div class="section-info">
                        <div class="section-title">
                            {$t(`tutorial.sections.${section.id}.title`)}
                        </div>
                        <div class="section-desc">
                            <!-- Could add description here if added to curriculum -->
                            Section {index + 1}
                        </div>
                    </div>
                    {#if !locked}
                        <div class="arrow">→</div>
                    {/if}
                </div>
            {/each}
        {:else}
            <div class="error">Course not found</div>
        {/if}
    </div>
</div>

<style>
    .course-detail-page {
        height: 100vh;
        width: 100vw;
        display: flex;
        flex-direction: column;
        background: #0f172a; /* Dark background matching theme */
        color: #fff;
        overflow-y: auto;
    }

    .header {
        padding: 2rem 4rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .btn-back {
        align-self: flex-start;
        background: none;
        border: none;
        color: #94a3b8;
        font-weight: 600;
        cursor: pointer;
        padding: 0;
        font-size: 1rem;
        transition: color 0.2s;
    }

    .btn-back:hover {
        color: #fff;
    }

    h1 {
        font-size: 2.5rem;
        font-weight: 800;
        color: #f8fafc;
        margin: 0;
    }

    .sections-list {
        flex: 1;
        padding: 3rem 4rem;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        max-width: 900px;
        margin: 0 auto;
        width: 100%;
        box-sizing: border-box;
    }

    .section-card {
        display: flex;
        align-items: center;
        padding: 1.5rem 2rem;
        gap: 1.5rem;
        border-radius: 16px;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        border: 1px solid rgba(255, 255, 255, 0.05);
        background: rgba(30, 41, 59, 0.5);
    }

    .section-card:hover:not(.locked) {
        transform: translateY(-2px);
        background: rgba(30, 41, 59, 0.8);
        border-color: rgba(59, 130, 246, 0.5);
    }

    .section-card.locked {
        opacity: 0.5;
        cursor: not-allowed;
        filter: grayscale(0.8);
    }

    .section-card.cleared {
        border-color: rgba(16, 185, 129, 0.3);
        background: rgba(16, 185, 129, 0.05);
    }

    .status-icon {
        font-size: 1.5rem;
        width: 2rem;
        text-align: center;
    }

    .section-info {
        flex: 1;
    }

    .section-title {
        font-size: 1.25rem;
        font-weight: 700;
        margin-bottom: 0.25rem;
        color: #e2e8f0;
    }

    .section-desc {
        font-size: 0.9rem;
        color: #94a3b8;
        font-family: "Fira Code", monospace;
    }

    .arrow {
        color: #64748b;
        font-weight: 700;
        font-size: 1.25rem;
    }
</style>
