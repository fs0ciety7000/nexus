import { useSystemStats } from "@/hooks/useSystemStats";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { LineChart, Line, ResponsiveContainer } from "recharts";
import { Activity, Cpu, Thermometer } from "lucide-react";
import { useEffect, useState } from "react";

interface DataPoint {
  value: number;
  timestamp: number;
}

const MAX_DATA_POINTS = 30;

export default function SystemSentinel() {
  const { stats, error, isLoading } = useSystemStats();
  const [cpuHistory, setCpuHistory] = useState<DataPoint[]>([]);
  const [gpuHistory, setGpuHistory] = useState<DataPoint[]>([]);

  // Update history when new stats arrive
  useEffect(() => {
    if (stats) {
      setCpuHistory((prev) => {
        const newData = [...prev, { value: stats.cpuUsage, timestamp: stats.timestamp }];
        return newData.slice(-MAX_DATA_POINTS);
      });

      if (stats.gpuUsage !== null) {
        setGpuHistory((prev) => {
          const newData = [...prev, { value: stats.gpuUsage!, timestamp: stats.timestamp }];
          return newData.slice(-MAX_DATA_POINTS);
        });
      }
    }
  }, [stats]);

  if (isLoading) {
    return (
      <Card className="col-span-2">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Activity className="h-5 w-5 text-nexus-accent" />
            System Sentinel
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-muted-foreground">Loading system data...</div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card className="col-span-2 border-nexus-danger">
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-nexus-danger">
            <Activity className="h-5 w-5" />
            System Sentinel - Error
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-nexus-danger">{error}</div>
        </CardContent>
      </Card>
    );
  }

  if (!stats) return null;

  const ramUsagePercent = ((stats.ramUsed / stats.ramTotal) * 100).toFixed(1);

  return (
    <Card className="col-span-2">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Activity className="h-5 w-5 text-nexus-accent animate-pulse" />
          System Sentinel
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* CPU Stats */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Cpu className="h-4 w-4 text-nexus-accent" />
              <span className="text-sm font-medium">CPU</span>
            </div>
            <div className="flex items-center gap-4">
              {stats.cpuTemp && (
                <div className="flex items-center gap-1 text-xs text-muted-foreground">
                  <Thermometer className="h-3 w-3" />
                  {stats.cpuTemp.toFixed(0)}°C
                </div>
              )}
              <span className="text-2xl font-bold text-nexus-accent">
                {stats.cpuUsage.toFixed(1)}%
              </span>
            </div>
          </div>

          {/* CPU Sparkline */}
          <div className="h-16 w-full rounded-md bg-nexus-surface/50 p-2">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={cpuHistory}>
                <Line
                  type="monotone"
                  dataKey="value"
                  stroke="#06b6d4"
                  strokeWidth={2}
                  dot={false}
                  isAnimationActive={false}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>

          {/* Per-core usage bars */}
          <div className="grid grid-cols-4 gap-2">
            {stats.cpuCores.slice(0, 8).map((usage, i) => (
              <div key={i} className="space-y-1">
                <div className="flex justify-between text-xs text-muted-foreground">
                  <span>Core {i}</span>
                  <span>{usage.toFixed(0)}%</span>
                </div>
                <div className="h-1.5 w-full rounded-full bg-nexus-surface">
                  <div
                    className="h-full rounded-full bg-nexus-accent transition-all duration-300"
                    style={{ width: `${Math.min(usage, 100)}%` }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* GPU Stats */}
        {stats.gpuUsage !== null && (
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Activity className="h-4 w-4 text-emerald-500" />
                <span className="text-sm font-medium">GPU</span>
              </div>
              <div className="flex items-center gap-4">
                {stats.gpuTemp && (
                  <div className="flex items-center gap-1 text-xs text-muted-foreground">
                    <Thermometer className="h-3 w-3" />
                    {stats.gpuTemp.toFixed(0)}°C
                  </div>
                )}
                <span className="text-2xl font-bold text-emerald-500">
                  {stats.gpuUsage.toFixed(1)}%
                </span>
              </div>
            </div>

            {/* GPU Sparkline */}
            <div className="h-16 w-full rounded-md bg-nexus-surface/50 p-2">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={gpuHistory}>
                  <Line
                    type="monotone"
                    dataKey="value"
                    stroke="#10b981"
                    strokeWidth={2}
                    dot={false}
                    isAnimationActive={false}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        )}

        {/* RAM Stats */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium">RAM</span>
            <span className="text-sm text-muted-foreground">
              {(stats.ramUsed / 1024).toFixed(1)} GB / {(stats.ramTotal / 1024).toFixed(1)} GB
            </span>
          </div>
          <div className="h-2 w-full rounded-full bg-nexus-surface">
            <div
              className="h-full rounded-full bg-gradient-to-r from-nexus-accent to-emerald-500 transition-all duration-300"
              style={{ width: `${ramUsagePercent}%` }}
            />
          </div>
          <div className="text-xs text-right text-muted-foreground">{ramUsagePercent}%</div>
        </div>
      </CardContent>
    </Card>
  );
}
