import { Hero } from "./Hero"
import { Features } from "./Features"
import { CTA } from "./CTA"
import { Demo } from "./Demo"
import { BottomCTA } from "./BottomCTA"

export function LandingPage() {
  return (
    <div className="min-h-screen">
      <Hero />
      <CTA />
      <div className="border-t border-zinc-800" />
      <Demo />
      <div className="border-t border-zinc-800" />
      <Features />
      <div className="border-t border-zinc-800" />
      <BottomCTA />
    </div>
  )
}
