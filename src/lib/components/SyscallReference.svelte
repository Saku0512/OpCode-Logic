<script lang="ts">
  export let language: "en" | "ja" = "en";

  interface Syscall {
    number: number;
    name: string;
    args: { name: string; register: string; description: string; descriptionJa?: string }[];
    returns: string;
    returnsJa?: string;
    description: string;
    descriptionJa?: string;
  }

  const syscalls: Syscall[] = [
    {
      number: 0,
      name: "read",
      args: [
        { name: "fd", register: "RDI", description: "File descriptor (0=stdin)", descriptionJa: "ファイルディスクリプタ（0=stdin）" },
        {
          name: "buf",
          register: "RSI",
          description: "Buffer to read into",
          descriptionJa: "読み込みバッファ",
        },
        { name: "count", register: "RDX", description: "Number of bytes", descriptionJa: "バイト数" },
      ],
      returns: "Number of bytes read in RAX",
      returnsJa: "RAXに読み込まれたバイト数",
      description: "Read from file descriptor",
      descriptionJa: "ファイルディスクリプタから読み込む",
    },
    {
      number: 1,
      name: "write",
      args: [
        {
          name: "fd",
          register: "RDI",
          description: "File descriptor (1=stdout, 2=stderr)",
          descriptionJa: "ファイルディスクリプタ（1=stdout、2=stderr）",
        },
        { name: "buf", register: "RSI", description: "Buffer to write from", descriptionJa: "書き込みバッファ" },
        { name: "count", register: "RDX", description: "Number of bytes", descriptionJa: "バイト数" },
      ],
      returns: "Number of bytes written in RAX",
      returnsJa: "RAXに書き込まれたバイト数",
      description: "Write to file descriptor",
      descriptionJa: "ファイルディスクリプタに書き込む",
    },
    {
      number: 2,
      name: "open",
      args: [
        { name: "filename", register: "RDI", description: "Path to file", descriptionJa: "ファイルパス" },
        {
          name: "flags",
          register: "RSI",
          description: "Open flags (O_RDONLY=0, O_WRONLY=1)",
          descriptionJa: "オープンフラグ（O_RDONLY=0、O_WRONLY=1）",
        },
        { name: "mode", register: "RDX", description: "File mode", descriptionJa: "ファイルモード" },
      ],
      returns: "File descriptor in RAX (-errno on error)",
      returnsJa: "RAXにファイルディスクリプタ（エラー時は-errno）",
      description: "Open a file",
      descriptionJa: "ファイルを開く",
    },
    {
      number: 3,
      name: "close",
      args: [{ name: "fd", register: "RDI", description: "File descriptor", descriptionJa: "ファイルディスクリプタ" }],
      returns: "0 on success, -errno on error",
      returnsJa: "成功時は0、エラー時は-errno",
      description: "Close a file descriptor",
      descriptionJa: "ファイルディスクリプタを閉じる",
    },
    {
      number: 9,
      name: "mmap",
      args: [
        { name: "addr", register: "RDI", description: "Desired address (NULL)", descriptionJa: "希望アドレス（NULL）" },
        { name: "length", register: "RSI", description: "Size in bytes", descriptionJa: "サイズ（バイト単位）" },
        { name: "prot", register: "RDX", description: "Protection flags", descriptionJa: "保護フラグ" },
        { name: "flags", register: "R10", description: "Mapping flags", descriptionJa: "マッピングフラグ" },
        { name: "fd", register: "R8", description: "File descriptor", descriptionJa: "ファイルディスクリプタ" },
        { name: "offset", register: "R9", description: "File offset", descriptionJa: "ファイルオフセット" },
      ],
      returns: "Memory address in RAX",
      returnsJa: "RAXにメモリアドレス",
      description: "Memory map file or device",
      descriptionJa: "ファイルまたはデバイスをメモリにマップ",
    },
    {
      number: 10,
      name: "mprotect",
      args: [
        { name: "addr", register: "RDI", description: "Address to protect", descriptionJa: "保護するアドレス" },
        { name: "length", register: "RSI", description: "Size in bytes", descriptionJa: "サイズ（バイト単位）" },
        {
          name: "prot",
          register: "RDX",
          description: "Protection: PROT_READ=1, PROT_WRITE=2, PROT_EXEC=4",
          descriptionJa: "保護：PROT_READ=1、PROT_WRITE=2、PROT_EXEC=4",
        },
      ],
      returns: "0 on success, -errno on error",
      returnsJa: "成功時は0、エラー時は-errno",
      description: "Change memory protection",
      descriptionJa: "メモリ保護を変更",
    },
    {
      number: 12,
      name: "brk",
      args: [
        { name: "addr", register: "RDI", description: "New break address", descriptionJa: "新しいブレークアドレス" },
      ],
      returns: "New break address in RAX",
      returnsJa: "RAXに新しいブレークアドレス",
      description: "Change data segment size (allocate heap memory)",
      descriptionJa: "データセグメントサイズを変更（ヒープメモリを割り当て）",
    },
    {
      number: 32,
      name: "dup2",
      args: [
        { name: "oldfd", register: "RDI", description: "Source descriptor", descriptionJa: "ソースディスクリプタ" },
        { name: "newfd", register: "RSI", description: "Target descriptor", descriptionJa: "ターゲットディスクリプタ" },
      ],
      returns: "0 on success, -errno on error",
      returnsJa: "成功時は0、エラー時は-errno",
      description: "Duplicate file descriptor",
      descriptionJa: "ファイルディスクリプタを複製",
    },
    {
      number: 39,
      name: "getpid",
      args: [],
      returns: "Process ID in RAX",
      returnsJa: "RAXにプロセスID",
      description: "Get current process ID",
      descriptionJa: "現在のプロセスIDを取得",
    },
    {
      number: 60,
      name: "exit",
      args: [
        {
          name: "code",
          register: "RDI",
          description: "Exit code (0=success)",
          descriptionJa: "終了コード（0=成功）",
        },
      ],
      returns: "Never returns",
      returnsJa: "戻らない",
      description: "Terminate process",
      descriptionJa: "プロセスを終了",
    },
    {
      number: 62,
      name: "kill",
      args: [
        { name: "pid", register: "RDI", description: "Process ID", descriptionJa: "プロセスID" },
        { name: "sig", register: "RSI", description: "Signal number", descriptionJa: "シグナル番号" },
      ],
      returns: "0 on success, -errno on error",
      returnsJa: "成功時は0、エラー時は-errno",
      description: "Send signal to process",
      descriptionJa: "プロセスにシグナルを送信",
    },
    {
      number: 63,
      name: "rename",
      args: [
        { name: "oldname", register: "RDI", description: "Old path", descriptionJa: "旧パス" },
        { name: "newname", register: "RSI", description: "New path", descriptionJa: "新パス" },
      ],
      returns: "0 on success, -errno on error",
      returnsJa: "成功時は0、エラー時は-errno",
      description: "Rename or move file",
      descriptionJa: "ファイルをリネームまたは移動",
    },
    {
      number: 78,
      name: "clone",
      args: [
        { name: "fn", register: "RDI", description: "Function to execute", descriptionJa: "実行する関数" },
        { name: "child_stack", register: "RSI", description: "Child stack", descriptionJa: "子スタック" },
        { name: "flags", register: "RDX", description: "Clone flags", descriptionJa: "クローンフラグ" },
        { name: "arg", register: "R10", description: "Argument", descriptionJa: "引数" },
        { name: "parent_tidptr", register: "R8", description: "Parent tid", descriptionJa: "親tid" },
      ],
      returns: "Child PID in RAX (parent), 0 in RAX (child)",
      returnsJa: "RAXに子PID（親プロセス）、0（子プロセス）",
      description: "Create child process",
      descriptionJa: "子プロセスを作成",
    },
    {
      number: 231,
      name: "exit_group",
      args: [
        {
          name: "code",
          register: "RDI",
          description: "Exit code",
          descriptionJa: "終了コード",
        },
      ],
      returns: "Never returns",
      returnsJa: "戻らない",
      description: "Terminate all threads in group",
      descriptionJa: "グループ内すべてのスレッドを終了",
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
  <h2 class="section-title">
    {language === "en" ? "System Calls (x86_64 Linux)" : "システムコール（x86_64 Linux）"}
  </h2>
  <p class="section-description">
    {language === "en"
      ? "System calls are the interface between user applications and the Linux kernel. To invoke a syscall, set the appropriate arguments in registers and execute the syscall instruction with the syscall number in RAX."
      : "システムコールはユーザーアプリケーションとLinuxカーネル間のインターフェイスです。システムコールを実行するには、適切な引数をレジスタに設定し、RAXにシステムコール番号を指定してsyscall命令を実行します。"}
  </p>

  <div class="search-box">
    <input
      type="text"
      placeholder={language === "en" ? "Search syscall by name or number..." : "名前または番号でシステムコールを検索..."}
      on:input={handleSearch}
      class="search-input"
    />
  </div>

  {#if filteredSyscalls.length === 0}
    <div class="no-results">
      {language === "en"
        ? `No syscalls found matching "${searchQuery}"`
        : `"${searchQuery}" に一致するシステムコールが見つかりません`}
    </div>
  {:else}
    <div class="syscalls-list">
      {#each filteredSyscalls as syscall (syscall.number)}
        <div class="syscall-card">
          <div class="syscall-header">
            <div class="syscall-info">
              <h3 class="syscall-name">{syscall.name}</h3>
              <p class="syscall-description">
                {language === "en" ? syscall.description : syscall.descriptionJa || syscall.description}
              </p>
            </div>
            <div class="syscall-number">
              <span class="label">syscall</span>
              <span class="number">{syscall.number}</span>
            </div>
          </div>

          {#if syscall.args.length > 0}
            <div class="syscall-args">
              <strong class="subsection-title">
                {language === "en" ? "Arguments:" : "引数："}
              </strong>
              <table class="args-table">
                <tbody>
                  {#each syscall.args as arg}
                    <tr>
                      <td class="register-col">{arg.register}</td>
                      <td class="arg-col">
                        <span class="arg-name">{arg.name}</span>
                        <span class="arg-desc">
                          {language === "en" ? arg.description : arg.descriptionJa || arg.description}
                        </span>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}

          <div class="syscall-returns">
            <strong>{language === "en" ? "Returns:" : "戻り値："}</strong>
            <span>
              {language === "en" ? syscall.returns : syscall.returnsJa || syscall.returns}
            </span>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="usage-guide">
    <div class="how-to-section">
      <h3 class="section-title">
        {language === "en" ? "How to Make a System Call" : "システムコールの実行方法"}
      </h3>
      <div class="code-example">
        <pre><code>mov rax, <span class="number">60</span>       <span class="comment">; {language === "en" ? "syscall number (exit)" : "システムコール番号（終了）"}</span>
mov rdi, <span class="number">0</span>        <span class="comment">; {language === "en" ? "argument 1: exit code" : "引数1: 終了コード"}</span>
syscall             <span class="comment">; {language === "en" ? "invoke the syscall" : "システムコール実行"}</span></code></pre>
      </div>
    </div>

    <h3 class="section-title">
      {language === "en"
        ? "Syscall Calling Convention (System V ABI for x86_64)"
        : "システムコール呼び出し規約（x86_64用 System V ABI）"}
    </h3>
    <div class="convention-table">
      <table>
        <thead>
          <tr>
            <th>{language === "en" ? "Position" : "位置"}</th>
            <th>{language === "en" ? "Register" : "レジスタ"}</th>
            <th>{language === "en" ? "Purpose" : "用途"}</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>{language === "en" ? "Syscall #" : "システムコール#"}</td>
            <td class="code">RAX</td>
            <td>{language === "en" ? "Syscall number" : "システムコール番号"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 1" : "引数 1"}</td>
            <td class="code">RDI</td>
            <td>{language === "en" ? "First argument" : "第1引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 2" : "引数 2"}</td>
            <td class="code">RSI</td>
            <td>{language === "en" ? "Second argument" : "第2引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 3" : "引数 3"}</td>
            <td class="code">RDX</td>
            <td>{language === "en" ? "Third argument" : "第3引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 4" : "引数 4"}</td>
            <td class="code">R10</td>
            <td>{language === "en" ? "Fourth argument" : "第4引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 5" : "引数 5"}</td>
            <td class="code">R8</td>
            <td>{language === "en" ? "Fifth argument" : "第5引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Arg 6" : "引数 6"}</td>
            <td class="code">R9</td>
            <td>{language === "en" ? "Sixth argument" : "第6引数"}</td>
          </tr>
          <tr>
            <td>{language === "en" ? "Return" : "戻り値"}</td>
            <td class="code">RAX</td>
            <td>{language === "en" ? "Return value (-errno on error)" : "戻り値（エラー時は-errno）"}</td>
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
