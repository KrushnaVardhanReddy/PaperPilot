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
              label: 'Image Intelligence & GIF Studio (v0.8.x New)',
              items: [
                { label: 'img_deskew', slug: 'reference/tools/img_deskew' },
                { label: 'img_crop_to_content', slug: 'reference/tools/img_crop_to_content' },
                { label: 'img_optimize_for_llm', slug: 'reference/tools/img_optimize_for_llm' },
                { label: 'img_redact', slug: 'reference/tools/img_redact' },
                { label: 'img_convert', slug: 'reference/tools/img_convert' },
                { label: 'images_to_gif', slug: 'reference/tools/images_to_gif' },
                { label: 'pdf_to_gif', slug: 'reference/tools/pdf_to_gif' },
                { label: 'showcase_gif', slug: 'reference/tools/showcase_gif' },
              ],
            },
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
          items: [
            { label: 'Architecture Overview', slug: 'architecture/zero-copy-streaming' },
            { label: 'System & Interface Benchmarks', slug: 'architecture/benchmarks' },
          ],
        },
      ],
      customCss: [],
    }),
    sitemap(),
  ],
});
