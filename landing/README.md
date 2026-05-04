# landing

Standalone awwwards-style landing page for the `girl-agent` repo.

## preview

```bash
# from project root
npx serve landing
# or just open landing/index.html in a browser
```

## deploy

Drop the `landing/` folder onto:
- **GitHub Pages** — set Pages source to `/landing`
- **Netlify / Vercel** — point root to `landing/`, no build step
- **Cloudflare Pages** — same

No bundler. Single `index.html`, fonts via Fontshare CDN, JS uses native `IntersectionObserver`.

## edit

Update the GitHub URL placeholder (`https://github.com`) in `index.html` to your real repo URL — there are 4 occurrences.
