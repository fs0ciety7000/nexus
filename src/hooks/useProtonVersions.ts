import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ProtonVersion } from "@/lib/tauri";

export function useProtonVersions() {
  const [versions, setVersions] = useState<ProtonVersion[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadVersions = useCallback(async () => {
    try {
      setIsLoading(true);
      const result = await invoke<ProtonVersion[]>("list_proton_versions");
      setVersions(result);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load Proton versions");
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    loadVersions();
  }, [loadVersions]);

  return {
    versions,
    isLoading,
    error,
    refresh: loadVersions,
  };
}
