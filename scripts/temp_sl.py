import sqlite3
import os
import sys

# --- Configuration ---
DB_FILE = "testdb.sqlite"

def setup_data():
    """Creates tables and inserts data."""
    if os.path.exists(DB_FILE):
        os.remove(DB_FILE)

    print(f"🛠  Creating '{DB_FILE}' with sample data...")
    
    conn = sqlite3.connect(DB_FILE)
    cur = conn.cursor()

    cur.execute("""
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT,
            role TEXT,
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
    cur.executemany("INSERT INTO users (username, role) VALUES (?, ?)", users)
    
    conn.commit()
    conn.close()

def main():
    try:
        setup_data()

        abs_path = os.path.abspath(DB_FILE)
        conn_str = f"{abs_path}"
        
        print("" + "="*60)
        print("🚀 DATABASE READY")
        print("="*60)
        print(f"SQLite File Path: {conn_str}")
        print(f"Data: Table 'users' and View 'user_roles' created.")
        print("="*60)

        input("Press [ENTER] to delete the database file...")

    except KeyboardInterrupt:
        print("Interrupted.")
    finally:
        if os.path.exists(DB_FILE):
            os.remove(DB_FILE)
            print(f"🧹 Removed '{DB_FILE}'")
        print("✅ Done.")

if __name__ == "__main__":
    main()
