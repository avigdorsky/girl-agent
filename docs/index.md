# Документация girl-agent

Добро пожаловать в документацию `girl-agent`.

Если ты здесь впервые, читай страницы в таком порядке:

1. [Установка](installation.md)
2. [Конфигурация](configuration.md)
3. [Дашборд и команды](dashboard.md)
4. [Реализм-модули](realism.md)
5. [Фичи](features.md)
6. [Troubleshooting](troubleshooting.md)

## Что это за проект

`girl-agent`  это Telegram-персона, которая живёт в переписке как человек.

Она не просто отвечает на каждое сообщение. Она может:

- быть офлайн;
- спать;
- быть занятой;
- читать и молчать;
- ставить реакции;
- отвечать позже;
- раздражаться;
- запоминать важные вещи;
- менять тон в зависимости от отношений.

## Для кого эта документация

Для трёх типов людей:

## 1. Для новичка

Если ты просто хочешь запустить проект, начни с [installation.md](installation.md).

Там инструкция написана пошагово:

```text
скачал -> установил -> запустил wizard -> вставил ключи -> написал в Telegram
```

## 2. Для того, кто настраивает персонажа

Если проект уже работает, но персона пишет не так, как надо, читай:

- [configuration.md](configuration.md)
- [realism.md](realism.md)

Главные файлы для настройки:

```text
data/<slug>/persona.md
data/<slug>/speech.md
data/<slug>/communication.md
data/<slug>/memory/long-term.md
data/<slug>/config.json
```

## 3. Для разработчика

Если хочешь менять код, смотри:

```text
src/engine/runtime.ts
src/engine/behavior-tick.ts
src/engine/prompt.ts
src/engine/presence.ts
src/telegram/userbot.ts
src/telegram/bot.ts
```

## Самая короткая установка

```powershell
git clone <repo-url> girl-agent
cd girl-agent
npm install
npm run dev
```

## Что важно понимать сразу

## Агент не обязан отвечать

Если он молчит  это не всегда баг.

Он мог решить:

- она спит;
- она занята;
- сообщение скучное;
- человек раздражает;
- лучше прочитать и не отвечать;
- ответить позже.

Смотри команду:

```text
:why
```

## Userbot реалистичнее, но сложнее

`bot` mode проще.

`userbot` mode реалистичнее:

- read receipts;
- typing;
- реакции;
- блокировка;
- редактирование;
- удаление сообщений.

Но userbot требует:

- отдельный Telegram аккаунт;
- `api_id`;
- `api_hash`;
- вход по телефону.

## Не публикуй секреты

Нельзя публиковать:

```text
data/
config.json
sessionString
apiKey
botToken
apiHash
```

## Основные команды

```text
:status
:why
:reset
:stage <id>
:wake
:pause
:resume
:read
:log
:quit
```

Подробно  в [dashboard.md](dashboard.md).

## Где что лежит

```text
data/<slug>/
 config.json              настройки
 persona.md               характер
 speech.md                речь
 communication.md         границы
 relationship.md          стадия и score
 memory/long-term.md      память
 daily-life/              день персонажа
 agenda.md                планы
 log/                     логи диалогов
```

## Если что-то сломалось

Сначала:

```powershell
npm run typecheck
```

Потом в dashboard:

```text
:status
:why
:log
```

Если не помогло  [troubleshooting.md](troubleshooting.md).
