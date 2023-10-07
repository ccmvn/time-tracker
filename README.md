# 📚 Time Tracker School Project 🕒

This is a school project I've been working on for approximately 3 days. The assignment was to create a project, and we
had various topics to choose from. I chose to build a Time Tracker 🕰️, allowing users to manage their work hours and
absences.

## 🔨 Tech Stack

- Written in Rust 🦀
- Uses the Actix-Web framework for the backend 🌐
- SQLite for the database 🗄️
- Tera for templating 📝

## 🚀 Full Features List

### User Features

1. **Login and Logout**: Secure user authentication 🛡️
2. **Home Dashboard**: View a summary of your work hours and absences 🏠
3. **Add Time Entry**: Log your work hours ⏲️
4. **Edit Time Entry**: Modify logged work hours 📝
5. **Delete Time Entry**: Remove incorrect work hour logs ❌
6. **Add Absence Entry**: Log absences 🚫
7. **Edit Absence Entry**: Modify logged absences ✏️
8. **Delete Absence Entry**: Remove incorrect absence logs 🗑️
9. **Settings**: Change password and email settings ⚙️

### Admin Features

1. **Admin Dashboard**: View all users 👨‍💼👩‍💼
2. **User Details**: See specific details about each user 👤
3. **Admin Update Authority**: Change user permissions and roles 🔐
4. **Admin Add/Edit/Delete Time Entry**: Manage work hours for users 🕒
5. **Admin Add/Edit/Delete Absence Entry**: Manage absences for users 📆

⚠️ **Note**: Please keep in mind that this is a school project and is not intended for production use. 📝

## 🛠️ Installation

To install and run the project, follow these steps:

1. Clone the repository: `git clone <repository_url>`
2. Navigate to the project folder: `cd <project_folder>`
3. Install Rust if not installed: [Rust Programming Language](https://www.rust-lang.org/tools/install)
4. Run `cargo build` to compile the code.
5. Run `cargo run` to start the server.

<details>
  <summary>🔍 Database Schema (click to expand)</summary>

  ```sql
  -- Table structure for absence_entries
CREATE TABLE absence_entries
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL,
    absence_date TEXT    NOT NULL,
    reason       TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Table structure for time_entries
CREATE TABLE time_entries
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id    INTEGER,
    task       TEXT,
    spent_time INTEGER,
    date       TEXT
);

-- Table structure for users
CREATE TABLE users
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    username  TEXT NOT NULL UNIQUE,
    password  TEXT NOT NULL,
    email     TEXT NOT NULL UNIQUE,
    authority TEXT DEFAULT 'EMPLOYEE'
);
```

</details>
