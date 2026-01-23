<script lang="ts">
  interface Register {
    name: string;
    bits: number;
    purpose: string;
    commonUses: string[];
    notes?: string;
  }

  const registers: Register[] = [
    {
      name: "RAX",
      bits: 64,
      purpose: "Accumulator Register",
      commonUses: [
        "Return value from functions",
        "Syscall number specification",
        "Arithmetic operations",
      ],
      notes: "Primary register for arithmetic and logic operations",
    },
    {
      name: "RBX",
      bits: 64,
      purpose: "Base Register",
      commonUses: [
        "Base address for memory indexing",
        "General purpose storage",
      ],
      notes: "Callee-saved (must preserve if used)",
    },
    {
      name: "RCX",
      bits: 64,
      purpose: "Counter Register",
      commonUses: [
        "Loop counter",
        "String operations count",
        "Function argument (4th arg in System V ABI)",
      ],
      notes: "Often used for loop control",
    },
    {
      name: "RDX",
      bits: 64,
      purpose: "Data Register",
      commonUses: [
        "I/O operations",
        "Division (quotient in RAX, remainder in RDX)",
        "Function argument (3rd arg in System V ABI)",
      ],
      notes: "Extended for multiplication/division",
    },
    {
      name: "RSI",
      bits: 64,
      purpose: "Source Index",
      commonUses: [
        "String source",
        "Function argument (2nd arg in System V ABI)",
        "Memory source address",
      ],
      notes: "Commonly used for data source",
    },
    {
      name: "RDI",
      bits: 64,
      purpose: "Destination Index",
      commonUses: [
        "String destination",
        "Function argument (1st arg in System V ABI)",
        "System call first argument",
      ],
      notes: "Primary register for syscall arguments",
    },
    {
      name: "RBP",
      bits: 64,
      purpose: "Base Pointer",
      commonUses: [
        "Stack frame base pointer",
        "Function local variable access",
      ],
      notes: "Callee-saved, used for stack frame setup",
    },
    {
      name: "RSP",
      bits: 64,
      purpose: "Stack Pointer",
      commonUses: ["Stack top pointer", "Function return address storage"],
      notes: "Automatically managed by CALL/RET, do not manipulate directly",
    },
    {
      name: "R8-R15",
      bits: 64,
      purpose: "General Purpose Extended Registers",
      commonUses: [
        "Function arguments (R8-R11)",
        "General purpose operations",
      ],
      notes: "Caller-saved (R8-R11), caller must preserve if needed",
    },
  ];

  const subRegisters = [
    {
      parent: "RAX",
      child: "EAX",
      bits: 32,
      description: "32-bit access to RAX (zeros upper 32 bits)",
    },
    {
      parent: "RAX",
      child: "AX",
      bits: 16,
      description: "16-bit access to lower 16 bits of RAX",
    },
    {
      parent: "RAX",
      child: "AL/AH",
      bits: 8,
      description: "8-bit access (AL=lower byte, AH=upper byte)",
    },
  ];
</script>

<div class="register-reference">
  <h2 class="section-title">General Purpose Registers</h2>
  <p class="section-description">
    x86_64 architecture provides 16 general-purpose 64-bit registers. Below is
    a guide to their primary purposes and common uses.
  </p>

  <div class="registers-grid">
    {#each registers as reg (reg.name)}
      <div class="register-card">
        <div class="register-header">
          <h3 class="register-name">{reg.name}</h3>
          <span class="register-bits">{reg.bits} bits</span>
        </div>
        <div class="register-purpose">{reg.purpose}</div>
        <div class="register-uses">
          <strong>Common Uses:</strong>
          <ul>
            {#each reg.commonUses as use}
              <li>{use}</li>
            {/each}
          </ul>
        </div>
        {#if reg.notes}
          <div class="register-notes">
            <strong>Note:</strong> {reg.notes}
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <h2 class="section-title" style="margin-top: 3rem;">Sub-Register Access</h2>
  <p class="section-description">
    You can access parts of larger registers using different instruction
    suffixes. For example:
  </p>

  <div class="subregisters-table">
    <table>
      <thead>
        <tr>
          <th>Parent Register</th>
          <th>Sub-Register</th>
          <th>Width</th>
          <th>Description</th>
        </tr>
      </thead>
      <tbody>
        {#each subRegisters as sub}
          <tr>
            <td class="code">{sub.parent}</td>
            <td class="code highlight">{sub.child}</td>
            <td>{sub.bits}-bit</td>
            <td>{sub.description}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="info-box">
    <strong>💡 Tip:</strong> When you access a 32-bit register (e.g.,
    <span class="code">EAX</span>), the upper 32 bits of the 64-bit register
    (RAX) are automatically zeroed. This is useful for zero-extending values.
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
