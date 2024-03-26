REM Build Docker image
docker build -t my_rust_build:0.0.1 --target development .

docker run -d -t --name my_rust_container my_rust_build:0.0.1

REM Execute a bash shell in the running container
docker exec -it my_rust_container bash