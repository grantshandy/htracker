# htracker

A minimalist todo list, habit tracker, and mood logger. Will be available on web, mobile, and desktop :)

# Repo Structure

 - `/server`: The backend that manages serving the website and api that connects to the backend database. (Rust)
 - `/website`: The frontend website that is served by the webserver. (VueJS & Tailwindcss)
 - `/mobile-app`: The mobile app that communicates to the website api. (React Native)

 # Building/Debugging
 
 There's a lot going on here, this guide should help if you want to build any part of the project yourself.

### Dependencies

  - Rust
  - NodeJS
  - Android Studio/Xcode (optional, for the app)
  - [`cargo-make`](https://github.com/sagiegurari/cargo-make)

### Running For Development
```
cargo make --no-workspace run
```

### Building For Release
```
cargo make --no-workspace release
```