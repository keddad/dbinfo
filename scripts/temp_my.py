import subprocess
import time
import sys
import pymysql

CONTAINER_NAME = "temp_mysql_ephemeral"
DB_NAME = "testdb"
DB_USER = "root"
DB_PASS = "supersecretpassword"
HOST_PORT = "33060"
IMAGE = "mysql:8.0"

def run_docker():
    """Starts the MySQL container."""
    subprocess.call(["docker", "rm", "-f", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    print(f"🐳 Starting MySQL on port {HOST_PORT}...")
    try:
        subprocess.check_call([
            "docker", "run", "-d",
            "--name", CONTAINER_NAME,
            "-e", f"MYSQL_ROOT_PASSWORD={DB_PASS}",
            "-e", f"MYSQL_DATABASE={DB_NAME}",
            "-p", f"{HOST_PORT}:33060",
            IMAGE
        ], stdout=subprocess.DEVNULL)
    except subprocess.CalledProcessError:
        print("❌ Failed to start Docker container. Is Docker running?")
        sys.exit(1)

def wait_for_mysql():
    """Polls the DB until it accepts connections."""
    print("⏳ Waiting for database to initialize...")
    retries = 60
    while retries > 0:
        try:
            conn = pymysql.connect(
                host="127.0.0.1", port=int(HOST_PORT),
                user=DB_USER, password=DB_PASS, database=DB_NAME,
                connect_timeout=2
            )
            conn.close()
            return True
        except Exception as e:
            time.sleep(1)
            retries -= 1
            print(".", end="", flush=True)
    return False

def setup_data():
    """Creates tables and inserts data."""
    print(f"🛠  Populating '{DB_NAME}' with sample data...")
    
    conn = pymysql.connect(
        host="localhost", port=int(HOST_PORT),
        user=DB_USER, password=DB_PASS, database=DB_NAME
    )
    cur = conn.cursor()

    cur.execute("""
        CREATE TABLE users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(50),
            role VARCHAR(20),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    """)

    cur.execute("""
        CREATE VIEW user_roles AS 
        SELECT role, COUNT(*) as count FROM users GROUP BY role;
    """)

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
    print(f"🧹 Shutting down and removing '{CONTAINER_NAME}'...")
    subprocess.call(["docker", "stop", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    subprocess.call(["docker", "rm", CONTAINER_NAME], 
                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print("✅ Done.")

def main():
    try:
        run_docker()
        
        if not wait_for_mysql():
            print("❌ Timeout waiting for MySQL.")
            cleanup()
            sys.exit(1)

        setup_data()

        conn_str = f"mysql://{DB_USER}:{DB_PASS}@localhost:{HOST_PORT}/{DB_NAME}"
        
        print("" + "="*60)
        print("🚀 DATABASE READY")
        print("="*60)
        print(f"Connection String: {conn_str}")
        print(f"Data: Table 'users' and View 'user_roles' created.")
        print("="*60)

        input("Press [ENTER] to tear down the database...")

    except KeyboardInterrupt:
        print("Interrupted.")
    finally:
        cleanup()

if __name__ == "__main__":
    main()
