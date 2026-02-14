import subprocess
import time
import sys
import psycopg2
from psycopg2 import sql

# --- Configuration ---
CONTAINER_NAME = "temp_postgres_ephemeral"
DB_NAME = "testdb"
DB_USER = "postgres"
DB_PASS = "supersecretpassword"
HOST_PORT = "54321"  # The port on your machine
IMAGE = "postgres:15-alpine"

def run_docker():
    """Starts the Postgres container."""
    # Ensure a previous run didn't leave a zombie container
    subprocess.call(["docker", "rm", "-f", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    print(f"🐳 Starting PostgreSQL on port {HOST_PORT}...")
    try:
        subprocess.check_call([
            "docker", "run", "-d",
            "--name", CONTAINER_NAME,
            "-e", f"POSTGRES_PASSWORD={DB_PASS}",
            "-p", f"{HOST_PORT}:5432",
            IMAGE
        ], stdout=subprocess.DEVNULL)
    except subprocess.CalledProcessError:
        print("❌ Failed to start Docker container. Is Docker running?")
        sys.exit(1)

def wait_for_postgres():
    """Polls the DB until it accepts connections."""
    print("⏳ Waiting for database to initialize...")
    retries = 30
    while retries > 0:
        try:
            # Connect to the default 'postgres' database to check health
            conn = psycopg2.connect(
                dbname="postgres", user=DB_USER, password=DB_PASS, 
                host="localhost", port=HOST_PORT
            )
            conn.close()
            return True
        except psycopg2.OperationalError:
            time.sleep(1)
            retries -= 1
            print(".", end="", flush=True)
    return False

def setup_data():
    """Creates the DB, tables, and inserts data."""
    print(f"\n🛠  Populating '{DB_NAME}' with sample data...")
    
    # 1. Create the Database
    conn = psycopg2.connect(
        dbname="postgres", user=DB_USER, password=DB_PASS, 
        host="localhost", port=HOST_PORT
    )
    conn.autocommit = True
    cur = conn.cursor()
    cur.execute(sql.SQL("CREATE DATABASE {}").format(sql.Identifier(DB_NAME)))
    cur.close()
    conn.close()

    # 2. Connect to New DB and Create Table
    conn = psycopg2.connect(
        dbname=DB_NAME, user=DB_USER, password=DB_PASS, 
        host="localhost", port=HOST_PORT
    )
    cur = conn.cursor()

    cur.execute("""
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50),
            role VARCHAR(20),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    """)

    # 3. Insert Data
    users = [
        ("Alice", "Admin"),
        ("Bob", "User"),
        ("Charlie", "Moderator")
    ]
    cur.executemany("INSERT INTO users (username, role) VALUES (%s, %s)", users)
    conn.commit()
    conn.close()

def cleanup():
    """Stops and removes the container."""
    print(f"\n🧹 Shutting down and removing '{CONTAINER_NAME}'...")
    subprocess.call(["docker", "stop", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    subprocess.call(["docker", "rm", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print("✅ Done.")

def main():
    try:
        run_docker()
        
        if not wait_for_postgres():
            print("\n❌ Timeout waiting for Postgres.")
            cleanup()
            sys.exit(1)

        setup_data()

        conn_str = f"postgres://{DB_USER}:{DB_PASS}@localhost:{HOST_PORT}/{DB_NAME}"
        
        print("\n" + "="*60)
        print("🚀 DATABASE READY")
        print("="*60)
        print(f"Connection String: \n\n    {conn_str}\n")
        print(f"Data: Table 'users' created with 3 rows.")
        print("="*60)

        input("Press [ENTER] to tear down the database...")

    except KeyboardInterrupt:
        print("\nInterrupted.")
    finally:
        cleanup()

if __name__ == "__main__":
    main()