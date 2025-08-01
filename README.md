# 🔔 JNotify

A desktop application written in Rust that monitors Jenkins build statuses and notifies the user of failed builds via system notifications. It runs in the background as a system tray icon and includes a simple GUI built with Slint for visualizing build information.

## 🎓 Academic Project

This project was developed as part of the **Rust Programming** course in the **Computer Science** program at the **AGH University of Science and Technology**.

## ✨ Features

- 🔄 Checks Jenkins build statuses every 5 seconds using names from a `builds.txt` file
- 📢 Sends a system notification when a build fails
- 🖥️ Runs in the system tray with a small icon
- 🧩 GUI (built with Slint):
  - Displays build name, current status, last build time, and last trigger time
  - Uses a tile-based layout
  - Refreshes automatically every 5 seconds

## 🛠️ Technologies & Dependencies

The project is written entirely in Rust, using the following libraries:

- `notify-rust` – for system notifications  
- `reqwest` + `serde` – for HTTP requests and JSON parsing  
- `tokio` – for asynchronous runtime  
- `slint` – for the graphical user interface  
- `tray-item` – for the system tray functionality  
- `dotenv` – for environment configuration  
- `chrono` – for date/time handling

## 🔧 `.env` File

The app requires a `.env` file in the project root with the following content:

```
JENKINS_URL=your_url
JENKINS_USER=your_user
JENKINS_TOKEN=your_token
```

⚠️ **Note:** This file is not included in the repository.

## 📋 `builds.txt` File

The `builds.txt` file should contain the names of Jenkins jobs you want to monitor — one per line.

Example:

```
build-project-frontend  
build-api-backend  
```

## 🚀 Running the App

1. Set up the `.env` file and `builds.txt`
2. Build the project:

   ```cmd
   cargo build --release
   ```

3. Run the application:

   ```
   cargo run
   ```

## 🖼️ Screenshots

### Tray Icon

<div align="center">
  
  ![Zrzut ekranu 2025-06-29 113933](https://github.com/user-attachments/assets/90b7631f-805e-4adb-81ac-1fc9bf5d7a67)

</div>

### Notification

<div align="center">
  
  ![Zrzut ekranu 2025-06-29 120703](https://github.com/user-attachments/assets/07a831fa-6879-47b8-aaf7-b4e5be0bad84)

</div>

### GUI

<div align="center">
  
  ![Zrzut ekranu 2025-06-29 114032](https://github.com/user-attachments/assets/788aacaf-e1e7-4700-9be5-e29cc7beef9c)

</div>
