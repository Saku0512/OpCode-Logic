<script lang="ts">
  export let language: "en" | "ja" = "en";

  interface Register {
    name: string;
    bits: number;
    purpose: string;
    purposeJa: string;
    commonUses: string[];
    commonUsesJa: string[];
    notes?: string;
    notesJa?: string;
  }

  const registers: Register[] = [
    {
      name: "RAX",
      bits: 64,
      purpose: "Accumulator Register",
      purposeJa: "アキュムレータレジスタ",
      commonUses: [
        "Return value from functions",
        "Syscall number specification",
        "Arithmetic operations",
      ],
      commonUsesJa: [
        "関数の戻り値",
        "システムコール番号の指定",
        "算術演算",
      ],
      notes: "Primary register for arithmetic and logic operations",
      notesJa: "算術演算とロジック演算の主要レジスタ",
    },
    {
      name: "RBX",
      bits: 64,
      purpose: "Base Register",
      purposeJa: "ベースレジスタ",
      commonUses: [
        "Base address for memory indexing",
        "General purpose storage",
      ],
      commonUsesJa: [
        "メモリインデックスのベースアドレス",
        "汎用ストレージ",
      ],
      notes: "Callee-saved (must preserve if used)",
      notesJa: "呼び出し側保存（使用時に保存する必要がある）",
    },
    {
      name: "RCX",
      bits: 64,
      purpose: "Counter Register",
      purposeJa: "カウンターレジスタ",
      commonUses: [
        "Loop counter",
        "String operations count",
        "Function argument (4th arg in System V ABI)",
      ],
      commonUsesJa: [
        "ループカウンタ",
        "文字列操作カウント",
        "関数引数（System V ABIの第4引数）",
      ],
      notes: "Often used for loop control",
      notesJa: "ループ制御に頻繁に使用される",
    },
    {
      name: "RDX",
      bits: 64,
      purpose: "Data Register",
      purposeJa: "データレジスタ",
      commonUses: [
        "I/O operations",
        "Division (quotient in RAX, remainder in RDX)",
        "Function argument (3rd arg in System V ABI)",
      ],
      commonUsesJa: [
        "I/O操作",
        "除算（RAXに商、RDXに余りが格納）",
        "関数引数（System V ABIの第3引数）",
      ],
      notes: "Extended for multiplication/division",
      notesJa: "乗算・除算用に拡張される",
    },
    {
      name: "RSI",
      bits: 64,
      purpose: "Source Index",
      purposeJa: "ソースインデックス",
      commonUses: [
        "String source",
        "Function argument (2nd arg in System V ABI)",
        "Memory source address",
      ],
      commonUsesJa: [
        "文字列のソース",
        "関数引数（System V ABIの第2引数）",
        "メモリのソースアドレス",
      ],
      notes: "Commonly used for data source",
      notesJa: "データソースに一般的に使用される",
    },
    {
      name: "RDI",
      bits: 64,
      purpose: "Destination Index",
      purposeJa: "デスティネーションインデックス",
      commonUses: [
        "String destination",
        "Function argument (1st arg in System V ABI)",
        "System call first argument",
      ],
      commonUsesJa: [
        "文字列の宛先",
        "関数引数（System V ABIの第1引数）",
        "システムコール第1引数",
      ],
      notes: "Primary register for syscall arguments",
      notesJa: "システムコール引数の主要レジスタ",
    },
    {
      name: "RBP",
      bits: 64,
      purpose: "Base Pointer",
      purposeJa: "ベースポインタ",
      commonUses: [
        "Stack frame base pointer",
        "Function local variable access",
      ],
      commonUsesJa: [
        "スタックフレームのベースポインタ",
        "関数のローカル変数アクセス",
      ],
      notes: "Callee-saved, used for stack frame setup",
      notesJa: "呼び出し側保存、スタックフレーム設定に使用",
    },
    {
      name: "RSP",
      bits: 64,
      purpose: "Stack Pointer",
      purposeJa: "スタックポインタ",
      commonUses: ["Stack top pointer", "Function return address storage"],
      commonUsesJa: ["スタックトップポインタ", "関数の戻りアドレス格納"],
      notes: "Automatically managed by CALL/RET, do not manipulate directly",
      notesJa: "CALL/RETで自動管理、直接操作しないこと",
    },
    {
      name: "R8-R15",
      bits: 64,
      purpose: "General Purpose Extended Registers",
      purposeJa: "汎用拡張レジスタ",
      commonUses: [
        "Function arguments (R8-R11)",
        "General purpose operations",
      ],
      commonUsesJa: [
        "関数引数（R8-R11）",
        "汎用操作",
      ],
      notes: "Caller-saved (R8-R11), caller must preserve if needed",
      notesJa: "呼び出し元保存（R8-R11）、必要に応じて保存",
    },
  ];

  const subRegisters = [
    {
      parent: "RAX",
      child: "EAX",
      bits: 32,
      description: "32-bit access to RAX (zeros upper 32 bits)",
      descriptionJa: "RAXへの32ビットアクセス（上位32ビットはゼロになる）",
    },
    {
      parent: "RAX",
      child: "AX",
      bits: 16,
      description: "16-bit access to lower 16 bits of RAX",
      descriptionJa: "RAXの下位16ビットへの16ビットアクセス",
    },
    {
      parent: "RAX",
      child: "AL/AH",
      bits: 8,
      description: "8-bit access (AL=lower byte, AH=upper byte)",
      descriptionJa: "8ビットアクセス（AL=下位バイト、AH=上位バイト）",
    },
  ];
</script>

