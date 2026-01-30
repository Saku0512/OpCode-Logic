export interface Slide {
    id: string;
    image?: string;
    // Translation keys for title and content
}

export interface Exercise {
    id: string;
    levelId: string; // References a backend level ID
    initialCode?: string;
}

export interface Section {
    id: string;
    slides: Slide[];
    exercise?: Exercise;
}

export interface Course {
    id: string;
    sections: Section[];
}

export const courses: Course[] = [
    {
        id: "basics",
        sections: [
            {
                id: "intro",
                slides: [
                    { id: "s1" },
                    { id: "s2" },
                    { id: "s3" }
                ],
                exercise: {
                    id: "ex1",
                    levelId: "01_Mov&Call", // Reuse existing level for now or create new ones
                    initialCode: "section .text\n    global _start\n\n_start:\n    ; MISSION: Put 123 into RAX and exit\n    mov rax, 123\n    mov rax, 60\n    xor rdi, rdi\n    syscall"
                }
            },
            {
                id: "registers",
                slides: [
                    { id: "s1" },
                    { id: "s2" }
                ],
                exercise: {
                    id: "ex1",
                    levelId: "02_Addition",
                    initialCode: "section .text\n    global _start\n\n_start:\n    ; MISSION: Read two bytes, add 1, and write\n    ; (Level 02_Addition logic)\n    mov rax, 60\n    xor rdi, rdi\n    syscall"
                }
            }
        ]
    }
];

export function getCourse(id: string) {
    return courses.find(c => c.id === id);
}

export function getSection(courseId: string, sectionId: string) {
    const course = getCourse(courseId);
    return course?.sections.find(s => s.id === sectionId);
}
