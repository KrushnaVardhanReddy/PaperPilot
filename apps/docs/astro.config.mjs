import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

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
          label: 'Master 50+ Tools Reference',
          items: [
            { label: '50 Tools Handbook (All-in-One)', slug: 'reference/50-tools-handbook' },
            {
              label: 'All 50 Tools Index',
              autogenerate: { directory: 'reference/tools' },
            },
          ],
        },
        {
          label: 'Developer Gateway & AI Agents',
          autogenerate: { directory: 'developer' },
        },
      ],
      customCss: [],
    }),
  ],
});
