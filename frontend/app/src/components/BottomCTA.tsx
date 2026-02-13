import { Button } from "@/components/ui/button"
import { ArrowRight } from "lucide-react"
import { Link } from "react-router-dom"

export function BottomCTA() {
  return (
    <section className="py-24 px-6">
      <div className="max-w-4xl mx-auto space-y-16">
        {/* Main CTA */}
        <div className="space-y-6">
          <h2 className="text-2xl font-bold">Powerful, fast, and built for developers</h2>
          <p className="text-zinc-400 text-base leading-relaxed max-w-3xl">
            Jump is built with Rust for blazing-fast performance. Navigate your terminal with vim
            keybindings, fuzzy search, persistent bookmarks, and tree navigation.
          </p>
          <div>
            <Button
              variant="outline"
              className="group"
              asChild
            >
              <Link to="/docs/introduction" className="flex items-center gap-2">
                Read docs
                <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
              </Link>
            </Button>
          </div>
        </div>

        {/* Footer Links Grid */}
        <div className="border-t border-zinc-800 pt-12">
          <div className="grid grid-cols-3 gap-6 mb-12">
            <a
              href="https://github.com/JoseMaurette1/jump"
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-400 hover:text-zinc-100 transition-colors text-sm"
            >
              GitHub
            </a>
            <Link
              to="/docs/introduction"
              className="text-zinc-400 hover:text-zinc-100 transition-colors text-sm"
            >
              Docs
            </Link>
            <a
              href="https://github.com/JoseMaurette1/jump/releases"
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-400 hover:text-zinc-100 transition-colors text-sm"
            >
              Changelog
            </a>
          </div>

          <div className="flex items-center justify-between text-xs text-zinc-600 border-t border-zinc-800 pt-8">
            <span>©2026 Jump</span>
            <div className="flex gap-6">
              <span>Brand</span>
              <span>Privacy</span>
              <span>Terms</span>
              <span>English</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
