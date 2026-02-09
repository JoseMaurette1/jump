import { useState } from "react"
import { Check, Copy } from "lucide-react"

export function Installation() {
  const [copied, setCopied] = useState<string | null>(null)

  const copyToClipboard = (text: string, id: string) => {
    navigator.clipboard.writeText(text)
    setCopied(id)
    setTimeout(() => setCopied(null), 2000)
  }

  const CodeBlock = ({ code, id }: { code: string; id: string }) => (
    <div className="relative rounded-lg border bg-zinc-950 py-4 dark:bg-zinc-900 group">
      <div className="absolute right-4 top-4">
        <button
          onClick={() => copyToClipboard(code.trim(), id)}
          className="rounded-md p-2 hover:bg-zinc-800 transition-colors"
          aria-label="Copy code"
        >
          {copied === id ? (
            <Check className="h-4 w-4 text-green-500" />
          ) : (
            <Copy className="h-4 w-4 text-zinc-500 group-hover:text-zinc-300" />
          )}
        </button>
      </div>
      <pre className="overflow-x-auto px-4">
        <code className="font-mono text-sm text-zinc-50 block">
          {code}
        </code>
      </pre>
    </div>
  )

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          Installation
        </h1>
        <p className="text-lg text-muted-foreground">
          Install jump on your machine.
        </p>
      </div>
      
      <div className="space-y-4">
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Linux / WSL
        </h2>
        <CodeBlock 
            id="linux"
            code="curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash"
        />

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          macOS (Apple Silicon)
        </h2>
        <CodeBlock 
            id="macos"
            code="curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | zsh"
        />

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Windows (PowerShell)
        </h2>
        <CodeBlock 
            id="windows"
            code="irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex"
        />

        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Shell Setup
        </h2>
        <p className="leading-7">
            The installer should handle this for you, but if you need to set it up manually:
        </p>
        
        <div className="mt-4">
            <h3 className="font-semibold mb-2">Bash / Zsh</h3>
            <CodeBlock 
                id="shell"
                code={`jump() {
  cd "$(command jump "$@")"
}
alias j="jump"`}
            />
        </div>
      </div>
    </div>
  )
}
