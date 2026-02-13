export function Hero() {
  return (
    <section className="py-16 px-6 max-w-7xl mx-auto">
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="inline-block">
          <span className="text-xs px-2 py-1 rounded bg-zinc-900 border border-zinc-800 text-zinc-400">
            New
          </span>
          <span className="ml-2 text-sm text-zinc-500">
			SQlite Support for Bookmarks
          </span>
        </div>

        <h1 className="text-4xl sm:text-5xl font-bold leading-tight">
          The fastest way to navigate your terminal
        </h1>

        <p className="text-base text-zinc-400 max-w-2xl">
          Free and open source. Built with Rust for speed. Vim keybindings included.
        </p>
      </div>
    </section>
  )
}
