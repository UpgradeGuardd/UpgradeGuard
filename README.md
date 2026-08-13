# Mingle Backend

**The backend infrastructure powering Mingle's social, payment, identity, and cross-platform services.**

Mingle Backend handles authentication, user accounts, username resolution, real-time communication, payment services, transaction indexing, notifications, media, and the cross-platform AI agent that connects Mingle identities with X, Telegram, and Discord.

The backend does not own core on-chain rules — those live in `mingle-contracts`.

---

# Repositories

| Repository | Responsibility |
|---|---|
| `mingle-app` | Web and mobile frontend |
| `mingle-backend` | Backend services and AI agent |
| `mingle-contracts` | Soroban smart contracts |

---

# Technology Stack

| Layer | Technology |
|---|---|
| Framework | NestJS |
| Language | TypeScript |
| Database | PostgreSQL + Prisma |
| Real-time | Socket.io |
| Blockchain | Stellar / Soroban |
| Authentication | JWT / session-based |
| AI Agent | LLM-based orchestration layer |
| Social integrations | X / Telegram / Discord APIs |
| Testing | Jest |
| Observability | Structured logging + error tracking (e.g. Pino/Winston, Sentry) |

---

# Project Structure

```text
mingle-backend/
├── src/
│   ├── auth/
│   ├── users/
│   ├── usernames/
│   ├── profiles/
│   ├── payments/
│   ├── transactions/
│   ├── chat/
│   ├── feed/
│   ├── media/
│   ├── notifications/
│   ├── identity/
│   ├── agent/
│   │   ├── x/
│   │   ├── telegram/
│   │   ├── discord/
│   │   ├── verification/
│   │   └── orchestration/
│   ├── stellar/
│   ├── common/
│   └── config/
├── prisma/
├── test/
├── .env.example
└── package.json
```

---

# Core Modules

* **Auth** — registration, login, sessions, tokens; kept separate from wallet ownership
* **Users & Profiles** — account records, settings, discovery
* **Usernames** — availability, resolution, registration coordination, username → Stellar account mapping. The Soroban username registry remains authoritative for on-chain ownership; the backend is a lookup/coordination layer, not the source of truth.
* **Payments** — payment preparation, validation, recipient/asset resolution, status tracking, requests. Every payment-creating endpoint requires an **idempotency key** so a retried request can't create a duplicate transfer.
* **Transaction Indexing** — streams ledger/transaction events from Horizon or Soroban RPC (polling fallback) to build readable payment history. Blockchain state remains authoritative for settlement.
* **Chat** — conversations, messages, reactions, payment messages, delivered in real time via WebSockets (`message.created`, `payment.confirmed`, etc.)
* **Feed** — posts, comments, reactions, stories, Money Moments. Media is stored in object storage; the database holds metadata only.
* **Notifications** — messages, payments, comments, mentions, identity-linking events, delivered via FCM/APNs.

---

# Cross-Platform AI Agent

Connects Mingle identities to X, Telegram, and Discord: identity linking, account verification, payment interaction coordination, cross-platform discovery. **The agent never directly controls user private keys or has unrestricted payment authority.**

```text
X / Telegram / Discord → Platform Adapter → Mingle AI Agent
                                                   │
                                     ┌─────────────┴─────────────┐
                                     ▼                           ▼
                              Identity Service            Payment Service
                                     │                           │
                                     └─────────────┬─────────────┘
                                                    ▼
                                            Mingle Backend → Stellar
```

Each platform has its own adapter (`agent/x`, `agent/telegram`, `agent/discord`) so platform-specific code stays out of core business logic. Agent-initiated payment requests carry stricter rate limits than standard endpoints, since this is the highest-risk surface in the system.

**Identity linking flow:** Mingle User → Select Platform → Platform Authentication → Ownership Verification → Identity Association → Mingle Username. A social handle is never treated as proof of Mingle account ownership on its own. Linked platform tokens (`SocialIdentity`) are encrypted at rest.

