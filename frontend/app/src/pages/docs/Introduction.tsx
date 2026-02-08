export function Introduction() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          Introduction
        </h1>
        <p className="text-lg text-muted-foreground">
          Navigate your terminal with deterministic key strokes.
        </p>
      </div>
      <div className="space-y-4">
        <p className="leading-7 [&:not(:first-child)]:mt-6">
          <strong>jump</strong> is a minimal, Vim-inspired directory navigation tool built in Rust. 
        </p>
        <p className="leading-7">
          Unlike other tools that use fuzzy search or history ranking, <strong>jump</strong> assigns short, deterministic 2-letter labels to directories. You look at the label, type it, and you are there.
        </p>
        <ul className="list-disc pl-6 space-y-2 mt-4 text-muted-foreground">
            <li><strong>Deterministic:</strong> Same directory always gets the same label (where possible).</li>
            <li><strong>Fast:</strong> No indexing daemon, no background processes.</li>
            <li><strong>Muscle Memory:</strong> Learn the keys for your favorite projects.</li>
        </ul>
      </div>
    </div>
  )
}
