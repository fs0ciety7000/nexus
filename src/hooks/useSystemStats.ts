import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { SystemStats } from "@/lib/tauri";

export function useSystemStats() {
  const [stats, setStats] = useState<SystemStats | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setup = async () => {
      try {
        // Get initial snapshot
        const initialStats = await invoke<SystemStats>("get_system_stats");
        setStats(initialStats);
        setIsLoading(false);

        // Subscribe to real-time updates
        const unlistenFn = await listen<SystemStats>("system-stats", (event) => {
          setStats(event.payload);
          setError(null);
        });

        unlisten = unlistenFn;
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to load system stats");
        setIsLoading(false);
      }
    };

    setup();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return { stats, error, isLoading };
}
