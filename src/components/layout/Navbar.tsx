import { Terminal, Settings } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function Navbar() {
  return (
    <nav className="h-16 border-b border-nexus-border glass flex items-center justify-between px-6">
      <div className="flex items-center gap-3">
        <Terminal className="h-6 w-6 text-nexus-accent" />
        <div>
          <h1 className="text-xl font-bold">Nexus Dashboard</h1>
          <p className="text-xs text-muted-foreground">Command Center</p>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon">
          <Settings className="h-5 w-5" />
        </Button>
      </div>
    </nav>
  );
}
