import { Button } from "@/components/ui/button"
import { Copy } from "lucide-react"
import { useState } from "react"

const installCommands = {
    linux: {
	label: "linux",
	cmd: "curl -fsSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | bash"
    },
    macos: {
	label: "macOS",
	cmd: "curl -fsSL https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.sh | zsh"
    },
    windows: {
	label: "windows",
	cmd: "irm https://raw.githubusercontent.com/JoseMaurette1/jump/master/install.ps1 | iex"
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
	<section className="px-6 pb-16 max-w-7xl mx-auto">
	    {/* Install Command Box */}
	    <div className="max-w-2xl mx-auto">
		<div className="bg-zinc-900/50 rounded-xl border border-zinc-800 overflow-hidden">
		    {/* Tabs */}
		    <div className="flex border-b border-zinc-800 bg-zinc-950/50">
			{(Object.keys(installCommands) as Array<keyof typeof installCommands>).map((key) => (
			    <button
				key={key}
				onClick={() => setActiveTab(key)}
				className={`px-6 py-3 text-sm font-mono transition-colors relative ${
				    activeTab === key
					? 'text-zinc-100'
					: 'text-zinc-500 hover:text-zinc-300'
				}`}
			    >
				{installCommands[key].label}
				{activeTab === key && (
				    <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />
				)}
			    </button>
			))}
		    </div>

		    {/* Command */}
		    <div className="relative">
			<div className="p-6 font-mono text-sm">
			    <code className="text-zinc-100 block pr-12">
				{installCommands[activeTab].cmd}
			    </code>
			</div>
			<Button
			    size="icon"
			    variant="ghost"
			    className="absolute top-3 right-3 text-zinc-500 hover:text-zinc-100 hover:bg-zinc-800 transition-colors"
			    onClick={() => copyToClipboard(installCommands[activeTab].cmd)}
			>
			    {copied === installCommands[activeTab].cmd ? (
				<span className="text-green-500 font-bold text-lg">✓</span>
			    ) : (
				<Copy className="h-4 w-4" />
			    )}
			</Button>
		    </div>
		</div>
	    </div>
	</section>
    )
}
