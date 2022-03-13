# Email List

Web app to keep track of who is subcribed or unsubscribed to your email list.

## Technologies

Rust, SQLite, Diesel, Tera

## Description

Users can update their email preferences with a unique UUID.

This can be obtained from examining the crud.db file. Which is the database file. (idea is you will send that id in their email with a unsubscribe link)

After a user signs up or you insert their record in the database they can then update their subscription preferences at `/user/<uuid>`



# Environment variables

You can currently see the needed env variables in the `.cargo/config.toml` file.

```
DATABASE_URL = "crud.db"
APP_NAME = "APPNAME"
PORT = "8080"
IPBIND = "0.0.0.0"
```

# Usage

cargo up

go to `localhost:8080` to add a new user to your subscription list.


# UI

![](https://files.catbox.moe/qadshn.png)

![](https://files.catbox.moe/el7b6n.png)

![](https://files.catbox.moe/6uknim.png)


