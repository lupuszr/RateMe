Install diesel CLI for postgreSql:

- cargo install diesel_cli --no-default-features --features postgres
  https://diesel.rs/guides/getting-started

Init/create DB:

1. you should create new user in your DB driver:

- username: hackaton_rateMe
- password: hackaton_rateMe

2. run the command for actually creating DB:

- diesel setup,
  or if that is not working, then:
  diesel setup --database-url postgres://hackaton_rateMe:hackaton_rateMe@localhost/hackaton_rateMe

3. create migration:

- diesel migration generate my_new_table

4. run/undo migration:

- diesel migration run
- diesel migration revert
