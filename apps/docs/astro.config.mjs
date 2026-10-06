import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://docs.usepaperpilot.com',
  integrations: [
    starlight({
      title: 'PaperPilot Documentation',
      description: 'Official guides, 44-tool reference, embed widget, and developer API for PaperPilot.',
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
          label: 'Master 44-Tool Reference',
          autogenerate: { directory: 'reference' },
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
