docker build -t minimal-http-server .
docker run --rm --cpus=1 --memory=100m -d -p 8080:8080 --name minimal-http-server minimal-http-server