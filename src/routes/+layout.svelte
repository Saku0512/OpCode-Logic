<script lang="ts">
    import "../lib/i18n";
    import { locale, waitLocale } from "svelte-i18n";
    import { onMount } from "svelte";

    let ready = false;

    onMount(async () => {
        const savedLocale = localStorage.getItem("preferred-language");
        if (savedLocale) {
            $locale = savedLocale;
        }
        await waitLocale();
        ready = true;
    });
</script>

{#if ready}
    <slot />
{:else}
    <div class="initial-loading">
        <div class="spinner"></div>
    </div>
{/if}

<style>
    .initial-loading {
        height: 100vh;
        width: 100vw;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #0d0f17;
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 3px solid rgba(59, 130, 246, 0.1);
        border-top-color: #3b82f6;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
