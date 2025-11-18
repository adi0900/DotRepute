import { ResourceLink } from "@/components/resources/ResourceLink";

// External Section
export function ExternalSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">External Resources</h1>

      <div className="grid md:grid-cols-2 gap-6">
        <ResourceLink
          theme={theme}
          title="Rust Documentation"
          description="Official Rust programming language documentation"
          url="https://doc.rust-lang.org/"
        />

        <ResourceLink
          theme={theme}
          title="ink! Documentation"
          description="Smart contract framework for Polkadot"
          url="https://use.ink/"
        />

        <ResourceLink
          theme={theme}
          title="Polkadot Wiki"
          description="Comprehensive guide to Polkadot ecosystem"
          url="https://wiki.polkadot.network/"
        />

        <ResourceLink
          theme={theme}
          title="SubQuery Docs"
          description="Blockchain indexing and querying"
          url="https://academy.subquery.network/"
        />

        <ResourceLink
          theme={theme}
          title="Next.js Documentation"
          description="React framework for production"
          url="https://nextjs.org/docs"
        />

        <ResourceLink
          theme={theme}
          title="TailwindCSS"
          description="Utility-first CSS framework"
          url="https://tailwindcss.com/"
        />
      </div>
    </div>
  );
}
