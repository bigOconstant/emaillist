# Email List

Web app to keep track of who is subcribed or unsubscribed to your email list.

## Technologies

Rust, SQLite, Diesel, Tera

## Description

Users can update their email preferences with a unique UUID.

This can be obtained from examining the crud.db file. Which is the database file. (idea is you will send that id in their email with a unsubscribe link)

After a user signs up or you insert their record in the database they can then update their subscription preferences at `/user/<uuid>`


# Liscence