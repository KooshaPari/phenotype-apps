# Dashboard App Structure Implementation

This document outlines the implementation plan for optimizing the Next.js app structure in the MCP dashboard.

## Overview

The app structure optimization will provide:
- Consistent routing with Next.js App Router
- Proper error boundaries and loading states
- Responsive design for all screen sizes
- Organized component structure
- Type safety with TypeScript

## Implementation Details

### 1. Directory Structure

Create an organized directory structure:

```
mcp-dashboard-next/
├── public/            # Static assets
├── src/
│   ├── app/           # Next.js App Router pages
│   │   ├── (auth)/    # Authentication routes
│   │   ├── api/       # API routes
│   │   ├── agents/    # Agent management pages
│   │   ├── projects/  # Project management pages
│   │   ├── workflows/ # Workflow management pages
│   │   ├── analytics/ # Analytics pages
│   │   ├── settings/  # Settings pages
│   │   ├── layout.tsx # Root layout
│   │   └── page.tsx   # Home page
│   ├── components/    # React components
│   │   ├── ui/        # shadcn/ui components
│   │   ├── layouts/   # Layout components
│   │   ├── agents/    # Agent-related components
│   │   ├── projects/  # Project-related components
│   │   ├── workflows/ # Workflow-related components
│   │   ├── analytics/ # Analytics components
│   │   ├── jarvis/    # Jarvis chat components
│   │   └── common/    # Common components
│   ├── contexts/      # React context providers
│   ├── hooks/         # Custom React hooks
│   ├── lib/           # Utility functions
│   │   ├── api/       # API client
│   │   ├── utils/     # Utility functions
│   │   └── websocket/ # WebSocket client
│   ├── types/         # TypeScript type definitions
│   └── styles/        # Global styles
├── .env.local         # Environment variables
└── next.config.js     # Next.js configuration
```

### 2. Root Layout

Create a root layout with global providers:

```typescript
// src/app/layout.tsx
import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { AppProvider } from "@/contexts/AppContext";
import { ThemeProvider } from "@/contexts/ThemeContext";
import { ToastProvider } from "@/contexts/ToastContext";
import { WebSocketProvider } from "@/contexts/WebSocketContext";
import { Toaster } from "@/components/ui/toaster";
import MainLayout from "@/components/layouts/MainLayout";
import "./globals.css";

// Load fonts
const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

// Metadata
export const metadata: Metadata = {
  title: "MCP Agent Dashboard",
  description: "Multi-agent orchestration dashboard for MCP",
  viewport: "width=device-width, initial-scale=1",
  icons: {
    icon: "/favicon.ico",
  },
};

// Root layout
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        {/* Add UI fixes */}
        <link rel="stylesheet" href="/ui-fixes.css" />
        <script src="/ui-fixes.js" defer></script>
      </head>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-[#1f2022] text-[#f6f5f5]`}
      >
        <ThemeProvider>
          <WebSocketProvider>
            <AppProvider>
              <ToastProvider>
                <MainLayout>{children}</MainLayout>
                <Toaster />
              </ToastProvider>
            </AppProvider>
          </WebSocketProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
```

### 3. Home Page

Create a home page with dashboard overview:

```typescript
// src/app/page.tsx
import { Metadata } from "next";
import DashboardOverview from "@/components/common/DashboardOverview";
import { AgentStats } from "@/components/agents/AgentStats";
import { ProjectStats } from "@/components/projects/ProjectStats";
import { WorkflowStats } from "@/components/workflows/WorkflowStats";
import { ActivityFeed } from "@/components/common/ActivityFeed";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export const metadata: Metadata = {
  title: "Dashboard | MCP Agent Dashboard",
  description: "Overview of your MCP agent system",
};

