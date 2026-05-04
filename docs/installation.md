# Установка

Эта инструкция написана максимально просто. Если ты вообще впервые запускаешь проект на Node.js, просто иди сверху вниз и не пропускай шаги.

## Что ты в итоге получишь

После установки у тебя будет:

1. Папка проекта `girl-agent`.
2. Установленные зависимости Node.js.
3. Созданный профиль персонажа в `data/<slug>/`.
4. Подключение к Telegram через bot mode или userbot mode.
5. Подключение к LLM-провайдеру.
6. Открытый TUI dashboard, где видно сообщения, статус и команды.

## Перед началом

Нужно подготовить 3 вещи.

### 1. Node.js

Нужен Node.js версии 20 или выше.

Проверь:

```powershell
node --version
```

Если видишь что-то вроде:

```text
v20.11.0
```

или выше  всё нормально.

Если команды `node` нет, установи Node.js LTS с официального сайта:

```text
https://nodejs.org/
```

Потом закрой терминал, открой заново и снова проверь `node --version`.

### 2. Telegram способ подключения

Есть два режима.

#### Bot mode

Это проще.

Ты создаёшь бота через `@BotFather`, получаешь токен и вставляешь его в wizard.

Подходит если:

- хочешь быстро проверить проект;
- не хочешь логиниться в Telegram аккаунт;
- нормально, что в Telegram будет видно, что это бот.

Минусы:

- это именно бот;
- меньше реализма;
- нет полноценного userbot-поведения как у обычного аккаунта.

#### Userbot mode

Это реалистичнее.

Проект входит в Telegram как обычный аккаунт через MTProto/GramJS.

Подходит если:

- хочешь максимум реализма;
- нужны read receipts, реакции, typing и поведение обычного пользователя;
- готов получить `api_id` и `api_hash` на `my.telegram.org`.

Минусы:

- сложнее настройка;
- лучше использовать отдельный тестовый аккаунт;
- нельзя публиковать `sessionString`.

### 3. API ключ LLM

Нужен ключ от провайдера модели.

Поддерживаются пресеты вроде:

- Anthropic
- OpenAI
- OpenRouter
- Groq
- DeepSeek
- custom OpenAI-compatible API

Самый простой путь  взять один API-ключ у провайдера и вставить его в wizard.

## Шаг 1. Скачать проект

Открой PowerShell в папке, где хочешь хранить проект.

Выполни:

```powershell
git clone <repo-url> girl-agent
```

Зайди в папку:

```powershell
cd girl-agent
```

Если проект уже скачан, просто перейди в его папку.

## Шаг 2. Поставить зависимости

В папке проекта выполни:

```powershell
npm install
```

Это скачает библиотеки:

- Telegram Bot API клиент;
- GramJS для userbot mode;
- Ink для TUI;
- LLM SDK;
- TypeScript инструменты.

Если установка прошла без красных ошибок  идём дальше.

## Шаг 3. Запустить wizard

Выполни:

```powershell
npm run dev
```

Если профилей ещё нет, откроется wizard.

Wizard  это пошаговая настройка прямо в терминале. Он задаёт вопросы, а ты отвечаешь.

## Шаг 4. Выбрать режим Telegram

Wizard спросит:

```text
Bot  обычный @BotFather бот
Userbot  твой реальный аккаунт
```

### Если выбрал Bot

1. Открой Telegram.
2. Найди `@BotFather`.
3. Напиши ему `/newbot`.
4. Выбери имя бота.
5. Выбери username бота, который заканчивается на `bot`.
6. BotFather даст токен вида:

```text
123456789:AAExampleTokenExampleToken
```

7. Вставь этот токен в wizard.

### Если выбрал Userbot

1. Открой:

```text
https://my.telegram.org
```

2. Войди по номеру телефона.
3. Открой `API development tools`.
4. Создай приложение.
5. Скопируй:

```text
api_id
api_hash
```

6. Вставь их в wizard.
7. Введи телефон в международном формате:

```text
+79991234567
```

