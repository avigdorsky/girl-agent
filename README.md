![girl-agent banner](assets/final.png)

[website]: https://girl-agent.dev
[docs]: docs/index.html

**[website]** &nbsp;·&nbsp; **[docs]**

---

Она не отвечает на каждое сообщение. Иногда читает и молчит. Иногда ставит реакцию. Иногда отвечает через час, потому что была занята или просто не хотела.

Это не баг. Так задумано.

`girl-agent` — ИИ-девушка, которая ведёт себя в переписке как человек. Со сном, настроением, расписанием, памятью и характером. Без "конечно, я понимаю" и ChatGPT-повадок.

---

## Быстрый старт

```powershell
git clone https://github.com/TheSashaDev/girl-agent.git
cd girl-agent
npm install
npm run dev
```

Wizard задаст пару вопросов — имя, возраст, Telegram-подключение, LLM-ключ. Всё.

Если профиль уже есть:

```powershell
npm run dev -- --profile=arina
```

---

## Что под капотом

Поведение собирается из нескольких слоёв, а не из одного промпта.

**Она не всегда онлайн.** Паттерн присутствия зависит от персонажа: кто-то в телефоне круглые сутки, кто-то заходит раз в час, кто-то только вечером.

**Ночью спит.** Можно разбудить через `:wake`, но без команды шанс ответа низкий.

**Расписание дня.** У каждого дня есть расписание: пары, работа, дорога, свободное время. Если она на занятиях, телефон может быть недоступен.

**Отношения.** Пять счётчиков: интерес, доверие, привлекательность, раздражение, неловкость. Меняются от каждого диалога. Высокое раздражение — чаще игнор и холод. Высокая неловкость — сухие ответы и отстранение.

**Стадии сближения.** Отношения проходят стадии: от "дала тг, но холодная" до "давно вместе". Стадия влияет на тепло, флирт, длину ответов и допустимость реакций.

**Конфликты.** Если давить, спамить или нарушать границы — включается конфликт. Она может замолчать на часы или дни.

**Память.** Важные события пишутся в `long-term.md` и всплывают в будущих диалогах.

**Anti-AI.** Промпт запрещает markdown, "конечно", "я понимаю", эмодзи-ряды, вопросы в конце сообщений и всё, что палит ChatGPT.

**Userbot mode.** Настоящий Telegram-аккаунт через MTProto. Умеет читать сообщения, ставить реакции, печатать, удалять и редактировать. Выглядит как живой человек, а не как бот.

---

## Почему не просто GPTs или промпт

Вариантов сделать "девушку в Telegram" несколько — от костыльных до полноценных. Разберём, что есть и где дыры.

### ChatGPT GPTs

**Как это работает:** Кастомный бот внутри ChatGPT с system prompt. В GPT Builder задаёшь имя, описание, инструкции — и всё. Логика поведения = промпт.

**Что упущено:**
- GPTs вообще не используют память. Каждая сессия начинается с нуля — даже если у тебя включена память в обычном ChatGPT, Custom GPTs её не видят.
- Нет userbot mode — это не Telegram, это веб-интерфейс ChatGPT.
- Нет реакций, read receipts, печати, редактирования сообщений.
- Нет расписания, сна, presence — бот всегда "онлайн".
- Память ограничена контекстным окном — длинные диалоги обрезаются, и бот забывает что было раньше.

**Итог:** Это не персонаж, это просто чат-бот с кастомным промптом. Нет состояния между сессиями, нет реалистичного поведения.

### OpenClaw + prompt (markdown-файлы)

**Как это работает:** OpenClaw — фреймворк для AI-ассистентов. Личность задаётся через markdown-файлы: SOUL.md (характер), IDENTITY.md (кто она), USER.md (кто ты). Есть Telegram bridge через GramJS (MTProto), memory через markdown-файлы, system prompts per-chat.

**Технические детали:**
- MTProto через GramJS — настоящий Telegram userbot.
- Memory: multi-turn context с configurable history depth (default 20), persisted to disk как JSON файлы в `~/.openclaw/telegram-userbot/`.
- System prompts: default system prompt + per-chat override.
- Auto-trimming: старые сообщения удаляются когда лимит превышен.
- Per-chat isolation: отдельная история для каждого DM/группы.
- Human-like reply delay: configurable pause + typing indicator.

**Что упущено:**
- Нет реализм-модулей: presence, sleep, conflict, daily-life, relationship stages — всё зависит только от промпта.
- Нет agenda — бот не планирует действия, не живёт своей жизнью.
- Память — это просто история сообщений, нет выделенного long-term storage для важных событий.
- Нет relationship score — бот не понимает стадию отношений (stranger → convinced → close).
- Нет conflict system — если давить, бот не замолчит, не изменит поведение.

**Итог:** Это хороший bridge для Telegram, но это не персонаж-движок. Поведение = промпт + история сообщений, без состояния.

### HeatherBot

**Как это работает:** Локальный Telegram userbot (MTProto via Telethon), persona в YAML-файле, 4-слойная персонализация памяти, 17 kink-specific personality overlays. ~10K строк Python кода.

