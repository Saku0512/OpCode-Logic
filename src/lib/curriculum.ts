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
                    { id: "s3" },
                    { id: "s4" }
                ],
                exercise: {
                    id: "ex1",
                    levelId: "Tutorial_Exit",
                    initialCode: "section .text\n    global _start\n\n_start:\n    ; MISSION: Set RAX to 60 and RDI to 0, then execute syscall\n    "
                }
            },
            {
                id: "io",
                slides: [
                    { id: "s1" },
                    { id: "s2" },
                    { id: "s3" }
                ],
                exercise: {
                    id: "ex1",
                    levelId: "01_Mov&Call",
                    initialCode: "section .bss\n    buf resb 1\n\nsection .text\n    global _start\n\n_start:\n    ; TASK 1: Read 1 byte from stdin (syscall 0)\n    ; Hint: rax=0, rdi=0, rsi=buf, rdx=1\n    ; Write your code here...\n    \n\n    ; TASK 2: Write 1 byte to stdout (syscall 1)\n    ; Hint: rax=1, rdi=1, rsi=buf, rdx=1\n    ; Write your code here...\n    \n\n    ; TASK 3: Exit (syscall 60)\n    mov rax, 60\n    xor rdi, rdi\n    syscall"
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
                    initialCode: "section .bss\n    buf1 resb 1\n    buf2 resb 1\n\nsection .text\n    global _start\n\n_start:\n    ; MISSION: Read a byte, Add 1, Write it (Do this twice!)\n    \n    ; --- FIRST BYTE ---\n    ; 1. Read (syscall 0) into buf1\n    mov rax, 0\n    mov rdi, 0\n    mov rsi, buf1\n    mov rdx, 1\n    syscall\n    \n    ; 2. Add 1 to buf1\n    mov bl, [buf1]\n    add bl, 1\n    mov [buf1], bl\n    \n    ; 3. Write buf1 (syscall 1)\n    mov rax, 1\n    mov rdi, 1\n    mov rsi, buf1\n    mov rdx, 1\n    syscall\n\n    ; --- SECOND BYTE ---\n    ; 4. Read (syscall 0) into buf2\n    ; Write your code here...\n    \n    ; 5. Add 1 to buf2\n    ; Write your code here...\n    \n    ; 6. Write buf2 (syscall 1)\n    ; Write your code here...\n    \n    ; Terminate\n    mov rax, 60\n    xor rdi, rdi\n    syscall"
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
