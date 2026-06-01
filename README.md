# CRM

Система управления проектами для строительных/инженерных организаций. Позволяет вести проекты, разбивать их на этапы, отслеживать стоимость, дедлайны, подтверждения ГИП и оплаты, прикреплять файлы и акты, а также вести журнал комментариев по каждому этапу.

## Стек

**Backend** — Rust, Actix-web 4, PostgreSQL (sqlx), MinIO (S3-совместимый объектный storage), JWT (access + refresh токены), bcrypt, lettre (SMTP)

**Frontend** — React 18, TypeScript, Redux Toolkit, Vite, SCSS

**Инфраструктура** — Docker Compose, Caddy (reverse proxy + TLS), GitHub Container Registry

---

## Архитектура (Elegant Objects)

Проект строится по принципам EO: объекты владеют поведением и представляют себя сами, без геттеров/сеттеров, статических методов, DTO и наследования реализации.

```
Backend/src/
├── endpoint/          # HTTP-обработчики, по одному файлу на маршрут
│   ├── auth/          # login, refresh, logout
│   ├── users/         # регистрация и профиль
│   ├── invites/       # создание инвайтов
│   ├── projects/      # проекты и этапы
│   └── admin/         # служебные эндпойнты
│
├── model/
│   ├── project/       # Project, Stage, Act, Attachment, Comment и их Detailed-варианты
│   ├── user/          # User, Role, Invite и их представления
│   ├── session/       # AccessToken, RefreshToken, Claims
│   ├── credential/    # Username, Password, хэшированные и валидированные обёртки
│   ├── notification/  # BurningDeadline, DeadlineDigest, QueuedNotification
│   ├── schedule/      # Timetable, Schedule, TimeOfDay, PollInterval
│   └── task/          # объекты-операции: инкапсулируют DB-запросы и side-эффекты
│
├── middleware/        # JwtMiddleware, login_governor (rate limiting)
├── state.rs           # AppState: PgPool, Storage, Mailer
├── storage.rs         # обёртка над aws-sdk-s3 (MinIO)
├── mail.rs            # Mailer через lettre/SMTP
├── jwt.rs             # подпись и верификация токенов
└── routes.rs          # регистрация всех маршрутов

Backend/migrations/    # SQL-миграции (sqlx migrate)

Frontend/src/
├── App.tsx
└── main.tsx
```

**Фоновые задачи** (запускаются при старте сервера):
- **12:00 ежедневно** — рассылка дайджеста дедлайнов пользователям с включёнными уведомлениями
- **каждые 2 минуты** — отправка накопленных уведомлений из очереди

---

## API Endpoints

Все маршруты начинаются с `/api`. Маршруты, требующие авторизации, отмечены `[auth]`.

### Аутентификация

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `POST` | `/auth/login` | Вход по username/password. Rate-limited. Возвращает access и refresh токены |
| `POST` | `/auth/refresh` | Обновление access-токена по refresh-токену из cookie |
| `POST` | `/auth/logout` | Отзыв refresh-токена |

### Пользователи

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `POST` | `/users` | Регистрация по инвайт-токену |
| `GET` | `/users/me` | `[auth]` Текущий пользователь |
| `PATCH` | `/users/me/username` | `[auth]` Смена имени пользователя |
| `PATCH` | `/users/me/password` | `[auth]` Смена пароля |
| `PATCH` | `/users/me/email` | `[auth]` Смена email |
| `PATCH` | `/users/me/roles` | `[auth]` Обновление ролей |
| `PATCH` | `/users/me/notifications` | `[auth]` Включение/выключение email-уведомлений |

**Роли:** `gip`, `lawyer`, `accountant`

### Инвайты

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `POST` | `/invites` | `[auth]` Создание ссылки-приглашения |

### Проекты

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `GET` | `/projects` | `[auth]` Список проектов |
| `POST` | `/projects` | `[auth]` Создание проекта |
| `GET` | `/projects/deadlines` | `[auth]` Этапы с горящими дедлайнами |
| `DELETE` | `/projects/{project_id}` | `[auth]` Удаление проекта |
| `PATCH` | `/projects/{project_id}/title` | `[auth]` Переименование проекта |

### Этапы

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `GET` | `/projects/{project_id}/stages` | `[auth]` Список этапов проекта |
| `POST` | `/projects/{project_id}/stages` | `[auth]` Добавление этапа в конец |
| `GET` | `/projects/{project_id}/stages/{stage_id}` | `[auth]` Этап по позиции |
| `POST` | `/projects/{project_id}/stages/{stage_id}` | `[auth]` Вставка этапа перед указанным |
| `DELETE` | `/projects/{project_id}/stages/{stage_id}` | `[auth]` Удаление этапа |
| `PATCH` | `/projects/{project_id}/stages/{stage_id}/title` | `[auth]` Переименование этапа |
| `PATCH` | `/projects/{project_id}/stages/{stage_id}/deadline` | `[auth]` Установка дедлайна |
| `PATCH` | `/projects/{project_id}/stages/{stage_id}/cost` | `[auth]` Установка стоимости |
| `PATCH` | `/projects/{project_id}/stages/{stage_id}/gip-confirmed` | `[auth]` Подтверждение ГИП |
| `PATCH` | `/projects/{project_id}/stages/{stage_id}/payment-confirmed` | `[auth]` Подтверждение оплаты |

