# rust-warp-jwt-sql-crud

This is a demo for new Rust developers that intend to cover a variety of topics and used in a CRUD, SQL-backed
microservice with JWT authentication.

The driver behind this simple demo, even though there is a plethora of similar boilerplates available, was the difficulty to
find good examples tying everything together using the latest frameworks available.

It may not have the best practices fully implemented, but it is hopefully a good start for developers from other languages venturing into Rust.

### Dependent Components ###
This application requires only a Postgres database, with parameters configurable in the **.env** file.

To spin up a quick DB instance and Adminer, you can use the following commands, assuming you have Docker installed:
```
docker-compose up -d db adminer
```
You can access the Adminer at http://localhost:8080 and login with the following details:

    System: PostgreSQL  
    Server: db  
    Username: demo_user  
    Password: demo_password  
    Database: demo_db (optional)

Alternatively, you can create a postgres container named **db** without using docker-compose:
```
docker run -d --name postgres -e POSTGRES_USER=demo_user -e POSTGRES_PASSWORD=demo_password -e POSTGRES_DB=demo_db -p 5432:5432 db
```
(Remove the **-d** parameter to run both options in the foreground)

## Compiling & Running ##
To compile the code, the table contained in **db.sql** must be created. The statements in the file will also create the users:

    admin@test.com / abc123
    user@test.com / abc123  
Then you can run the application with the command:

    cargo run

To run with hot deployment support, you need to have **systemfd** and **cargo-watch** installed:

    cargo install systemfd
    cargo install cargo-watch
    systemfd --no-pid -s http::0.0.0.0:8000 -- cargo watch -x run



Assuming you have Curl installed, you can test the API with the commands:

#### Register 
    curl -H 'Content-Type: application/json' -d '{"email": "test@test.com", "password": "abc123"}' http://localhost:8000/api/auth/register

#### Login
    curl -H 'Content-Type: application/json' -d '{"email": "test@test.com", "password": "abc123"}' http://localhost:8000/api/auth/login

Save the token returned and use it for the authenticated calls:

#### Test
    curl -H 'Authorization: Bearer <TOKEN>' http://localhost:8000/api/user
    curl -H 'Authorization: Bearer <TOKEN>' http://localhost:8000/api/admin


#### Docker Deployment
The docker debug/release part is still under work, updates should come soon. 