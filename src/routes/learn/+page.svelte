<script lang="ts">
    import { t } from "svelte-i18n";
    import { courses } from "$lib/curriculum";
    import { goto } from "$app/navigation";
    import LanguageSelector from "$lib/components/LanguageSelector.svelte";
    import { tutorialProgress } from "$lib/stores/tutorialProgress";

    function startCourse(courseId: string) {
        goto(`/learn/${courseId}`);
    }

    function goBack() {
        goto("/");
    }
</script>

<main class="screen">
    <div class="panel glass">
        <div class="header">
            <div class="title-group">
                <button class="btn-back" on:click={goBack}
                    >← {$t("common.back_to_stages_short")}</button
                >
                <div class="title">{$t("tutorial.common.course_list")}</div>
            </div>
            <LanguageSelector />
        </div>

        <div class="grid">
            {#each courses as course}
                <button
                    class="course-card"
                    on:click={() => startCourse(course.id)}
                >
                    <div class="card-content">
                        <div class="course-title">
                            {$t(`tutorial.courses.${course.id}.title`)}
                        </div>
                        <div class="course-desc">
                            {$t(`tutorial.courses.${course.id}.description`)}
                        </div>
                        <div class="card-footer">
                            <span class="meta"
                                >{course.sections.length} Sections</span
                            >
                            <span class="meta-progress">
                                {tutorialProgress.getClearedCount(
                                    $tutorialProgress,
                                    course.id,
                                    course.sections.map((s) => s.id),
                                )} / {course.sections.length} Done
                            </span>
                            <span class="btn-start">Start Course →</span>
                        </div>
                    </div>
                </button>
            {/each}
        </div>
    </div>
</main>

<style>
    .screen {
        height: 100vh;
        width: 100vw;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        box-sizing: border-box;
    }

    .panel {
        width: min(900px, 96vw);
        padding: 2rem;
        box-sizing: border-box;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 2rem;
    }

    .title-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .btn-back {
        background: none;
        border: none;
        color: #64748b;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        padding: 0;
        text-align: left;
    }

    .title {
        font-size: 1.75rem;
        font-weight: 900;
        color: #fff;
    }

    .grid {
        display: grid;
        grid-template-columns: 1fr;
        gap: 1.5rem;
    }

    .course-card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 16px;
        padding: 2rem;
        text-align: left;
        cursor: pointer;
        transition: all 0.2s;
        color: #e2e8f0;
        width: 100%;
    }

    .course-card:hover {
        transform: translateY(-2px);
        background: rgba(255, 255, 255, 0.05);
        border-color: rgba(59, 130, 246, 0.3);
        box-shadow: 0 10px 30px -15px rgba(59, 130, 246, 0.4);
    }

    .course-title {
        font-size: 1.5rem;
        font-weight: 800;
        color: #fff;
        margin-bottom: 0.75rem;
    }

    .course-desc {
        color: #94a3b8;
        line-height: 1.6;
        margin-bottom: 1.5rem;
    }

    .card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .meta {
        font-family: "Fira Code", monospace;
        font-size: 0.8rem;
        color: #64748b;
    }

    .meta-progress {
        font-family: "Fira Code", monospace;
        font-size: 0.8rem;
        color: #10b981;
        background: rgba(16, 185, 129, 0.1);
        padding: 0.2rem 0.5rem;
        border-radius: 4px;
    }

    .btn-start {
        color: #3b82f6;
        font-weight: 700;
        font-size: 0.9rem;
    }
</style>
