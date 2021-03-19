# run
start postgres
migrate dev db
migrate pro db

doctl apps create --spec spec.yaml

SKIP_DOCKER=true ./scripts/init_db.sh
cargo test
vi scripts/init_db.sh
doctl auth init
doctl apps list
history | grep sqlx
DATABASE_URL=postgresql://{USERNAME}:{PASSWORD}@{URL}:{PORT}/newsletter?sslmode=require sqlx migrate run\n
