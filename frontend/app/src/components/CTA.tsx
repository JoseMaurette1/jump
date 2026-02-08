import { Button } from "@/components/ui/button"
import { Copy, Terminal } from "lucide-react"
import { useState } from "react"

const installCommands = {
    linux: {
	label: "Linux",
	cmd: "curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash",
	shell: "bash"
    },
    macos: {
	label: "macOS",
	cmd: "curl -sSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | zsh",
	shell: "zsh"
    },
    windows: {
	label: "Windows (PowerShell)",
	cmd: "irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex",
	shell: "powershell"
    }
}

export function CTA() {
    const [activeTab, setActiveTab] = useState<'linux' | 'macos' | 'windows'>('linux')
    const [copied, setCopied] = useState<string | null>(null)

    const copyToClipboard = (text: string) => {
	navigator.clipboard.writeText(text)
	setCopied(text)
	setTimeout(() => setCopied(null), 2000)
    }

    return (

	<>
	    <div className="w-full max-w-2xl mx-auto">
		<div className="grid w-full grid-cols-3 mb-8 bg-muted p-1 rounded-lg">
		    <button
			onClick={() => setActiveTab('linux')}
			className={`text-sm font-medium py-2 rounded-md transition-all ${activeTab === 'linux'
			    ? 'bg-background text-foreground shadow-sm'
			    : 'text-muted-foreground hover:text-foreground'
			}`}
		    >
			Linux
		    </button>
		    <button
			onClick={() => setActiveTab('macos')}
			className={`text-sm font-medium py-2 rounded-md transition-all ${activeTab === 'macos'
			    ? 'bg-background text-foreground shadow-sm'
			    : 'text-muted-foreground hover:text-foreground'
			}`}
		    >
			macOS
		    </button>
		    <button
			onClick={() => setActiveTab('windows')}
			className={`text-sm font-medium py-2 rounded-md transition-all ${activeTab === 'windows'
			    ? 'bg-background text-foreground shadow-sm'
			    : 'text-muted-foreground hover:text-foreground'
			}`}
		    >
			Windows
		    </button>
		</div>

		<div className="relative group text-left">
		    <div className="bg-zinc-950 p-6 rounded-lg font-mono text-sm overflow-x-auto border border-zinc-800 shadow-xl">
			<div className="flex items-center gap-2 text-zinc-500 mb-2 select-none uppercase text-xs font-bold tracking-wider">
			    <Terminal className="h-3 w-3" />
			    <span>{installCommands[activeTab].shell}</span>
			</div>
			<code className="block text-zinc-100 whitespace-pre-wrap break-all pr-12">
			    {installCommands[activeTab].cmd}
			</code>
		    </div>
		    <Button
			size="icon"
			variant="ghost"
			className="absolute top-4 right-4 text-zinc-400 hover:text-white hover:bg-zinc-800"
			onClick={() => copyToClipboard(installCommands[activeTab].cmd)}
		    >
			{copied === installCommands[activeTab].cmd ? (
			    <span className="text-green-500 font-bold">✓</span>
			) : (
				<Copy className="h-4 w-4" />
			    )}
		    </Button>
		</div>
	    </div>

	    <p className="mt-8 text-sm flex text-muted-foreground justify-center">
		After installation, restart your shell so PATH updates apply.
	    </p>
	</>
    )
}