export default function HomePage() {
  return (
    <div className="flex flex-col gap-6 p-6">
      <h1 className="text-3xl font-bold text-[#7ebab5]">Dashboard</h1>
      
      <DashboardOverview />
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <AgentStats />
        <ProjectStats />
        <WorkflowStats />
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Recent Activity</CardTitle>
          </CardHeader>
          <CardContent>
            <ActivityFeed limit={10} />
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader>
            <CardTitle>System Status</CardTitle>
          </CardHeader>
          <CardContent>
            {/* System status component */}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
```

### 4. Error Handling

Create error boundary components:

```typescript
// src/components/common/ErrorBoundary.tsx
"use client";

import React, { Component, ErrorInfo, ReactNode } from "react";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { AlertTriangle } from "lucide-react";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("Uncaught error:", error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        this.props.fallback || (
          <Card className="w-full max-w-md mx-auto my-8 border-red-500/50">
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-red-500">
                <AlertTriangle className="h-5 w-5" />
                Something went wrong
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">
                {this.state.error?.message || "An unexpected error occurred"}
              </p>
              {process.env.NODE_ENV !== "production" && this.state.error && (
                <pre className="mt-4 p-4 bg-muted rounded-md overflow-auto text-xs">
                  {this.state.error.stack}
                </pre>
              )}
            </CardContent>
            <CardFooter>
              <Button
                variant="outline"
                onClick={() => this.setState({ hasError: false, error: null })}
              >
                Try again
              </Button>
            </CardFooter>
          </Card>
        )
      );
    }

    return this.props.children;
  }
}
```

### 5. Loading States

Create loading components:

```typescript
// src/components/common/LoadingSpinner.tsx
import { Loader2 } from "lucide-react";
import { cn } from "@/lib/utils";

interface LoadingSpinnerProps {
  size?: "sm" | "md" | "lg";
  className?: string;
}

export function LoadingSpinner({ size = "md", className }: LoadingSpinnerProps) {
  const sizeClasses = {
    sm: "h-4 w-4",
    md: "h-8 w-8",
    lg: "h-12 w-12",
  };

  return (
    <Loader2
      className={cn("animate-spin text-[#7ebab5]", sizeClasses[size], className)}
    />
  );
}
```

```typescript
// src/app/loading.tsx
import { LoadingSpinner } from "@/components/common/LoadingSpinner";

export default function Loading() {
  return (
    <div className="flex items-center justify-center min-h-[60vh]">
      <div className="flex flex-col items-center gap-4">
        <LoadingSpinner size="lg" />
        <p className="text-[#7ebab5] animate-pulse">Loading...</p>
      </div>
    </div>
  );
}
```

### 6. Not Found Page

Create a not found page:

```typescript
// src/app/not-found.tsx
import Link from "next/link";
import { FileQuestion } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function NotFound() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] text-center p-6">
      <FileQuestion className="h-24 w-24 text-[#7ebab5] mb-6" />
      <h1 className="text-4xl font-bold text-[#7ebab5] mb-2">404</h1>
      <h2 className="text-2xl font-semibold mb-4">Page Not Found</h2>
      <p className="text-muted-foreground max-w-md mb-8">
        The page you are looking for doesn't exist or has been moved.
      </p>
      <div className="flex gap-4">
        <Button asChild>
          <Link href="/">Go Home</Link>
        </Button>
        <Button variant="outline" onClick={() => window.history.back()}>
          Go Back
        </Button>
      </div>
    </div>
  );
}
```

### 7. Global Error Page

Create a global error page:

```typescript
// src/app/error.tsx
"use client";

import { useEffect } from "react";
import { AlertTriangle } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error("Global error:", error);
  }, [error]);

  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] text-center p-6">
      <AlertTriangle className="h-24 w-24 text-red-500 mb-6" />
      <h1 className="text-2xl font-bold mb-4">Something went wrong!</h1>
      <p className="text-muted-foreground max-w-md mb-8">
        {error.message || "An unexpected error occurred"}
      </p>
      <div className="flex gap-4">
        <Button onClick={reset}>Try again</Button>
        <Button variant="outline" onClick={() => window.location.href = "/"}>
          Go Home
        </Button>
      </div>
      {process.env.NODE_ENV !== "production" && (
        <div className="mt-8 p-4 bg-muted rounded-md overflow-auto text-xs text-left w-full max-w-2xl">
          <p className="font-bold mb-2">Error details:</p>
          <pre>{error.stack}</pre>
        </div>
      )}
    </div>
  );
}
```

### 8. Responsive Design

Create responsive utility components:

```typescript
// src/components/common/ResponsiveContainer.tsx
import { cn } from "@/lib/utils";

