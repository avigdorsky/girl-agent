# Конфигурация

Эта страница объясняет, какие настройки есть в `girl-agent`, где они лежат и что можно менять руками.

Главная идея простая:

```text
один персонаж = одна папка data/<slug>/
```

Например:

```text
data/arina/
```

Внутри лежат конфиг, характер, речь, память, логи и служебные файлы.

## Где лежит профиль

Структура обычно такая:

```text
data/<slug>/
 config.json
 persona.md
 speech.md
 communication.md
 relationship.md
 hormones.md
 conflict.md
 agenda.md
 memory/
   long-term.md
 daily-life/
   YYYY-MM-DD.md
 stickers/
   library.md
 log/
    YYYY-MM-DD.md
```

Не все файлы обязаны появиться сразу. Некоторые создаются во время работы.

## Главное правило

Если не понимаешь, что менять  меняй только эти файлы:

```text
persona.md
speech.md
communication.md
memory/long-term.md
config.json
```

Остальные лучше трогать только если понимаешь, как они используются.

## config.json

`config.json`  главный файл профиля.

Пример:

```json
{
  "slug": "arina",
  "name": "Арина",
  "age": 22,
  "nationality": "RU",
  "tz": "Europe/Moscow",
  "mode": "userbot",
  "stage": "first-date-done",
  "llm": {
    "presetId": "anthropic",
    "proto": "anthropic",
    "baseURL": "",
    "apiKey": "sk-ant-...",
    "model": "claude-sonnet-4-20250514"
  },
  "telegram": {
    "apiId": 123456,
    "apiHash": "abcdef123456",
    "phone": "+79991234567",
    "sessionString": "..."
  },
  "mcp": [],
  "ownerId": 123456789,
  "createdAt": "2026-05-04T12:00:00.000Z",
  "sleepFrom": 23,
  "sleepTo": 8,
  "nightWakeChance": 0.05,
  "vibe": "short",
  "presencePattern": "burst-checker",
  "personaNotes": "коротко отвечает, не любит давление",
  "busySchedule": []
}
```

## Поля config.json простыми словами

### slug

Имя папки профиля.

```json
"slug": "arina"
```

Значит профиль лежит в:

```text
data/arina/
```

### name

Имя персонажа.

```json
"name": "Арина"
```

### age

Возраст персонажа.

```json
"age": 22
```

Возраст влияет на лексику и реализм. Например, для школьного возраста проект старается использовать `уроки/школа`, а не `пары/универ`.

### nationality

Регион речи.

```json
"nationality": "RU"
```

Возможные значения:

```text
RU
UA
```

### tz

Часовой пояс.

```json
"tz": "Europe/Moscow"
```

Он влияет на:

- сон;
- daily-life;
- логи;
- когда персона онлайн/офлайн.

### mode

Режим Telegram.

```json
"mode": "bot"
```

или:

```json
"mode": "userbot"
```

`bot` проще. `userbot` реалистичнее.

### stage

Стадия отношений.

```json
"stage": "tg-given-cold"
```

Доступные стадии:

```text
met-irl-got-tg
tg-given-cold
tg-given-warming
convinced
first-date-done
dating-early
dating-stable
long-term
dumped
```

Менять можно командой:

```text
:stage first-date-done
```

### llm

Настройки модели.

```json
"llm": {
  "presetId": "anthropic",
  "proto": "anthropic",
  "baseURL": "",
  "apiKey": "sk-ant-...",
  "model": "claude-sonnet-4-20250514"
}
```

- `presetId`  имя провайдера.
- `proto`  протокол API: `openai` или `anthropic`.
- `baseURL`  адрес API, если custom provider.
- `apiKey`  ключ доступа.
- `model`  модель.

### telegram

Для bot mode:

```json
"telegram": {
  "botToken": "123456:ABC"
}
```

Для userbot mode:

```json
"telegram": {
  "apiId": 123456,
  "apiHash": "abcdef",
  "phone": "+79991234567",
  "sessionString": "..."
}
```

Никогда не публикуй `botToken`, `apiHash`, `sessionString`.

### mcp

Дополнительные инструменты.

Пример Exa:

```json
"mcp": [
  {
    "id": "exa",
    "secrets": {
      "EXA_API_KEY": "..."
    }
  }
]
```

### ownerId

Telegram ID основного человека.

```json
"ownerId": 123456789
```

Обычно выставляется автоматически по первому сообщению.

### sleepFrom / sleepTo

Когда персона спит.

```json
"sleepFrom": 23,
"sleepTo": 8
```

Это значит: спит с 23:00 до 08:00 по её часовому поясу.

