export function HowItWorks() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          How it Works
        </h1>
        <p className="text-lg text-muted-foreground">
          Under the hood of the jump TUI.
        </p>
      </div>

      <div className="space-y-4">
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Overview
        </h2>
        <p className="leading-7">
          When you run <code>j</code>, the shell function invokes the <code>jump</code> binary, which opens
          a full-screen TUI. On exit the binary prints the selected path to stdout — the shell wrapper
          then either <code>cd</code>s into it (directory) or opens it in Neovim (file).
        </p>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          The TUI
        </h2>
        <ol className="list-decimal pl-6 space-y-2 leading-7">
          <li>
            <strong>Directory scan</strong> — <code>fs::scan_directories()</code> walks one level deep
            with <code>walkdir</code>, collecting directories (and optionally files when the
            <code>f</code> toggle is on). Results are sorted: directories first (alphabetically), then
            files (alphabetically).
          </li>
          <li>
            <strong>Fuzzy filter</strong> — Powered by <code>SkimMatcherV2</code>. Bookmarks are merged
            into results and ranked by score. Press <code>/</code> to enter search mode.
          </li>
          <li>
            <strong>State machine</strong> — The event loop in <code>main.rs</code> dispatches key events
            to one of six modes: <code>Normal</code>, <code>Search</code>, <code>BookmarkInput</code>,{" "}
            <code>BookmarkRemove</code>, <code>CreateEntry</code>, or <code>ShowHelp</code>.
          </li>
          <li>
            <strong>Rendering</strong> — <code>draw_fuzzy()</code> renders a two-line header (key hints +
            toggle status), an input/mode bar, and a list. Directories show a trailing <code>/</code>;
            files render in white (selected: yellow). A full-screen help overlay is shown in{" "}
            <code>ShowHelp</code> mode.
          </li>
        </ol>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Creating Files & Directories
        </h2>
        <p className="leading-7">
          Press <code>a</code> to enter <code>CreateEntry</code> mode. Type a name and press{" "}
          <code>Enter</code>:
        </p>
        <ul className="list-disc pl-6 space-y-2 text-muted-foreground">
          <li>Name ending with <code>/</code> → creates a directory (with <code>mkdir -p</code> semantics).</li>
          <li>Any other name → creates an empty file (parent directories are created if needed).</li>
        </ul>
        <p className="leading-7 mt-2">
          The list reloads automatically after creation.
        </p>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Bookmarks
        </h2>
        <p className="leading-7">
          Bookmarks are stored in a SQLite database at{" "}
          <code>~/.local/share/jump/jump.db</code> (Linux/WSL) using WAL mode for safe concurrent
          access. Each bookmark has a path and an optional short alias. Bookmarks are merged with the
          directory listing and float to the top of fuzzy results.
        </p>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Shell Integration
        </h2>
        <p className="leading-7">
          The <code>jump()</code> shell function captures the binary's stdout:
        </p>
        <pre className="mb-4 mt-4 overflow-x-auto rounded-lg border bg-zinc-950 py-4 dark:bg-zinc-900">
          <code className="font-mono text-sm text-zinc-50">
{`jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" ]]; then
        if [[ -d "$target" ]]; then
            cd "$target"
        elif [[ -f "$target" ]]; then
            vim "$target"
        fi
    fi
}`}
          </code>
        </pre>
        <p className="leading-7">
          This means selecting any entry in the TUI does the right thing — directories change your
          working directory, files open in your editor.
        </p>
      </div>
    </div>
  )
}
