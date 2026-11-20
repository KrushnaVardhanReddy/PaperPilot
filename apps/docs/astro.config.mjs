import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://docs.usepaperpilot.com',
  server: {
    port: 4321,
    host: true,
  },
  integrations: [
    starlight({
      title: 'PaperPilot Documentation',
      description: 'Official guides, 50+ tool reference, embed widget, and developer API for PaperPilot.',
      social: {
        github: 'https://github.com/KrushnaVardhanReddy/PaperPilot',
      },
      components: {
        Header: './src/components/VersionSelectHeader.astro',
      },
      sidebar: [
        {
          label: 'Getting Started',
          autogenerate: { directory: 'getting-started' },
        },
        {
          label: 'Embed.js Widget Integration',
          autogenerate: { directory: 'embed' },
        },
        {
          label: 'Master Tools Reference',
          items: [
            { label: 'Tools Handbook (All-in-One)', slug: 'reference/tools-handbook' },
            {
              label: 'All Tools Index (100 Tools)',
              autogenerate: { directory: 'reference/tools' },
            },
          ],
        },
        {
          label: 'Developer Gateway & AI Agents',
          autogenerate: { directory: 'developer' },
        },
        {
          label: 'Architecture & Performance',
          autogenerate: { directory: 'architecture' },
        },
      ],
      customCss: [],
    }),
    sitemap(),
  ],
});
