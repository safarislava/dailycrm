# CRM

Система управления проектами для строительных/инженерных организаций. Позволяет вести проекты, разбивать их на этапы, отслеживать стоимость, дедлайны, подтверждения ГИП и оплаты, прикреплять файлы и акты, а также вести журнал комментариев по каждому этапу.

## Стек

**Backend** — Rust, Actix-web 4, PostgreSQL (sqlx), MinIO (S3-совместимый объектный storage), JWT (access + refresh токены), bcrypt, lettre (SMTP)

**Frontend** — React 18, TypeScript, Redux Toolkit, Vite, SCSS

**Тестирование** — k6 (нагрузочные тесты)

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
│   ├── projects/      # проекты, этапы, подэтапы, файлы, акты, комментарии
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
├── mail.rs            # Mailer через lettre/SMTP (SSL/TLS или STARTTLS)
├── jwt.rs             # подпись и верификация токенов
├── routes.rs          # регистрация всех маршрутов
├── db.rs              # пул соединений PostgreSQL
├── cors.rs            # конфигурация CORS
└── common.rs          # общие вспомогательные типы

Backend/migrations/    # SQL-миграции (sqlx migrate)

Frontend/src/
├── components/        # UI-компоненты
├── store/             # Redux state-менеджмент
├── styles/            # Стили SCSS/CSS
├── types/             # TS типы данных
├── App.tsx
├── main.tsx
└── App.module.scss

test/
└── load_test.js       # Скрипт нагрузочного тестирования k6
```

**Фоновые задачи** (запускаются при старте сервера):
- **12:00 ежедневно** — рассылка дайджеста дедлайнов пользователям с включёнными уведомлениями
- **каждую минуту** — отправка накопленных уведомлений из очереди

---

## Переменные окружения

В `.env` должны быть заполнены следующие значения.

| Переменная         | Описание                                                                            |
|--------------------|-------------------------------------------------------------------------------------|
| `DATABASE_URL`     | PostgreSQL connection string: `postgres://user:password@host:5432/crm`              |
| `JWT_SECRET`       | Секрет для подписи JWT-токенов                                                      |
| `MINIO_ENDPOINT`   | URL MinIO: `http://localhost:9000` (dev) или `http://minio:9000` (prod)             |
| `MINIO_ACCESS_KEY` | Access key MinIO                                                                    |
| `MINIO_SECRET_KEY` | Secret key MinIO                                                                    |
| `SMTP_HOST`        | SMTP-хост для отправки почты (например, `smtp.resend.com`)                          |
| `SMTP_PORT`        | SMTP-порт (`465` — SSL/TLS, `587` — STARTTLS, любой другой — без шифрования)        |
| `SMTP_USERNAME`    | SMTP-логин                                                                          |
| `SMTP_PASSWORD`    | SMTP-пароль                                                                         |
| `MAIL_FROM`        | Адрес отправителя, например `CRM <noreply@example.com>`                             |

Для prod-деплоя дополнительно нужны (используются в `docker-compose.prod.yml`):

| Переменная                | Описание                                  |
|---------------------------|-------------------------------------------|
| `DOMAIN`                  | Домен сайта, например `dailycrm.mooo.com` |
| `POSTGRES_USER`           | Пользователь PostgreSQL                   |
| `POSTGRES_PASSWORD`       | Пароль PostgreSQL                         |
| `GITHUB_REPOSITORY_OWNER` | GitHub-логин для pull образов из GHCR     |

---

## Локальный запуск

### Зависимости

- Rust (edition 2024)
- Node.js 18+
- Docker + Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx): `cargo install sqlx-cli --no-default-features --features rustls,postgres`

## Нагрузочное тестирование

Для проверки производительности используется [k6](https://k6.io/). Сценарий теста находится в `test/load_test.js`.

Запуск тестов локально (требуется установленный k6):
```bash
k6 run test/load_test.js
```

---

## Деплой (Production)

Образы собираются и публикуются в GitHub Container Registry через CI.

```bash
# На сервере: создать .env с prod-значениями
docker compose -f docker-compose.prod.yml up -d
```

Caddy автоматически получает TLS-сертификат для `DOMAIN` через Let's Encrypt и проксирует весь трафик на frontend-контейнер.