### nightWakeChance

Шанс, что она проснётся ночью от сообщения.

```json
"nightWakeChance": 0.05
```

`0.05` = 5%.

### vibe

Стиль переписки.

```json
"vibe": "short"
```

Варианты:

- `short`  коротко, сухо, реалистично;
- `warm`  теплее, чаще отвечает, больше деталей.

### presencePattern

Паттерн присутствия.

Пример:

```json
"presencePattern": "burst-checker"
```

Он влияет на то, как часто персона появляется онлайн, читает сообщения и отвечает.

### personaNotes

Дополнительные пожелания, которые использовались при генерации персонажа.

```json
"personaNotes": "не любит длинные монологи, отвечает без эмодзи"
```

### busySchedule

Занятость персонажа.

Пример:

```json
"busySchedule": [
  {
    "days": [1, 2, 3, 4, 5],
    "from": "08:30",
    "to": "14:30",
    "activity": "уроки",
    "social": "school",
    "phoneAvailable": false
  }
]
```

Если `phoneAvailable: false`, она может не читать и не отвечать.

## persona.md

Характер персонажа.

Тут можно писать:

- биографию;
- привычки;
- что любит/не любит;
- как реагирует на давление;
- что считает кринжем;
- что её цепляет.

Пример:

```markdown
# Арина

Она не любит, когда ей пишут десять сообщений подряд.
Если человек давит, отвечает холоднее.
Сначала держит дистанцию, но может оттаять от нормального живого общения.
```

## speech.md

Манера речи.

Тут описывается, как она пишет в Telegram.

Пример:

```markdown
# Речь

Пишет коротко.
Редко ставит точки.
Может писать "пхпх", "ну", "та не".
Не использует "конечно", "я понимаю", "рада помочь".
```

## communication.md

Границы общения.

Пример:

```markdown
# Границы

Не отвечает на давление.
Не поддерживает скучный допрос.
Если ей неприятно, может игнорировать.
Если человек токсичный, может заблокировать.
```

## relationship.md

Текущие отношения и score.

Внутри хранится стадия и счётчики:

```text
interest
trust
attraction
annoyance
cringe
```

Их лучше менять через команды:

```text
:status
:reset
:stage <id>
```

## memory/long-term.md

Долгосрочная память.

Туда попадают важные факты и эмоциональные следы.

Пример:

```markdown
## 2026-05-04T13:20
- Он несколько раз писал одно и то же, она раздражалась.
- Он предложил встретиться, но сделал это слишком резко.
```

Можно редактировать руками.

## daily-life/

Папка с жизнью на день.

Пример:

```text
daily-life/2026-05-04.md
```

Daily-life влияет на:

- занята ли она;
- может ли сидеть в телефоне;
- почему отвечает позже;
- что может сказать о своём дне.

## agenda.md

Планы и проактивные сообщения.

Например:

- напомнить о разговоре;
- написать первой;
- вернуться к теме позже.

## log/

Логи диалога.

Пример:

```text
log/2026-05-04.md
```

Формат:

```text
[2026-05-04T12:00:00.000Z] он(123): привет
  -> она: ну привет
```

## CLI флаги

Посмотреть помощь:

```powershell
npm run dev -- --help
```

Основные флаги:

```text
--profile=<slug>
--mode=bot|userbot
--token=<bot_token>
--api-id=<n>
--api-hash=<hash>
--phone=<+7...>
--api-preset=<id>
--base-url=<url>
--proto=openai|anthropic
--model=<model>
--api-key=<key>
--name=<имя>
--age=<n>
--persona-notes=<text>
--nationality=RU|UA
--tz=<value>
--stage=<id>
--vibe=short|warm
--mcp=exa:KEY
--practice
--list
--reset
--help
```

## Примеры запуска

Список профилей:

```powershell
npm run dev -- --list
```

Запуск профиля:

```powershell
npm run dev -- --profile=arina
```

Сброс стадии профиля при запуске:

```powershell
npm run dev -- --profile=arina --reset
```

Проверка типов:

```powershell
npm run typecheck
```

## Что можно менять руками

Безопасно:

- `persona.md`
- `speech.md`
- `communication.md`
- `memory/long-term.md`
- `busySchedule` в `config.json`
- `sleepFrom`, `sleepTo`, `vibe`, `presencePattern`

Осторожно:

- `telegram`
- `llm`
- `ownerId`
- `relationship.md`
- `agenda.md`

Не публиковать:

- `apiKey`
- `botToken`
- `apiHash`
- `sessionString`

## Следующий шаг

Прочитай [dashboard.md](dashboard.md), чтобы понять команды во время работы.
