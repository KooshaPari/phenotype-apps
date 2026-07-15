# Dashboard UI Component Library Implementation

This document outlines the implementation plan for enhancing the UI component library in the MCP dashboard.

## Overview

The UI component library enhancement will provide:
- Complete shadcn/ui integration
- Neoglassmorphic design system
- Reusable component library
- Accessibility features
- Consistent styling with GMK Architect color scheme

## Implementation Details

### 1. Theme Configuration

Create a theme configuration file:

```typescript
// src/lib/theme.ts
export const theme = {
  colors: {
    // GMK Architect color scheme
    background: "#1f2022",
    foreground: "#f6f5f5",
    primary: "#7ebab5",
    primaryLight: "#a3d0cc",
    primaryDark: "#5a8a86",
    secondary: "#2d3748",
    muted: "#4a5568",
    accent: "#805ad5",
    destructive: "#e53e3e",
    success: "#38a169",
    warning: "#d69e2e",
  },
  fonts: {
    sans: "var(--font-geist-sans)",
    mono: "var(--font-geist-mono)",
  },
  radii: {
    sm: "0.125rem",
    md: "0.25rem",
    lg: "0.5rem",
    xl: "1rem",
    full: "9999px",
  },
  shadows: {
    sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
    md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
    lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
    xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
    neoglass: "0 8px 32px 0 rgba(0, 0, 0, 0.37)",
    "neoglass-hover": "0 8px 32px 0 rgba(126, 186, 181, 0.2)",
  },
  transitions: {
    fast: "150ms cubic-bezier(0.4, 0, 0.2, 1)",
    normal: "300ms cubic-bezier(0.4, 0, 0.2, 1)",
    slow: "500ms cubic-bezier(0.4, 0, 0.2, 1)",
  },
};

// Export individual theme values for easier access
export const colors = theme.colors;
export const fonts = theme.fonts;
export const radii = theme.radii;
export const shadows = theme.shadows;
export const transitions = theme.transitions;
```

### 2. Global CSS

Create global CSS with neoglassmorphic styles:

```css
/* src/app/globals.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 220 3% 13%;
    --foreground: 0 0% 96%;
    --card: 220 3% 13%;
    --card-foreground: 0 0% 96%;
    --popover: 220 3% 13%;
    --popover-foreground: 0 0% 96%;
    --primary: 174 35% 63%;
    --primary-foreground: 0 0% 96%;
    --secondary: 222 19% 23%;
    --secondary-foreground: 0 0% 96%;
    --muted: 222 19% 35%;
    --muted-foreground: 0 0% 80%;
    --accent: 262 60% 57%;
    --accent-foreground: 0 0% 96%;
    --destructive: 0 84% 57%;
    --destructive-foreground: 0 0% 96%;
    --success: 142 50% 43%;
    --success-foreground: 0 0% 96%;
    --warning: 38 74% 51%;
    --warning-foreground: 0 0% 96%;
    --border: 174 35% 63% / 0.1;
    --input: 174 35% 63% / 0.1;
    --ring: 174 35% 63% / 0.5;
    --radius: 0.5rem;
  }

  * {
    @apply border-border;
  }

  body {
    @apply bg-background text-foreground;
    font-feature-settings: "rlig" 1, "calt" 1;
  }
}

@layer components {
  /* Neoglassmorphic card */
  .neoglass {
    @apply bg-background/60 backdrop-blur-sm border border-primary/10 shadow-neoglass;
  }

  /* Gradient text */
  .gradient-text {
    @apply text-transparent bg-clip-text bg-gradient-to-r from-primary to-primary-light;
  }

  /* Gradient separator */
  .gradient-separator {
    @apply h-px w-full bg-gradient-to-r from-transparent via-primary/20 to-transparent;
  }

  /* Animated glow effect */
  .glow-effect {
    @apply relative overflow-hidden;
  }

  .glow-effect::after {
    content: "";
    @apply absolute inset-0 bg-gradient-to-r from-transparent via-primary/10 to-transparent opacity-0 transition-opacity duration-1000;
    animation: glow 3s ease-in-out infinite;
  }

  @keyframes glow {
    0%, 100% { opacity: 0; transform: translateX(-100%); }
    50% { opacity: 0.5; transform: translateX(100%); }
  }

  /* Animated border */
  .animated-border {
    @apply relative;
  }

  .animated-border::before {
    content: "";
    @apply absolute inset-0 rounded-[calc(var(--radius)+1px)] p-[1px];
    background: linear-gradient(
      90deg,
      rgba(126, 186, 181, 0.1),
      rgba(126, 186, 181, 0.3),
      rgba(126, 186, 181, 0.1)
    );
    mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    mask-composite: exclude;
    animation: border-animation 4s linear infinite;
  }

  @keyframes border-animation {
    0% { background-position: 0% center; }
    100% { background-position: 200% center; }
  }

  /* Tilt effect */
  .tilt-effect {
    @apply transition-transform duration-300 ease-out transform-gpu;
  }

  /* Particle background */
  .particle-background {
    @apply relative overflow-hidden;
  }

  .particle {
    @apply absolute rounded-full bg-primary/10 pointer-events-none;
    animation: float 10s infinite ease-in-out;
  }

  @keyframes float {
    0%, 100% { transform: translateY(0) translateX(0); }
    25% { transform: translateY(-30px) translateX(15px); }
    50% { transform: translateY(-15px) translateX(-15px); }
    75% { transform: translateY(-30px) translateX(5px); }
  }
}

/* Animation utilities */
@layer utilities {
  .animation-delay-100 {
    animation-delay: 100ms;
  }
  
  .animation-delay-200 {
    animation-delay: 200ms;
  }
  
  .animation-delay-300 {
    animation-delay: 300ms;
  }
  
  .animation-delay-500 {
    animation-delay: 500ms;
  }
  
  .animation-delay-700 {
    animation-delay: 700ms;
  }
}
```