<div class="register-reference">
  <h2 class="section-title">
    {language === "en" ? "General Purpose Registers" : "汎用レジスタ"}
  </h2>
  <p class="section-description">
    {language === "en"
      ? "x86_64 architecture provides 16 general-purpose 64-bit registers. Below is a guide to their primary purposes and common uses."
      : "x86_64アーキテクチャは16個の汎用64ビットレジスタを提供します。以下は、これらの主な用途と一般的な使用方法のガイドです。"}
  </p>

  <div class="registers-grid">
    {#each registers as reg (reg.name)}
      <div class="register-card">
        <div class="register-header">
          <h3 class="register-name">{reg.name}</h3>
          <span class="register-bits">{reg.bits} bits</span>
        </div>
        <div class="register-purpose">
          {language === "en" ? reg.purpose : reg.purposeJa}
        </div>
        <div class="register-uses">
          <strong>{language === "en" ? "Common Uses:" : "一般的な用途："}</strong>
          <ul>
            {#each language === "en" ? reg.commonUses : reg.commonUsesJa as use}
              <li>{use}</li>
            {/each}
          </ul>
        </div>
        {#if language === "en" ? reg.notes : reg.notesJa}
          <div class="register-notes">
            <strong>{language === "en" ? "Note:" : "注記："}</strong>
            {language === "en" ? reg.notes : reg.notesJa}
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <h2 class="section-title" style="margin-top: 3rem;">
    {language === "en" ? "Sub-Register Access" : "サブレジスタアクセス"}
  </h2>
  <p class="section-description">
    {language === "en"
      ? "You can access parts of larger registers using different instruction suffixes. For example:"
      : "異なる命令サフィックスを使用して、より大きなレジスタの一部にアクセスできます。例："}
  </p>

  <div class="subregisters-table">
    <table>
      <thead>
        <tr>
          <th>{language === "en" ? "Parent Register" : "親レジスタ"}</th>
          <th>{language === "en" ? "Sub-Register" : "サブレジスタ"}</th>
          <th>{language === "en" ? "Width" : "幅"}</th>
          <th>{language === "en" ? "Description" : "説明"}</th>
        </tr>
      </thead>
      <tbody>
        {#each subRegisters as sub}
          <tr>
            <td class="code">{sub.parent}</td>
            <td class="code highlight">{sub.child}</td>
            <td>{sub.bits}-bit</td>
            <td>{language === "en" ? sub.description : sub.descriptionJa}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="info-box">
    <strong>💡 {language === "en" ? "Tip:" : "ヒント："}</strong>
    {language === "en"
      ? "When you access a 32-bit register (e.g., EAX), the upper 32 bits of the 64-bit register (RAX) are automatically zeroed. This is useful for zero-extending values."
      : "32ビットレジスタ（例えばEAX）にアクセスすると、64ビットレジスタ（RAX）の上位32ビットが自動的にゼロになります。これはゼロ拡張に便利です。"}
  </div>
</div>

<style>
  .register-reference {
    width: 100%;
  }

  .section-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #fff;
    margin: 0 0 0.5rem;
  }

  .section-description {
    color: #cbd5e1;
    line-height: 1.6;
    margin-bottom: 2rem;
  }

  .registers-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .register-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    transition: all 0.2s;
  }

  .register-card:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(59, 130, 246, 0.4);
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.15);
  }

  .register-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 1rem;
    border-bottom: 2px solid rgba(59, 130, 246, 0.3);
    padding-bottom: 0.75rem;
  }

  .register-name {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 700;
    color: #60a5fa;
    font-family: "Fira Code", monospace;
  }

  .register-bits {
    background: rgba(59, 130, 246, 0.2);
    color: #93c5fd;
    padding: 0.25rem 0.75rem;
    border-radius: 6px;
    font-size: 0.85rem;
    font-weight: 600;
    font-family: "Fira Code", monospace;
  }

  .register-purpose {
    color: #94a3b8;
    font-weight: 600;
    margin-bottom: 1rem;
    font-size: 0.95rem;
  }

  .register-uses {
    margin-bottom: 1rem;
  }

  .register-uses strong {
    color: #cbd5e1;
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
  }

  .register-uses ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .register-uses li {
    color: #94a3b8;
    padding: 0.25rem 0 0.25rem 1.5rem;
    position: relative;
    font-size: 0.9rem;
    line-height: 1.4;
  }

  .register-uses li::before {
    content: "→";
    position: absolute;
    left: 0;
    color: #3b82f6;
    font-weight: bold;
  }

  .register-notes {
    background: rgba(34, 197, 94, 0.1);
    border-left: 3px solid #22c55e;
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.9rem;
    color: #d1fae5;
  }

  .register-notes strong {
    color: #10b981;
  }

  .subregisters-table {
    overflow-x: auto;
    margin-bottom: 2rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-family: "Fira Code", monospace;
    font-size: 0.9rem;
  }

  th {
    background: rgba(59, 130, 246, 0.1);
    color: #93c5fd;
    padding: 1rem;
    text-align: left;
    border-bottom: 2px solid rgba(59, 130, 246, 0.3);
    font-weight: 600;
  }

  td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    color: #cbd5e1;
  }

  tbody tr:hover {
    background: rgba(255, 255, 255, 0.02);
  }

  .code {
    background: rgba(0, 0, 0, 0.3);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    color: #60a5fa;
    font-weight: 600;
  }

  .highlight {
    background: rgba(59, 130, 246, 0.2);
    color: #93c5fd;
  }

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border-left: 4px solid #3b82f6;
    padding: 1rem;
    border-radius: 8px;
    color: #cbd5e1;
    line-height: 1.6;
  }

  .info-box strong {
    color: #93c5fd;
  }
</style>