### Акты

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `GET` | `/projects/{project_id}/stages/{stage_id}/act` | `[auth]` Список актов этапа |
| `POST` | `/projects/{project_id}/stages/{stage_id}/act` | `[auth]` Загрузка акта (multipart) |
| `DELETE` | `/projects/{project_id}/stages/{stage_id}/act/{act_id}` | `[auth]` Удаление акта |
| `GET` | `/projects/{project_id}/stages/{stage_id}/act/{act_id}/download` | `[auth]` Скачивание акта |

### Вложения

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `GET` | `/projects/{project_id}/stages/{stage_id}/attachments` | `[auth]` Список вложений |
| `POST` | `/projects/{project_id}/stages/{stage_id}/attachments` | `[auth]` Загрузка файла (multipart, до 50 МБ) |
| `GET` | `/projects/{project_id}/stages/{stage_id}/attachments/{attachment_id}/download` | `[auth]` Скачивание файла |
| `DELETE` | `/projects/{project_id}/stages/{stage_id}/attachments/{attachment_id}` | `[auth]` Удаление файла |

### Комментарии

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `GET` | `/projects/{project_id}/stages/{stage_id}/comments` | `[auth]` Список комментариев |
| `POST` | `/projects/{project_id}/stages/{stage_id}/comments` | `[auth]` Добавление комментария |
| `DELETE` | `/projects/{project_id}/stages/{stage_id}/comments/{comment_id}` | `[auth]` Удаление комментария |

### Администрирование

| Метод | Маршрут | Описание |
|-------|---------|----------|
| `POST` | `/admin/digest` | `[auth]` Принудительная рассылка дайджеста дедлайнов |

---

## Переменные окружения

Скопируйте `.env.example` в `Backend/.env` и заполните значения.

| Переменная | Описание |
|------------|----------|
| `DATABASE_URL` | PostgreSQL connection string: `postgres://user:password@host:5432/crm` |
| `JWT_SECRET` | Секрет для подписи JWT-токенов |
| `MINIO_ENDPOINT` | URL MinIO: `http://localhost:9000` (dev) или `http://minio:9000` (prod) |
| `MINIO_ACCESS_KEY` | Access key MinIO |
| `MINIO_SECRET_KEY` | Secret key MinIO |
| `SMTP_HOST` | SMTP-хост для отправки почты |
| `SMTP_PORT` | SMTP-порт (`465` — TLS, любой другой — без TLS) |
| `SMTP_USERNAME` | SMTP-логин |
| `SMTP_PASSWORD` | SMTP-пароль |
| `MAIL_FROM` | Адрес отправителя, например `CRM <noreply@example.com>` |

Для prod-деплоя дополнительно нужны (используются в `docker-compose.prod.yml`):

| Переменная | Описание |
|------------|----------|
| `DOMAIN` | Домен сайта, например `dailycrm.mooo.com` |
| `POSTGRES_USER` | Пользователь PostgreSQL |
| `POSTGRES_PASSWORD` | Пароль PostgreSQL |
| `GITHUB_REPOSITORY_OWNER` | GitHub-логин для pull образов из GHCR |

---

## Локальный запуск

### Зависимости

- Rust (edition 2024)
- Node.js 18+
- Docker + Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx): `cargo install sqlx-cli --no-default-features --features rustls,postgres`

### Backend

```bash
# Поднять PostgreSQL, MinIO и Mailpit
docker compose up -d

# Создать .env на основе примера и настроить DATABASE_URL
cp .env.example Backend/.env

# Запустить сервер (миграции применяются автоматически при старте)
cd Backend && cargo run
```

Сервер слушает на `0.0.0.0:8080`.

Mailpit (SMTP-перехватчик для разработки) доступен на `http://localhost:8025`.

### Frontend

```bash
cd Frontend
npm install
npm run dev
```

---

## Деплой (Production)

Образы собираются и публикуются в GitHub Container Registry через CI.

```bash
# На сервере: создать .env из .env.example с prod-значениями
docker compose -f docker-compose.prod.yml up -d
```

Caddy автоматически получает TLS-сертификат для `DOMAIN` через Let's Encrypt и проксирует весь трафик на frontend-контейнер.