### 3. Tailwind Configuration

Update the Tailwind configuration:

```typescript
// tailwind.config.ts
import { type Config } from "tailwindcss";
import { fontFamily } from "tailwindcss/defaultTheme";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
          light: "hsl(174, 35%, 73%)",
          dark: "hsl(174, 35%, 43%)",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        success: {
          DEFAULT: "hsl(var(--success))",
          foreground: "hsl(var(--success-foreground))",
        },
        warning: {
          DEFAULT: "hsl(var(--warning))",
          foreground: "hsl(var(--warning-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      fontFamily: {
        sans: ["var(--font-geist-sans)", ...fontFamily.sans],
        mono: ["var(--font-geist-mono)", ...fontFamily.mono],
      },
      boxShadow: {
        neoglass: "0 8px 32px 0 rgba(0, 0, 0, 0.37)",
        "neoglass-hover": "0 8px 32px 0 rgba(126, 186, 181, 0.2)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: "0" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
};

export default config;
```

### 4. Neoglass Effects Utility

Create a utility for neoglass effects:

```typescript
// src/lib/neoglass-effects.ts
/**
 * Neoglass Effects Library
 * 
 * A collection of functions to add neoglassmorphic effects to elements
 */

// Apply tilt effect to elements
export const applyTiltEffect = (selector = '.tilt-effect', intensity = 10) => {
  if (typeof window === 'undefined') return;

  const elements = document.querySelectorAll(selector);
  
  elements.forEach(element => {
    element.addEventListener('mousemove', (e) => {
      const rect = element.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      
      const xPercent = x / rect.width - 0.5;
      const yPercent = y / rect.height - 0.5;
      
      const rotateX = yPercent * intensity * -1;
      const rotateY = xPercent * intensity;
      
      element.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(1.02, 1.02, 1.02)`;
    });
    
    element.addEventListener('mouseleave', () => {
      element.style.transform = 'perspective(1000px) rotateX(0) rotateY(0) scale3d(1, 1, 1)';
    });
  });
};

// Initialize particle background
export const initParticleBackground = (selector = '.particle-background', count = 10) => {
  if (typeof window === 'undefined') return;

  const elements = document.querySelectorAll(selector);
  
  elements.forEach(element => {
    // Clear existing particles
    const existingParticles = element.querySelectorAll('.particle');
    existingParticles.forEach(p => p.remove());
    
    // Create new particles
    for (let i = 0; i < count; i++) {
      const particle = document.createElement('div');
      particle.classList.add('particle');
      
      // Random size between 5px and 20px
      const size = Math.random() * 15 + 5;
      particle.style.width = `${size}px`;
      particle.style.height = `${size}px`;
      
      // Random position
      particle.style.left = `${Math.random() * 100}%`;
      particle.style.top = `${Math.random() * 100}%`;
      
      // Random animation duration between 10s and 20s
      particle.style.animationDuration = `${Math.random() * 10 + 10}s`;
      
      // Random animation delay
      particle.style.animationDelay = `${Math.random() * 5}s`;
      
      // Random opacity
      particle.style.opacity = `${Math.random() * 0.3 + 0.1}`;
      
      element.appendChild(particle);
    }
  });
};

// Enhance buttons with hover effects
export const enhanceButtons = (selector = 'button, .button') => {
  if (typeof window === 'undefined') return;

  const buttons = document.querySelectorAll(selector);
  
  buttons.forEach(button => {
    button.addEventListener('mouseenter', () => {
      button.classList.add('shadow-neoglass-hover');
    });
    
    button.addEventListener('mouseleave', () => {
      button.classList.remove('shadow-neoglass-hover');
    });
  });
};

