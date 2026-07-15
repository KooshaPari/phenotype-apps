import { defineConfig } from 'vitepress'
import { crossProjectLinks } from './plugins/cross-project-links'

export default defineConfig({
  title: 'KDesktopVirt',
  description: 'Desktop-tier device automation and virtualization for the Phenotype eco-011 initiative.',
  appearance: true,
  lastUpdated: true,
  base: '/KDesktopVirt/',
  ignoreDeadLinks: true,

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'ADRs', link: '/adr/' },
      { text: 'Reference', link: '/reference/' },
      { text: 'Research', link: '/research/' },
    ],
    sidebar: [
      {
        text: 'Architecture',
        items: [
          { text: 'ADRs', link: '/adr/' },
          { text: 'Reference', link: '/reference/' },
        ],
      },
      {
        text: 'Operations',
        items: [
          { text: 'Deployment', link: '/DEPLOYMENT' },
          { text: 'CI Fixes', link: '/CI-FIXES' },
          { text: 'Validation', link: '/VALIDATION' },
        ],
      },
      {
        text: 'Audio / Video',
        items: [
          { text: 'Audio/Video System', link: '/AUDIO_VIDEO_SYSTEM' },
          { text: 'Cross-Platform Audio', link: '/CROSS-PLATFORM-AUDIO-ENHANCEMENT' },
          { text: 'Platform Audio Summary', link: '/PLATFORM-AUDIO-SUMMARY' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/KDesktopVirt' },
    ],
    search: { provider: 'local' },
    editLink: {
      pattern: 'https://github.com/KooshaPari/KDesktopVirt/edit/main/docs/:path',
    },
  },

  markdown: {
    config: (md) => {
      md.use(crossProjectLinks)
    },
  },

  vite: {
    build: {
      rollupOptions: {
        output: {
          manualChunks: undefined,
        },
      },
    },
  },
})
