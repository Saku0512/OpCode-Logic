<script lang="ts">
    import { locale, t } from "svelte-i18n";

    function setLanguage(lang: string) {
        $locale = lang;
        localStorage.setItem("preferred-language", lang);
    }
</script>

<div class="lang-selector">
    <button
        class="lang-btn"
        class:active={$locale?.startsWith("ja")}
        on:click={() => setLanguage("ja")}
    >
        {$t("common.lang_ja")}
    </button>
    <span class="sep">|</span>
    <button
        class="lang-btn"
        class:active={$locale?.startsWith("en")}
        on:click={() => setLanguage("en")}
    >
        {$t("common.lang_en")}
    </button>
</div>

<style>
    .lang-selector {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-family: "Fira Code", monospace;
        font-size: 0.75rem;
    }

    .lang-btn {
        background: none;
        border: none;
        color: #64748b;
        cursor: pointer;
        padding: 0.25rem 0.5rem;
        transition: all 0.2s;
        font-weight: 500;
    }

    .lang-btn:hover {
        color: #94a3b8;
    }

    .lang-btn.active {
        color: #3b82f6;
        font-weight: 700;
    }

    .sep {
        color: rgba(255, 255, 255, 0.1);
        font-size: 0.6rem;
    }
</style>
