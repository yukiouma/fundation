MYSQL_ID = $(docker ps -qf "name=mysql8.0" --format "{{.ID}}")

build:
	docker build -t fundation/repository:v0.1 .

run:
	docker run -d -p 8080:8080 --name fundation-repository -e DATABASE_URL="mysql://root:000000@172.17.0.3:3306/fundation?parseTime=True" fundation/repository:v0.1

