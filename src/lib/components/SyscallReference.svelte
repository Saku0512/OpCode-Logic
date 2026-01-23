<script lang="ts">
  interface Syscall {
    number: number;
    name: string;
    args: { name: string; register: string; description: string }[];
    returns: string;
    description: string;
  }

  const syscalls: Syscall[] = [
    {
      number: 0,
      name: "read",
      args: [
        { name: "fd", register: "RDI", description: "File descriptor (0=stdin)" },
        {
          name: "buf",
          register: "RSI",
          description: "Buffer to read into",
        },
        { name: "count", register: "RDX", description: "Number of bytes" },
      ],
      returns: "Number of bytes read in RAX",
      description: "Read from file descriptor",
    },
    {
      number: 1,
      name: "write",
      args: [
        {
          name: "fd",
          register: "RDI",
          description: "File descriptor (1=stdout, 2=stderr)",
        },
        { name: "buf", register: "RSI", description: "Buffer to write from" },
        { name: "count", register: "RDX", description: "Number of bytes" },
      ],
      returns: "Number of bytes written in RAX",
      description: "Write to file descriptor",
    },
    {
      number: 2,
      name: "open",
      args: [
        { name: "filename", register: "RDI", description: "Path to file" },
        {
          name: "flags",
          register: "RSI",
          description: "Open flags (O_RDONLY=0, O_WRONLY=1)",
        },
        { name: "mode", register: "RDX", description: "File mode" },
      ],
      returns: "File descriptor in RAX (-errno on error)",
      description: "Open a file",
    },
    {
      number: 3,
      name: "close",
      args: [{ name: "fd", register: "RDI", description: "File descriptor" }],
      returns: "0 on success, -errno on error",
      description: "Close a file descriptor",
    },
    {
      number: 9,
      name: "mmap",
      args: [
        { name: "addr", register: "RDI", description: "Desired address (NULL)" },
        { name: "length", register: "RSI", description: "Size in bytes" },
        { name: "prot", register: "RDX", description: "Protection flags" },
        { name: "flags", register: "R10", description: "Mapping flags" },
        { name: "fd", register: "R8", description: "File descriptor" },
        { name: "offset", register: "R9", description: "File offset" },
      ],
      returns: "Memory address in RAX",
      description: "Memory map file or device",
    },
    {
      number: 10,
      name: "mprotect",
      args: [
        { name: "addr", register: "RDI", description: "Address to protect" },
        { name: "length", register: "RSI", description: "Size in bytes" },
        {
          name: "prot",
          register: "RDX",
          description: "Protection: PROT_READ=1, PROT_WRITE=2, PROT_EXEC=4",
        },
      ],
      returns: "0 on success, -errno on error",
      description: "Change memory protection",
    },
    {
      number: 12,
      name: "brk",
      args: [
        { name: "addr", register: "RDI", description: "New break address" },
      ],
      returns: "New break address in RAX",
      description: "Change data segment size (allocate heap memory)",
    },
    {
      number: 32,
      name: "dup2",
      args: [
        { name: "oldfd", register: "RDI", description: "Source descriptor" },
        { name: "newfd", register: "RSI", description: "Target descriptor" },
      ],
      returns: "0 on success, -errno on error",
      description: "Duplicate file descriptor",
    },
    {
      number: 39,
      name: "getpid",
      args: [],
      returns: "Process ID in RAX",
      description: "Get current process ID",
    },
    {
      number: 60,
      name: "exit",
      args: [
        {
          name: "code",
          register: "RDI",
          description: "Exit code (0=success)",
        },
      ],
      returns: "Never returns",
      description: "Terminate process",
    },
    {
      number: 62,
      name: "kill",
      args: [
        { name: "pid", register: "RDI", description: "Process ID" },
        { name: "sig", register: "RSI", description: "Signal number" },
      ],
      returns: "0 on success, -errno on error",
      description: "Send signal to process",
    },
    {
      number: 63,
      name: "rename",
      args: [
        { name: "oldname", register: "RDI", description: "Old path" },
        { name: "newname", register: "RSI", description: "New path" },
      ],
      returns: "0 on success, -errno on error",
      description: "Rename or move file",
    },
    {
      number: 78,
      name: "clone",
      args: [
        { name: "fn", register: "RDI", description: "Function to execute" },
        { name: "child_stack", register: "RSI", description: "Child stack" },
        { name: "flags", register: "RDX", description: "Clone flags" },
        { name: "arg", register: "R10", description: "Argument" },
        { name: "parent_tidptr", register: "R8", description: "Parent tid" },
      ],
      returns: "Child PID in RAX (parent), 0 in RAX (child)",
      description: "Create child process",
    },
    {
      number: 231,
      name: "exit_group",
      args: [
        {
          name: "code",
          register: "RDI",
          description: "Exit code",
        },
      ],
      returns: "Never returns",
      description: "Terminate all threads in group",
    },
  ];

  let searchQuery = "";
  let filteredSyscalls = syscalls;

  function handleSearch(event: Event) {
    const query = (event.target as HTMLInputElement).value.toLowerCase();
    searchQuery = query;
    filteredSyscalls = syscalls.filter(
      (sc) =>
        sc.name.includes(query) ||
        sc.number.toString().includes(query) ||
        sc.description.toLowerCase().includes(query)
    );
  }
