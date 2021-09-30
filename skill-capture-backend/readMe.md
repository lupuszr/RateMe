Init/create DB:

1. you should create new user in your DB driver:

- username: hackaton_rateMe
- password: hackaton_rateMe

2. run the command for actually creating DB:

- diesel setup --database-url postgres://hackaton_rateMe:hackaton_rateMe@localhost/hackaton_rateMe
  (right now .env is not working, shall be checked)