interface ResponsiveContainerProps {
  children: React.ReactNode;
  className?: string;
  maxWidth?: "sm" | "md" | "lg" | "xl" | "2xl" | "full";
}

export function ResponsiveContainer({
  children,
  className,
  maxWidth = "2xl",
}: ResponsiveContainerProps) {
  const maxWidthClasses = {
    sm: "max-w-screen-sm",
    md: "max-w-screen-md",
    lg: "max-w-screen-lg",
    xl: "max-w-screen-xl",
    "2xl": "max-w-screen-2xl",
    full: "max-w-full",
  };

  return (
    <div
      className={cn(
        "w-full px-4 mx-auto",
        maxWidthClasses[maxWidth],
        className
      )}
    >
      {children}
    </div>
  );
}
```

```typescript
// src/components/common/ResponsiveGrid.tsx
import { cn } from "@/lib/utils";

interface ResponsiveGridProps {
  children: React.ReactNode;
  className?: string;
  cols?: {
    default: number;
    sm?: number;
    md?: number;
    lg?: number;
    xl?: number;
  };
  gap?: "none" | "sm" | "md" | "lg";
}

export function ResponsiveGrid({
  children,
  className,
  cols = { default: 1, md: 2, lg: 3 },
  gap = "md",
}: ResponsiveGridProps) {
  const gapClasses = {
    none: "gap-0",
    sm: "gap-2",
    md: "gap-4",
    lg: "gap-6",
  };

  // Build grid template columns classes
  let gridCols = `grid-cols-${cols.default}`;
  
  if (cols.sm) {
    gridCols += ` sm:grid-cols-${cols.sm}`;
  }
  
  if (cols.md) {
    gridCols += ` md:grid-cols-${cols.md}`;
  }
  
  if (cols.lg) {
    gridCols += ` lg:grid-cols-${cols.lg}`;
  }
  
  if (cols.xl) {
    gridCols += ` xl:grid-cols-${cols.xl}`;
  }

  return (
    <div className={cn("grid", gridCols, gapClasses[gap], className)}>
      {children}
    </div>
  );
}
```

### 9. Next.js Configuration

Create an optimized Next.js configuration:

```typescript
// next.config.ts
import { withSentryConfig } from "@sentry/nextjs";

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    domains: ["localhost"],
  },
  experimental: {
    serverActions: true,
  },
  // Configure redirects
  async redirects() {
    return [
      {
        source: "/dashboard",
        destination: "/",
        permanent: true,
      },
    ];
  },
  // Configure headers
  async headers() {
    return [
      {
        source: "/(.*)",
        headers: [
          {
            key: "X-Content-Type-Options",
            value: "nosniff",
          },
          {
            key: "X-Frame-Options",
            value: "DENY",
          },
          {
            key: "X-XSS-Protection",
            value: "1; mode=block",
          },
        ],
      },
    ];
  },
};

// Sentry configuration (optional)
const sentryWebpackPluginOptions = {
  silent: true,
};

// Export configuration
export default process.env.SENTRY_DSN
  ? withSentryConfig(nextConfig, sentryWebpackPluginOptions)
  : nextConfig;
```

## Implementation Steps

1. Create the directory structure
2. Implement the root layout
3. Create the home page
4. Implement error handling components
5. Create loading state components
6. Implement the not found page
7. Create the global error page
8. Implement responsive design utilities
9. Configure Next.js
10. Document usage patterns for the team