// Initialize scroll animations
export const initScrollAnimations = () => {
  if (typeof window === 'undefined') return;

  const animateOnScroll = () => {
    const elements = document.querySelectorAll('.animate-on-scroll');
    
    elements.forEach(element => {
      const elementTop = element.getBoundingClientRect().top;
      const elementVisible = 150;
      
      if (elementTop < window.innerHeight - elementVisible) {
        element.classList.add('animate-visible');
      } else {
        element.classList.remove('animate-visible');
      }
    });
  };
  
  window.addEventListener('scroll', animateOnScroll);
  animateOnScroll(); // Run once on init
};

// Initialize tooltips
export const initTooltips = (selector = '[data-tooltip]') => {
  if (typeof window === 'undefined') return;

  const elements = document.querySelectorAll(selector);
  
  elements.forEach(element => {
    const tooltip = document.createElement('div');
    tooltip.classList.add('tooltip');
    tooltip.textContent = element.getAttribute('data-tooltip');
    
    element.addEventListener('mouseenter', () => {
      document.body.appendChild(tooltip);
      
      const rect = element.getBoundingClientRect();
      tooltip.style.left = `${rect.left + rect.width / 2}px`;
      tooltip.style.top = `${rect.top - 10}px`;
      
      setTimeout(() => {
        tooltip.classList.add('tooltip-visible');
      }, 10);
    });
    
    element.addEventListener('mouseleave', () => {
      tooltip.classList.remove('tooltip-visible');
      
      tooltip.addEventListener('transitionend', () => {
        if (tooltip.parentNode) {
          tooltip.parentNode.removeChild(tooltip);
        }
      }, { once: true });
    });
  });
};

// Initialize all effects
export const initAllEffects = () => {
  applyTiltEffect();
  initParticleBackground();
  enhanceButtons();
  initScrollAnimations();
  initTooltips();
};
```

### 5. Card Component

Create an enhanced card component with neoglassmorphic design:

```typescript
// src/components/ui/card.tsx
import * as React from "react";
import { cn } from "@/lib/utils";

const Card = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & {
    variant?: "default" | "neoglass" | "outline";
    hoverEffect?: boolean;
  }
>(({ className, variant = "default", hoverEffect = false, ...props }, ref) => {
  const variantClasses = {
    default: "bg-card text-card-foreground",
    neoglass: "bg-card/60 backdrop-blur-sm border-primary/10",
    outline: "border border-primary/20 bg-transparent",
  };

  return (
    <div
      ref={ref}
      className={cn(
        "rounded-lg border shadow-sm",
        variantClasses[variant],
        hoverEffect && "transition-all duration-300 hover:shadow-neoglass-hover",
        className
      )}
      {...props}
    />
  );
});
Card.displayName = "Card";

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col space-y-1.5 p-6", className)}
    {...props}
  />
));
CardHeader.displayName = "CardHeader";

const CardTitle = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLHeadingElement> & {
    gradient?: boolean;
  }
>(({ className, gradient = false, ...props }, ref) => (
  <h3
    ref={ref}
    className={cn(
      "text-2xl font-semibold leading-none tracking-tight",
      gradient && "gradient-text",
      className
    )}
    {...props}
  />
));
CardTitle.displayName = "CardTitle";

const CardDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn("text-sm text-muted-foreground", className)}
    {...props}
  />
));
CardDescription.displayName = "CardDescription";

const CardContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div ref={ref} className={cn("p-6 pt-0", className)} {...props} />
));
CardContent.displayName = "CardContent";

const CardFooter = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex items-center p-6 pt-0", className)}
    {...props}
  />
));
CardFooter.displayName = "CardFooter";

export { Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent };
```

### 6. Button Component

Create an enhanced button component:

```typescript
// src/components/ui/button.tsx
import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/90",
        destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        outline: "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        link: "text-primary underline-offset-4 hover:underline",
        neoglass: "bg-primary/10 backdrop-blur-sm border border-primary/20 text-primary hover:bg-primary/20 transition-all duration-300",
        success: "bg-success text-success-foreground hover:bg-success/90",
        warning: "bg-warning text-warning-foreground hover:bg-warning/90",
      },
      size: {
        default: "h-10 px-4 py-2",
        sm: "h-9 rounded-md px-3",
        lg: "h-11 rounded-md px-8",
        icon: "h-10 w-10",
      },
      glow: {
        true: "relative after:absolute after:inset-0 after:z-[-1] after:opacity-0 after:transition-opacity hover:after:opacity-100 after:bg-primary/20 after:blur-lg after:rounded-md",
        false: "",
      },
      gradient: {
        true: "bg-gradient-to-r from-primary to-primary-light text-primary-foreground border-none",
        false: "",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
      glow: false,
      gradient: false,
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, glow, gradient, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, glow, gradient, className }))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = "Button";

export { Button, buttonVariants };
```

## Implementation Steps

1. Install required dependencies:
   ```bash
   npm install class-variance-authority clsx tailwind-merge tailwindcss-animate @radix-ui/react-slot
   ```

2. Configure Tailwind CSS
3. Create the theme configuration
4. Implement global CSS styles
5. Create the neoglass effects utility
6. Implement enhanced UI components
7. Document usage patterns for the team