8. Telegram пришлёт код.
9. Введи код в wizard.
10. Если включён 2FA пароль, введи пароль.

После этого wizard сохранит `sessionString` в `data/<slug>/config.json`.

## Шаг 5. Выбрать LLM провайдера

Wizard предложит провайдера.

Выбери тот, где у тебя есть API-ключ.

Дальше:

1. Выбери модель.
2. Вставь API-ключ.
3. Если используешь custom endpoint, укажи `baseURL` и протокол.

Важно: API-ключ хранится в `config.json`. Не публикуй этот файл.

## Шаг 6. Настроить персонажа

Wizard спросит:

1. Национальность/регион.
2. Имя.
3. Возраст.
4. Часовой пояс.
5. Сон.
6. Стиль общения.
7. Стадию отношений.
8. Дополнительные пожелания к персоне.

### Стиль общения

`short`  более реалистично-холодный стиль:

- короткие ответы;
- больше игнора;
- меньше объяснений;
- больше похоже на обычную переписку.

`warm`  теплее:

- чаще отвечает;
- может писать подробнее;
- чаще поддерживает диалог.

### Дополнительные пожелания

Можно написать обычным текстом:

```text
учится в школе, немного язвительная, не любит длинные сообщения, общается без эмодзи
```

Wizard использует это при генерации:

- `persona.md`
- `speech.md`
- `communication.md`
- busy schedule

## Шаг 7. Дождаться генерации файлов

После ответов wizard создаст папку:

```text
data/<slug>/
```

Там появятся основные файлы:

```text
config.json
persona.md
speech.md
communication.md
relationship.md
memory/long-term.md
agenda.md
log/
```

Если генерация занимает время  это нормально. Модель пишет профиль персонажа.

## Шаг 8. Запустить live режим

После wizard runtime стартует сам.

Ты увидишь dashboard:

```text
Арина  userbot  
stage: Сходили один раз
interest ...
trust ...
attraction ...
annoyance ...
cringe ...
```

Теперь агент слушает Telegram.

## Шаг 9. Написать персонажу

Открой Telegram и напиши аккаунту/боту.

Если всё работает, в dashboard появится входящее сообщение:

```text
 привет
```

Дальше агент решит:

- ответить;
- проигнорировать;
- прочитать и промолчать;
- поставить реакцию;
- ответить позже.

Это нормально. Он не обязан отвечать на каждое сообщение.

## Запуск уже созданного профиля

Если профиль уже есть:

```powershell
npm run dev -- --profile=<slug>
```

Пример:

```powershell
npm run dev -- --profile=arina
```

Если профиль один, проект может загрузить его автоматически.

## Посмотреть список профилей

```powershell
npm run dev -- --list
```

## Headless setup без wizard

Можно создать профиль флагами.

Bot mode:

```powershell
npm run dev -- --profile=anya --mode=bot --token=123456:ABC --api-preset=anthropic --model=claude-sonnet-4-20250514 --api-key=sk-ant-... --name=Аня --age=22 --nationality=RU --tz=Europe/Moscow --stage=tg-given-cold --vibe=short
```

Userbot mode:

```powershell
npm run dev -- --profile=anya --mode=userbot --api-id=123456 --api-hash=abcdef123456 --phone=+79991234567 --api-preset=anthropic --model=claude-sonnet-4-20250514 --api-key=sk-ant-... --name=Аня --age=22 --nationality=RU --tz=Europe/Moscow --stage=tg-given-cold --persona-notes="дерзкая, занятая, отвечает коротко"
```

## Что делать если не работает

1. Открой [troubleshooting.md](troubleshooting.md).
2. Проверь `npm run typecheck`.
3. Проверь `data/<slug>/config.json`.
4. Проверь, что ключи не пустые.
5. Проверь, что Telegram токен или userbot credentials правильные.

## Следующий шаг

После установки прочитай:

- [configuration.md](configuration.md)
- [dashboard.md](dashboard.md)
- [features.md](features.md)
