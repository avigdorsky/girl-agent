# contributing

```bash
git clone https://github.com/TheSashaDev/girl-agent && cd girl-agent
npm i && npm run dev
```

перед pr — `npm run typecheck` и `npm run build`. не коммить `data/`, `dist/`, `.env`.

легко добавить полезное в:

- `src/presets/llm.ts` — llm-пресеты
- `src/presets/mcp.ts` + `src/mcp/` — mcp-интеграции
- `src/presets/names.ts` — региональные имена
- `src/engine/` — стадии, behavior-tick, гормональная модель

nsfw не принимается. векторные бд и rag не принимаются. крупный рефакторинг — сначала issue.

pr идёт под mit.
