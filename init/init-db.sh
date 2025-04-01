#!/usr/bin/env bash
set -euo pipefail

# Enhanced logging function with timestamp, level, and optional exit code
log() {
    local level="$1"
    local message="$2"
    local exit_code="${3:-}"  # Optional exit code for ERROR level
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local log_entry="$timestamp [$level] $message"
    
    # Add exit code to log entry if provided (for ERROR cases)
    if [ -n "$exit_code" ] && [ "$level" = "ERROR" ]; then
        log_entry="$log_entry (Exit code: $exit_code)"
    fi
    
    # Output to console with color
    case "$level" in
        "INFO")
            echo -e "\033[32m$log_entry\033[0m"  # Green
            ;;
        "ERROR")
            echo -e "\033[31m$log_entry\033[0m" >&2  # Red to stderr
            ;;
        "WARN")
            echo -e "\033[33m$log_entry\033[0m"  # Yellow
            ;;
        *)
            echo "$log_entry"  # Default, no color
            ;;
    esac
}

# Trap errors and signals for comprehensive logging
trap 'log "ERROR" "Script failed at line $LINENO" "$?"; exit $?' ERR
trap 'log "INFO" "Script interrupted"; exit 1' INT TERM

# Main execution
log "INFO" "🚀 Initializing PostgreSQL setup..."

# Check required environment variables with detailed logging
log "INFO" "Validating environment variables..."
required_vars=("POSTGRES_USER" "POSTGRES_PASSWORD" "POSTGRES_DB")
for var in "${required_vars[@]}"; do
    if [ -z "${!var:-}" ]; then
        log "ERROR" "Required environment variable $var is not set."
        exit 1
    else
        log "INFO" "Found $var"
    fi
done

# Execute SQL statements with logging
log "INFO" "Executing PostgreSQL setup commands as ${POSTGRES_USER} on database ${POSTGRES_DB}..."
# Capture psql output and errors separately
psql_output=$(mktemp)
psql_errors=$(mktemp)

# Connect to the database specified by POSTGRES_DB
if psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" >"$psql_output" 2>"$psql_errors" <<-EOSQL
    -- Create user if it doesn't exist
    DO \$\$
    BEGIN
        IF NOT EXISTS (
            SELECT FROM pg_catalog.pg_roles WHERE rolname = '${POSTGRES_USER}'
        ) THEN
            RAISE NOTICE 'Creating role: ${POSTGRES_USER}';
            CREATE ROLE "${POSTGRES_USER}" LOGIN PASSWORD '${POSTGRES_PASSWORD}';
        ELSE
            RAISE NOTICE 'Role "${POSTGRES_USER}" already exists, skipping.';
        END IF;
    END
    \$\$;

    -- Create database if it doesn't exist
    -- Note: This may be redundant since POSTGRES_DB is already created by Docker
    DO \$\$
    BEGIN
        IF NOT EXISTS (
            SELECT FROM pg_database WHERE datname = '${POSTGRES_DB}'
        ) THEN
            RAISE NOTICE 'Creating database: ${POSTGRES_DB}';
            CREATE DATABASE "${POSTGRES_DB}"
                WITH OWNER = "${POSTGRES_USER}"
                     ENCODING = 'UTF8'
                     LC_COLLATE = 'en_US.UTF-8'
                     LC_CTYPE = 'en_US.UTF-8'
                     TEMPLATE = template0;
        ELSE
            RAISE NOTICE 'Database "${POSTGRES_DB}" already exists, skipping.';
        END IF;
    END
    \$\$;
EOSQL
then
    log "INFO" "PostgreSQL commands executed successfully."
    # Log psql output (NOTICE messages)
    while IFS= read -r line; do
        [ -n "$line" ] && log "INFO" "psql: $line"
    done < "$psql_output"
else
    log "ERROR" "PostgreSQL setup failed with exit code $?" "$?"
    log "ERROR" "psql errors: $(cat "$psql_errors")"
    rm -f "$psql_output" "$psql_errors"
    exit 1
fi

# Clean up temporary files
rm -f "$psql_output" "$psql_errors"

log "INFO" "✅ PostgreSQL initialization complete."