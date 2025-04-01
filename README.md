# Tesla Authenticator

![Tesla Authenticator Logo](https://frontegg.com/wp-content/uploads/2023/03/Blog_Authenticator-apps_Inner.png)

Tesla Authenticator is a secure, containerized authentication service for Tesla-related applications. Built using Rust, Actix Web, Diesel, and PostgreSQL, it enables secure user registration and login using JWT tokens. It also provides health checks, secure PostgreSQL authentication, and flexible environment configuration.

---

## 🚀 Features
- **User Authentication** with username, email, and password
- **JWT Session Management** for stateless authentication
- **PostgreSQL 17** integration with Diesel ORM
- **Dockerized Deployment** using Docker Compose
- **Secure Configuration** with `scram-sha-256` enforced in PostgreSQL
- **Healthchecks** to ensure readiness and liveness

---

## 🛠 Prerequisites
Make sure the following are installed on your system:
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install) (optional, for local builds)
- [Git](https://git-scm.com/downloads)

---

## 📁 Project Structure
```
Tesla_Authenticator/
├── config/                 # Configuration files
│   └── pg_hba.conf         # Custom PostgreSQL auth config
├── init/                   # Initialization scripts
│   └── init-db.sh          # PostgreSQL init script
├── migrations/             # Diesel DB migrations
├── src/                    # Application source code
│   ├── config.rs           # Env configuration loader
│   ├── db.rs               # Database schema/models
│   ├── errors.rs           # Error definitions
│   ├── handlers.rs         # HTTP endpoints
│   ├── middleware.rs       # JWT validation
│   ├── services/           # Business logic
│   │   └── auth.rs         # Auth service logic
│   └── main.rs             # App entry point
├── .env                    # Environment variables
├── Cargo.toml              # Rust dependencies
├── Dockerfile              # Auth service image
├── docker-compose.yml      # Compose configuration
└── README.md               # This file
```

---

## ⚙️ Setup Instructions

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/Tesla_Authenticator.git
cd Tesla_Authenticator
```

### 2. Configure Environment Variables
Create a `.env` file in the root directory:
```env
JWT_SECRET=REPLACE_ME_BASE64_SECRET
SERVER_PORT=8080
POSTGRES_USER=user
POSTGRES_PASSWORD=securepassword
POSTGRES_DB=db
POSTGRES_HOST=0.0.0.0
POSTGRES_PORT=5432
```

### 3. Configure PostgreSQL Authentication
Create or modify `config/pg_hba.conf`:
```
# TYPE  DATABASE        USER            ADDRESS                 METHOD
local   all             all                                     scram-sha-256
host    all             all             0.0.0.0/0               scram-sha-256
host    all             all             ::/0                    scram-sha-256
```

### 4. Build and Run with Docker Compose
```bash
docker-compose up --build
```
This will:
- Build the Rust app
- Launch PostgreSQL 17
- Initialize the database
- Run migrations

### 5. Verify the Application
- Visit: `http://localhost:8080`
- Health check: `curl http://localhost:8080/health`
- Logs: `docker-compose logs`

---

## 🧪 API Endpoints
| Method | Endpoint           | Description                    |
|--------|--------------------|--------------------------------|
| POST   | `/auth/register`   | Register a new user            |
| POST   | `/auth/login`      | Authenticate and get a JWT     |
| GET    | `/health`          | Healthcheck endpoint           |

---

## 🧰 Configuration Example (Rust)
The app uses environment variables loaded in `src/config.rs`:
```rust
let user = env::var("POSTGRES_USER")?;
let password = env::var("POSTGRES_PASSWORD")?;
let host = env::var("POSTGRES_HOST").unwrap_or("localhost".to_string());
let port = env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
let db = env::var("POSTGRES_DB")?;
```

---

## 🐘 Database Initialization Script
Located in `init/init-db.sh`. It validates required environment variables and creates the PostgreSQL user/database with detailed logs and error handling.

---

## 🧪 Troubleshooting
- **Cannot connect to DB**: Check Docker logs (`docker-compose logs db`)
- **JWT validation fails**: Ensure your `.env` file has a valid `JWT_SECRET`
- **App fails on start**: Check health logs or malformed `.env` values

---

## 🤝 Contributing
1. Fork the repo
2. Create your branch (`git checkout -b feature/xyz`)
3. Commit your changes
4. Push and open a PR

---

## 📄 License
This project is under the MIT License. See `LICENSE` for details.

