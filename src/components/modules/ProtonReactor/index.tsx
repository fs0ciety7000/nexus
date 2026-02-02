import { useProtonVersions } from "@/hooks/useProtonVersions";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Gamepad2, RefreshCw, Download, CheckCircle2 } from "lucide-react";
import { formatBytes, formatDate } from "@/lib/utils";

export default function ProtonReactor() {
  const { versions, isLoading, error, refresh } = useProtonVersions();

  if (error) {
    return (
      <Card className="border-nexus-danger">
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-nexus-danger">
            <Gamepad2 className="h-5 w-5" />
            Proton Reactor - Error
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-nexus-danger text-sm">{error}</div>
          <Button
            variant="outline"
            size="sm"
            className="mt-4"
            onClick={refresh}
          >
            <RefreshCw className="h-4 w-4 mr-2" />
            Retry
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Gamepad2 className="h-5 w-5 text-nexus-accent" />
            Proton Reactor
          </CardTitle>
          <Button
            variant="ghost"
            size="icon"
            onClick={refresh}
            disabled={isLoading}
          >
            <RefreshCw className={`h-4 w-4 ${isLoading ? "animate-spin" : ""}`} />
          </Button>
        </div>
        {versions.length > 0 && (
          <div className="text-xs text-muted-foreground">
            {versions.length} version{versions.length !== 1 ? "s" : ""} installed
          </div>
        )}
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <div className="text-muted-foreground">Scanning Proton installations...</div>
        ) : versions.length === 0 ? (
          <div className="text-center py-8">
            <Gamepad2 className="h-12 w-12 mx-auto mb-3 opacity-50" />
            <p className="text-muted-foreground mb-4">No Proton versions found</p>
            <Button variant="nexus" size="sm">
              <Download className="h-4 w-4 mr-2" />
              Download GE-Proton (Mocked)
            </Button>
          </div>
        ) : (
          <div className="space-y-3">
            {versions.map((version) => (
              <div
                key={version.id}
                className="p-4 rounded-lg glass hover:border-nexus-accent/50 transition-smooth"
              >
                <div className="flex items-start justify-between gap-3">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="font-medium text-sm truncate">{version.name}</h4>
                      {version.isGeProton && (
                        <Badge variant="nexus" className="text-xs">
                          GE-Proton
                        </Badge>
                      )}
                    </div>
                    <div className="flex flex-wrap gap-3 text-xs text-muted-foreground">
                      {version.version && (
                        <span>Version {version.version}</span>
                      )}
                      <span>{formatBytes(version.size * 1024 * 1024)}</span>
                      {version.installedDate && (
                        <span>Installed {formatDate(version.installedDate)}</span>
                      )}
                    </div>
                  </div>
                  <CheckCircle2 className="h-5 w-5 text-nexus-success flex-shrink-0" />
                </div>
              </div>
            ))}

            {/* Mock Download Button */}
            <Button
              variant="outline"
              className="w-full"
              onClick={() => alert("Download functionality is mocked for demo purposes")}
            >
              <Download className="h-4 w-4 mr-2" />
              Download Latest GE-Proton
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