</script>

<div class="syscall-reference">
  <h2 class="section-title">System Calls (x86_64 Linux)</h2>
  <p class="section-description">
    System calls are the interface between user applications and the Linux
    kernel. To invoke a syscall, set the appropriate arguments in registers and
    execute the <span class="code">syscall</span> instruction with the syscall
    number in RAX.
  </p>

  <div class="search-box">
    <input
      type="text"
      placeholder="Search syscall by name or number..."
      on:input={handleSearch}
      class="search-input"
    />
  </div>

  {#if filteredSyscalls.length === 0}
    <div class="no-results">No syscalls found matching "{searchQuery}"</div>
  {:else}
    <div class="syscalls-list">
      {#each filteredSyscalls as syscall (syscall.number)}
        <div class="syscall-card">
          <div class="syscall-header">
            <div class="syscall-info">
              <h3 class="syscall-name">{syscall.name}</h3>
              <p class="syscall-description">{syscall.description}</p>
            </div>
            <div class="syscall-number">
              <span class="label">syscall</span>
              <span class="number">{syscall.number}</span>
            </div>
          </div>

          {#if syscall.args.length > 0}
            <div class="syscall-args">
              <strong class="subsection-title">Arguments:</strong>
              <table class="args-table">
                <tbody>
                  {#each syscall.args as arg}
                    <tr>
                      <td class="register-col">{arg.register}</td>
                      <td class="arg-col">
                        <span class="arg-name">{arg.name}</span>
                        <span class="arg-desc">{arg.description}</span>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}

          <div class="syscall-returns">
            <strong>Returns:</strong>
            <span>{syscall.returns}</span>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="usage-guide">
    <div class="how-to-section">
      <h3 class="section-title">How to Make a System Call</h3>
      <div class="code-example">
        <pre><code>mov rax, 60       <span class="comment">; syscall number (exit)</span>
mov rdi, 0        <span class="comment">; argument 1: exit code</span>
syscall             <span class="comment">; invoke the syscall</span></code></pre>
      </div>
    </div>

    <h3 class="section-title">Syscall Calling Convention (System V ABI for x86_64)</h3>
    <div class="convention-table">
      <table>
        <thead>
          <tr>
            <th>Position</th>
            <th>Register</th>
            <th>Purpose</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Syscall #</td>
            <td class="code">RAX</td>
            <td>Syscall number</td>
          </tr>
          <tr>
            <td>Arg 1</td>
            <td class="code">RDI</td>
            <td>First argument</td>
          </tr>
          <tr>
            <td>Arg 2</td>
            <td class="code">RSI</td>
            <td>Second argument</td>
          </tr>
          <tr>
            <td>Arg 3</td>
            <td class="code">RDX</td>
            <td>Third argument</td>
          </tr>
          <tr>
            <td>Arg 4</td>
            <td class="code">R10</td>
            <td>Fourth argument</td>
          </tr>
          <tr>
            <td>Arg 5</td>
            <td class="code">R8</td>
            <td>Fifth argument</td>
          </tr>
          <tr>
            <td>Arg 6</td>
            <td class="code">R9</td>
            <td>Sixth argument</td>
          </tr>
          <tr>
            <td>Return</td>
            <td class="code">RAX</td>
            <td>Return value (-errno on error)</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</div>

<style>
  .syscall-reference {
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

  .code {
    background: rgba(0, 0, 0, 0.3);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    color: #60a5fa;
    font-family: "Fira Code", monospace;
    font-weight: 600;
  }

  .search-box {
    margin-bottom: 2rem;
  }

  .search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    color: #cbd5e1;
    font-family: "Fira Code", monospace;
    font-size: 0.95rem;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    background: rgba(0, 0, 0, 0.3);
    border-color: #3b82f6;
    box-shadow: 0 0 12px rgba(59, 130, 246, 0.2);
  }

  .search-input::placeholder {
    color: #64748b;
  }

  .no-results {
    text-align: center;
    color: #94a3b8;
    padding: 2rem;
    background: rgba(255, 255, 255, 0.02);
    border-radius: 8px;
  }

  .syscalls-list {
    display: grid;
    gap: 1.5rem;
  }

  .syscall-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    transition: all 0.2s;
  }

  .syscall-card:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(59, 130, 246, 0.4);
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.15);
  }

  .syscall-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid rgba(59, 130, 246, 0.2);
  }

  .syscall-info {
    flex: 1;
  }

  .syscall-name {
    margin: 0 0 0.5rem;
    font-size: 1.25rem;
    font-weight: 700;
    color: #60a5fa;
    font-family: "Fira Code", monospace;
  }

  .syscall-description {
    margin: 0;
    color: #94a3b8;
    font-size: 0.95rem;
  }

  .syscall-number {
    text-align: right;
  }

  .label {
    display: block;
    color: #64748b;
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 1px;
    margin-bottom: 0.25rem;
  }

  .number {
    display: block;
    font-size: 2rem;
    font-weight: 800;
    color: #3b82f6;
    font-family: "Fira Code", monospace;
  }

  .subsection-title {
    color: #cbd5e1;
    display: block;
    margin-bottom: 0.75rem;
    font-size: 0.9rem;
  }

  .syscall-args {
    margin-bottom: 1rem;
  }

  .args-table {
    width: 100%;
    border-collapse: collapse;
    font-family: "Fira Code", monospace;
    font-size: 0.9rem;
  }

  .args-table td {
    padding: 0.5rem 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .register-col {
    color: #60a5fa;
    font-weight: 600;
    padding-right: 1.5rem;
    width: 80px;
  }

  .arg-col {
    color: #cbd5e1;
  }

  .arg-name {
    display: block;
    color: #93c5fd;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .arg-desc {
    display: block;
    color: #94a3b8;
    font-size: 0.85rem;
    font-weight: 400;
  }

  .syscall-returns {
    color: #d1fae5;
    background: rgba(34, 197, 94, 0.1);
    border-left: 3px solid #22c55e;
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .syscall-returns strong {
    color: #10b981;
    margin-right: 0.5rem;
  }

  .usage-guide {
    margin-top: 3rem;
  }

  .how-to-section {
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.15) 0%, rgba(99, 102, 241, 0.1) 100%);
    border: 2px solid rgba(59, 130, 246, 0.3);
    border-radius: 12px;
    padding: 2rem;
    margin-bottom: 3rem;
  }

  .how-to-section .section-title {
    margin-top: 0;
    margin-bottom: 1.5rem;
    color: #93c5fd;
  }

  .code-example {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 0;
    overflow-x: auto;
  }

  .code-example pre {
    margin: 0;
    font-family: "Fira Code", monospace;
    font-size: 0.95rem;
    line-height: 1.6;
    color: #cbd5e1;
  }

  .code-example code {
    display: block;
  }

  .number {
    color: #60a5fa;
    font-weight: 500;
  }

  .comment {
    color: #64748b;
  }

  .convention-table {
    overflow-x: auto;
    margin: 1rem 0;
  }

  .convention-table table {
    width: 100%;
    border-collapse: collapse;
    font-family: "Fira Code", monospace;
    font-size: 0.9rem;
  }

  .convention-table th {
    background: rgba(59, 130, 246, 0.1);
    color: #93c5fd;
    padding: 1rem;
    text-align: left;
    border-bottom: 2px solid rgba(59, 130, 246, 0.3);
    font-weight: 600;
  }

  .convention-table td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    color: #cbd5e1;
  }

  .convention-table tbody tr:hover {
    background: rgba(255, 255, 255, 0.02);
  }
</style>
