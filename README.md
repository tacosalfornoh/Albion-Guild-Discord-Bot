# Commit Messages:

1. main.rs
  + Added dotenv initialization and environment variable loading
  + Configured Serenity client with event handler for interactions
  + Implemented main function with async runtime

2. handlers/mod.rs
  + Created module for interaction create handler
  + Implemented event handler for interaction create events

3. handlers/interaction_create_handler.rs
  + Added interaction create handler to process slash commands
  + Implemented command matching and response handling

4. commands/mod.rs
  + Created module for command handlers
  + Added submodules for balance, application, permissions, api, and ping commands

5. commands/ping.rs
  + Implemented ping command with run and register functions

6. utils/config.rs
  + Added utility function to fetch environment variables

7. utils/logging.rs
  + Added logging initialization function with env_logger

8. utils/mod.rs
  + Created module for utility functions
  + Added submodules for config and logging

9. Cargo.toml
  + Added dependencies for tokio, serenity, dotenv, log, and env_logger