# run
start postgres
migrate dev db
migrate pro db

doctl apps create --spec spec.yaml
doctl apps list --format ID
doctl apps update APP_ID --spec spec.yaml

SKIP_DOCKER=true ./scripts/init_db.sh
cargo test
vi scripts/init_db.sh
doctl auth init
doctl apps list
history | grep sqlx
DATABASE_URL=postgresql://{USERNAME}:{PASSWORD}@{URL}:{PORT}/newsletter?sslmode=require sqlx migrate run\n

// # do a cargo clean
cargo sqlx prepare --check -- --bin zero2prod
cargo sqlx prepare -- --bin zero2prod

docker run postgres:latest
docker build --tag zero2prod --file Dockerfile .

# LATEST?@?>@K#J!O#J
docker run -e POSTGRES_PASSWORD=password -d postgres
./scripts/init_db.sh


# TODO
- Add way to make admin
    - create 'admin' with random password
    - show password once
    - require password change on first login
- do https://www.lpalmieri.com/posts/session-based-authentication-in-rust/#5-2-4-unhappy-path-the-new-password-is-too-short
