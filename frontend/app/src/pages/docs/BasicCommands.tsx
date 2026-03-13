export function BasicCommands() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          Commands & Keys
        </h1>
        <p className="text-lg text-muted-foreground">
          Reference guide for jump controls.
        </p>
      </div>

      <div className="space-y-4">
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          CLI Options
        </h2>
        <p className="leading-7">
          You can use <code>jump</code> or the short alias <code>j</code>.
        </p>
        <pre className="mb-4 mt-6 overflow-x-auto rounded-lg border bg-zinc-950 py-4 dark:bg-zinc-900">
          <code className="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm text-zinc-50">
{`j              # Start navigator (hidden files hidden, files hidden)
j -a           # Start with hidden files visible
j --all        # Same as -a
j -h           # Show help
j -v           # Show version`}
          </code>
        </pre>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Navigation
        </h2>
        <div className="my-6 w-full overflow-y-auto">
          <table className="w-full">
            <thead>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <th className="border px-4 py-2 text-left font-bold">Key</th>
                <th className="border px-4 py-2 text-left font-bold">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>j / k</code></td>
                <td className="border px-4 py-2">Move selection down / up</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>h</code></td>
                <td className="border px-4 py-2">Navigate to parent directory</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>l</code></td>
                <td className="border px-4 py-2">Navigate into selected directory</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>g</code></td>
                <td className="border px-4 py-2">Go to first item</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>G</code></td>
                <td className="border px-4 py-2">Go to last item</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>Ctrl+U / Ctrl+D</code></td>
                <td className="border px-4 py-2">Page up / down</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>[0-9]</code> prefix</td>
                <td className="border px-4 py-2">Motion count (e.g. <code>3j</code> moves down 3)</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Search & Selection
        </h2>
        <div className="my-6 w-full overflow-y-auto">
          <table className="w-full">
            <thead>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <th className="border px-4 py-2 text-left font-bold">Key</th>
                <th className="border px-4 py-2 text-left font-bold">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>/</code></td>
                <td className="border px-4 py-2">Enter search mode (fuzzy filter)</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>Enter</code></td>
                <td className="border px-4 py-2">Select — cd into directory, or open file in Neovim</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>Esc</code></td>
                <td className="border px-4 py-2">Cancel search / quit</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Bookmarks
        </h2>
        <div className="my-6 w-full overflow-y-auto">
          <table className="w-full">
            <thead>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <th className="border px-4 py-2 text-left font-bold">Key</th>
                <th className="border px-4 py-2 text-left font-bold">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>b</code></td>
                <td className="border px-4 py-2">Bookmark selected directory (prompts for alias)</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>x</code></td>
                <td className="border px-4 py-2">Remove bookmark from selected directory</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Toggles & Creation
        </h2>
        <div className="my-6 w-full overflow-y-auto">
          <table className="w-full">
            <thead>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <th className="border px-4 py-2 text-left font-bold">Key</th>
                <th className="border px-4 py-2 text-left font-bold">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>.</code></td>
                <td className="border px-4 py-2">Toggle hidden files on/off</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>f</code></td>
                <td className="border px-4 py-2">Toggle file visibility (show files alongside dirs)</td>
              </tr>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>a</code></td>
                <td className="border px-4 py-2">Create — type a name, end with <code>/</code> for a directory, without for a file</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Other
        </h2>
        <div className="my-6 w-full overflow-y-auto">
          <table className="w-full">
            <thead>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <th className="border px-4 py-2 text-left font-bold">Key</th>
                <th className="border px-4 py-2 text-left font-bold">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr className="m-0 border-t p-0 even:bg-muted">
                <td className="border px-4 py-2"><code>?</code></td>
                <td className="border px-4 py-2">Show help screen (press any key to return)</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}
