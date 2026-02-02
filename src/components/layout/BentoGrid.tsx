import { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface BentoGridProps {
  children: ReactNode;
  className?: string;
}

export default function BentoGrid({ children, className }: BentoGridProps) {
  return (
    <div
      className={cn(
        "grid gap-4 p-6",
        "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
        "auto-rows-[minmax(200px,auto)]",
        className
      )}
    >
      {children}
    </div>
  );
}
