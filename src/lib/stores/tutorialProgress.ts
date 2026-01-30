import { writable } from "svelte/store";
import { browser } from "$app/environment";

interface TutorialProgress {
    clearedSections: Record<string, boolean>; // key: "courseId:sectionId"
}

const STORAGE_KEY = "opcode_logic_tutorial_progress";

function createProgressStore() {
    const initialState: TutorialProgress = {
        clearedSections: {},
    };

    if (browser) {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
            try {
                initialState.clearedSections = JSON.parse(stored).clearedSections || {};
            } catch (e) {
                console.error("Failed to parse tutorial progress", e);
            }
        }
    }

    const { subscribe, update } = writable<TutorialProgress>(initialState);

    return {
        subscribe,
        markCleared: (courseId: string, sectionId: string) => {
            update((state) => {
                const key = `${courseId}:${sectionId}`;
                const newState = {
                    ...state,
                    clearedSections: { ...state.clearedSections, [key]: true },
                };
                if (browser) {
                    localStorage.setItem(STORAGE_KEY, JSON.stringify(newState));
                }
                return newState;
            });
        },
        isCleared: (state: TutorialProgress, courseId: string, sectionId: string) => {
            return !!state.clearedSections[`${courseId}:${sectionId}`];
        },
        // Helper to get count of cleared sections for a course
        getClearedCount: (state: TutorialProgress, courseId: string, totalSections: string[]) => {
            return totalSections.filter(sid => state.clearedSections[`${courseId}:${sid}`]).length;
        }
    };
}

export const tutorialProgress = createProgressStore();
