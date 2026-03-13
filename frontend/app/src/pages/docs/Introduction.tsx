export function Introduction() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          Introduction
        </h1>
        <p className="text-lg text-muted-foreground">
          A Vim-inspired terminal navigator — fuzzy search, bookmarks, and file creation in one TUI.
        </p>
      </div>
      <div className="space-y-4">
        <p className="leading-7 [&:not(:first-child)]:mt-6">
          <strong>jump</strong> is a minimal, Vim-inspired directory and file navigator built in Rust.
          It replaces <code>cd</code> with an interactive TUI that lets you move through your filesystem
          using familiar Vim keybindings.
        </p>
        <p className="leading-7">
          Type <code>j</code> in your terminal and an interactive list appears. Use <code>j/k</code> to
          move, <code>h/l</code> to traverse the directory tree, <code>/</code> to fuzzy-search, and
          <code>Enter</code> to jump. Selecting a file opens it directly in your editor.
        </p>
        <ul className="list-disc pl-6 space-y-2 mt-4 text-muted-foreground">
          <li><strong>Fuzzy search:</strong> Press <code>/</code> and type — results filter instantly.</li>
          <li><strong>Bookmarks:</strong> Press <code>b</code> to pin any directory with a short alias.</li>
          <li><strong>File creation:</strong> Press <code>a</code> to create a file or directory without leaving the navigator.</li>
          <li><strong>File browsing:</strong> Press <code>f</code> to toggle files alongside directories.</li>
          <li><strong>Editor integration:</strong> Selecting a file opens it in Neovim.</li>
          <li><strong>No daemon:</strong> Zero background processes — runs entirely on demand.</li>
        </ul>
      </div>
    </div>
  )
}
