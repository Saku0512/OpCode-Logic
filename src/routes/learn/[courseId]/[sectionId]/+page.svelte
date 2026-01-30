<script lang="ts">
    import { page } from "$app/stores";
    import { getSection, courses } from "$lib/curriculum";
    import TutorialSlideViewer from "$lib/components/TutorialSlideViewer.svelte";
    import TutorialExerciseView from "$lib/components/TutorialExerciseView.svelte";
    import { goto } from "$app/navigation";
    import { t } from "svelte-i18n";

    $: params = $page.params as Record<string, string>;
    $: courseId = params.courseId;
    $: sectionId = params.sectionId;
    $: section = getSection(courseId, sectionId);

    let mode: "slides" | "exercise" = "slides";

    function handleSlidesFinish() {
        if (section?.exercise) {
            mode = "exercise";
        } else {
            nextSection();
        }
    }

    function handleExerciseComplete() {
        nextSection();
    }

    function nextSection() {
        const course = courses.find((c) => c.id === courseId);
        if (!course) return;

        const currentIndex = course.sections.findIndex(
            (s) => s.id === sectionId,
        );
        if (currentIndex < course.sections.length - 1) {
            const next = course.sections[currentIndex + 1];
            goto(`/learn/${courseId}/${next.id}`);
            mode = "slides";
        } else {
            goto("/learn");
        }
    }

    function backToSlides() {
        mode = "slides";
    }
</script>

<main class="tutorial-page">
    {#if section}
        {#if mode === "slides"}
            <TutorialSlideViewer
                {sectionId}
                slides={section.slides}
                onFinish={handleSlidesFinish}
            />
        {:else if mode === "exercise" && section.exercise}
            <TutorialExerciseView
                {sectionId}
                exercise={section.exercise}
                onBack={backToSlides}
                onComplete={handleExerciseComplete}
            />
        {/if}
    {:else}
        <div class="error">Section not found.</div>
    {/if}
</main>

<style>
    .tutorial-page {
        height: 100vh;
        width: 100vw;
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
    }

    .error {
        color: #ef4444;
        font-size: 1.25rem;
        font-weight: 700;
    }
</style>
