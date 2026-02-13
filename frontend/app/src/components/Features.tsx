const features = [
  {
    title: "Fuzzy search",
    description: "Type / to filter directories with real-time fuzzy matching"
  },
  {
    title: "Bookmarks",
    description: "Press b to bookmark any directory with custom keys"
  },
  {
    title: "Vim keybindings",
    description: "Navigate with j/k, jump with gg/G, motion counts like 3j"
  },
  {
    title: "Tree navigation",
    description: "Press h/l to navigate parent/child without leaving the TUI"
  },
  {
    title: "Relative line numbers",
    description: "Vim-style numbering shows distance from cursor"
  },
  {
    title: "SQLite persistence",
    description: "Bookmarks stored in SQLite with WAL mode for reliability"
  },
  {
    title: "Shell integration",
    description: "Works with bash, zsh, and fish shells"
  }
]

export function Features() {
  return (
    <section className="py-24 px-6 max-w-7xl mx-auto">
      <div className="max-w-4xl mx-auto space-y-12">
        <h2 className="text-3xl font-bold">What is Jump?</h2>

        <p className="text-base text-zinc-400 leading-relaxed">
          Jump is a minimal directory navigation tool that helps you move through your filesystem
          in the terminal. Built with Rust for speed and reliability.
        </p>

        <div className="space-y-6">
          {features.map((feature, idx) => (
            <div key={idx} className="flex gap-4">
              <span className="text-zinc-600 text-sm mt-1">[×]</span>
              <div>
                <h3 className="font-bold text-base mb-1">{feature.title}</h3>
                <p className="text-zinc-400 text-sm">{feature.description}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
