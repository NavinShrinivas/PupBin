sudo pacman -S postgresql
su
su -l postgres 
initdb -D /var/lib/postgres/data
systemctl enable postgres --now
createuser --interactive $USER
createdb PupBin
exit
exit
sudo systemctl status postgresql
psql PupBin 


# Migration for creating entities along with database schemas in SeaORM postgres
# cargo install sea-orm-cli
# sea-orm-cli migrate init
# create a enum in the migration file 
# each new migrastion needs a new file 
# Create the database similar to the one in the up function 
# create a .env file in migrations folder with connection url 
# `cargo run` in the migration folder
# Move to binary crate (main application) and do : 
# `sea-orm-cli generate entity -u DATABASE_CONN_URL -o ./src/entity`, and you now have perfectly working sea orm entities that rep youir databse.
