container_id=$(docker run --name some-postgres -e POSTGRES_PASSWORD=lol -p 1000:5432 -d postgres)
echo 'Starting postgres database in container with id $container_id '
echo 'Sleeping 5 seconds for database to start'
sleep 5
diesel setup
diesel migration run