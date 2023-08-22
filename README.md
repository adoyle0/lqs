# LQS
Its SQL but backwards. Query supported databases using SQL-FWS (From Where Select) format.

## Format
SQL-FWS format is the standard SQL format except the SELECT clause is the last clause
Ex.
```
FROM table WHERE id = 1 SELECT id
```

## Install

```
git clone
cargo build --release
export PATH=~/Documents/lqs/target/release/:$PATH
```

## Configure

```
lqs init

vim ~/.lqs/config
```

Create new connection in the same format as example.
Example connection connects to postgres as the postgres user

## Run
Enter program to run queries in series
```
lqs --connection=example
```
Exit lqs with the `exit` command

Or to just run 1 query and exit
```
lqs --connection=example --query="from table select *"
```

## Test
```
cargo test
```