**Payment authorization:** every agent- or user-initiated payment passes through Identity → Intent → Validation → Authorization → Payment Service → Wallet/Stellar. The agent coordinates; it never authorizes.

---

# Database

PostgreSQL. Core entities: `User`, `Profile`, `Username`, `Wallet`, `Conversation`, `Message`, `Post`, `Comment`, `Reaction`, `Payment`, `PaymentRequest`, `Transaction`, `Notification`, `SocialIdentity`, `SavingsCircle`. Blockchain state is not duplicated unnecessarily.

---

# API

All routes are versioned (`/api/v1/...`) so breaking changes don't disrupt existing clients:

```text
/api/v1/auth  /api/v1/users  /api/v1/usernames  /api/v1/payments
/api/v1/transactions  /api/v1/chat  /api/v1/feed  /api/v1/identity
```

**Principles:** validate all input, authenticate/authorize every protected route, paginate large collections, rate-limit sensitive endpoints (stricter for agent and payment routes), require idempotency keys on payment-creating endpoints, avoid leaking sensitive data, log security-relevant events.

---

# Security

* Hash passwords securely; protect and validate sessions/tokens
* Rate-limit authentication endpoints
* Validate payment parameters and verify recipient information
* Require idempotency keys for payment-creating requests
* Never store private keys unnecessarily
* Encrypt linked social platform credentials (`SocialIdentity` tokens) at rest, and other sensitive data where appropriate
* Maintain audit logs for important actions
* Use structured, correlated logging and an error-tracking service; monitor payment success/failure rate, indexing lag, and auth anomalies

---

# Environment Variables

```env
NODE_ENV=development
PORT=3000
DATABASE_URL=

JWT_SECRET=
JWT_REFRESH_SECRET=

STELLAR_NETWORK=testnet
STELLAR_RPC_URL=
STELLAR_HORIZON_URL=
MINGLE_USERNAME_CONTRACT_ID=
MINGLE_PAYMENT_CONTRACT_ID=

MEDIA_STORAGE_BUCKET=
MEDIA_STORAGE_ENDPOINT=
MEDIA_STORAGE_ACCESS_KEY=
MEDIA_STORAGE_SECRET_KEY=

X_CLIENT_ID=
X_CLIENT_SECRET=
TELEGRAM_BOT_TOKEN=
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
AI_PROVIDER_API_KEY=
SOCIAL_IDENTITY_ENCRYPTION_KEY=
```

Never commit `.env` files or secrets to Git.

---

# Local Development

```bash
git clone <repository-url>
cd mingle-backend
npm install
cp .env.example .env        # configure required variables
npx prisma migrate dev
npx prisma generate
npm run start:dev
```

**Testing:** `npm test` · `npm run test:watch` · `npm run test:cov`

---

# Development Principles

* Keep modules independent — payment logic doesn't belong in the chat module
* Keep Stellar logic behind services/adapters, not scattered through business logic
* Keep X/Telegram/Discord integrations isolated in `agent/`
* Never trust payment information supplied directly by the client
* Treat confirmed Stellar state as authoritative; the database tracks and indexes it, not replaces it

---

# Roadmap

1. **Payment Identity** — auth, accounts, username service, Stellar integration, payment service, transaction indexing
2. **Social Payments** — 1:1/group chat, payment requests, notifications, feed backend
3. **Social Layer** — posts, stories, comments, reactions, Money Moments, community services
4. **Community Finance** — savings circles, escrow services, QR payment services
5. **Cross-Platform Identity & Payments** — X/Telegram/Discord integration, AI agent, identity verification
6. **Ecosystem Infrastructure** — public username resolution API, developer API keys, SDKs, rate-limited public endpoints for external Stellar apps

---

# Project Status

Under active development. Initial focus: authentication, user management, username resolution, Stellar payment services, transaction indexing, real-time communication. Cross-platform AI functionality follows once core identity and payment infrastructure is stable.

---

# License

License information will be added as the project structure is finalized.