**Технические детали:**
- Local LLM: llama-server (port 1234, 12B params), Ollama для image analysis (port 11434), ComfyUI для генерации картинок (port 8188).
- Persona YAML: identity, personality, communication, sexual, ai_behavior, prompts — всё в одном файле.
- Per-user memory: 4-layer personalization (kink categories 21 шт, memorable moments). Каждый пользователь gets свой профиль который эволюционирует.
- 17 kink-specific overlays в heather_kink_personas.yaml — автоматически применяются на основе поведения юзера.
- Postprocessing: 7-stage response filter + human imperfections.
- MTProto userbot через Telethon — выглядит как реальный юзер.

**Что упущено:**
- Слишком специфично под NSFW — 17 kink overlays, sexual boundaries, breeding/CNC fantasies. Не для обычных отношений.
- Сложно настроить — нужно локально поднять llama-server, Ollama, ComfyUI, разобраться с YAML.
- Требует мощного GPU — 12B модель локально, это не на любом ноутбуке взлетит.
- Нет presence/sleep/conflict как отдельных модулей — всё в YAML и промптах.

**Итог:** Это мощное решение, но узкое — заточено под NSFW и требует тяжёлой инфраструктуры. Не для обычного пользователя.

### Character.AI

**Как это работает:** Закрытый сервис для AI-переписки. Персоны создаются через UI, поведение = prompt engineering + session-level memory buffers.

**Технические детали:**
- Session-level memory buffer: 10-15 turns в активном контексте.
- Summary embeddings для тематической continuity между сессиями.
- Persona consistency через refined prompt engineering.
- Memory limit: 3000-4000 tokens — persona и сообщения делят этот лимит.
- Когда история растёт, persona обрезается первой — бот забывает детали персонажа.

**Что упущено:**
- Нет Telegram — это только веб-интерфейс Character.AI.
- Нет контроля — всё на их серверах, нет локальности, нет кастомного поведения.
- Нет persistent identity modules — память сбрасывается между сессиями, только summary embeddings.
- Нет multimodal integration — нет аватаров, голоса, анимации.
- Нет dynamic personality re-writing — персонаж не эволюционирует.
- Memory ограничена — persona влазит только в начало, потом обрезается.

**Итог:** Это закрытый сервис с ограниченной памятью и без Telegram. Нет контроля над тем, как это работает.

### girl-agent

**Как это работает:** Движок с несколькими слоями состояния: presence, sleep, daily-life, relationship stages, conflict, memory, anti-AI. Userbot mode через MTProto.

**Технические детали:**
- Presence: паттерны присутствия (частота проверок, время офлайн, вероятность ответа офлайн).
- Sleep: время сна, night wake chance.
- Daily-life: расписание, занятость, приоритеты.
- Relationship stages: stranger → convinced → close → intimate → bonded.
- Relationship score: interest, trust, attraction, annoyance, cringe.
- Conflict: если давить/спамить/нарушать границы — включается конфликт, может замолчать на часы/дни.
- Memory: важные события пишутся в long-term.md, всплывают в будущих диалогах.
- Anti-AI: промпт запрещает markdown, "конечно", "я понимаю", эмодзи-ряды — всё, что палит ChatGPT.
- Userbot mode через MTProto: умеет читать, ставить реакции, печатать, удалять, редактировать.
- Agenda: бот планирует действия, живёт своей жизнью, не просто отвечает на сообщения.

**Что упущено:**
- — (нет явных упущений)

**Итог:** Это не просто промпт. Это движок с несколькими слоями решения: presence, sleep, conflict, relationship score, agenda. Поведение собирается из состояния, а не из текстовых инструкций.

---

## Безопасность

Не публикуй `data/`, `config.json`, `sessionString` и API-ключи. Для userbot mode используй отдельный тестовый аккаунт.

---

## Лицензия

### English

This project is source-available for personal testing, evaluation, and contributions.

You may clone and run it locally, open issues, and submit pull requests.

Commercial use, paid hosting, resale, public competing clones, and using the code inside commercial products are not allowed without written permission.

Full license text: [LICENSE](./LICENSE).

### Русский

Этот проект является source-available: исходный код открыт для личного тестирования, оценки и предложений по улучшению.

Вы можете клонировать проект, запускать его локально, создавать issues и отправлять pull requests.

Коммерческое использование, платный хостинг, перепродажа, публичные конкурирующие клоны и использование кода внутри коммерческих продуктов запрещены без письменного разрешения.

Полный текст лицензии: [LICENSE](./LICENSE).

### Українська

Цей проєкт є source-available: вихідний код відкритий для особистого тестування, оцінки та пропозицій щодо покращення.

Ви можете клонувати проєкт, запускати його локально, створювати issues і надсилати pull requests.

Комерційне використання, платний хостинг, перепродаж, публічні конкуруючі клони та використання коду всередині комерційних продуктів заборонені без письмового дозволу.

Повний текст ліцензії: [LICENSE](./LICENSE).
