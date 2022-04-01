# Sim Signs Sim Racing Astrology Bot
A discord bot to enhance your connection to the stars, built for sim racers
(Built as an april fool's joke)

## Features
The bot utilizes a database connection to the database to store a zodiac sign for the user and provide the user with information related to their sign such as the optimal car and track choice a monthly horoscope.

The bot can be easily extended to do more with the information, especially text based


## Configuration
###### Environment Variables
- `DISCORD_TOKEN` - Discord token for the bot
- `DB_CONNECTION` - Connection string to the database used by the bot
  - `host=<> dbname=<> user=<> password=<>`

## Requirements and dependencies
#### Rust toolchain
 - Easiest way to install the rust toolchain is with rustup
   - https://www.rust-lang.org/tools/install

#### serenity
 - Rust library for the Discord API
   - https://github.com/serenity-rs/serenity
 
#### tokio
 - Rust runtime for asynchronous applications
   - https://github.com/tokio-rs/tokio
 
#### tokio-postgres
 - PostgreSQL support for rust
   - https://github.com/sfackler/rust-postgres
   
    
## Acknowledgements
Thanks to https://github.com/owlgee for the art

Thanks to Danny and Lena for help with the monthly horoscope writing
