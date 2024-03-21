use sqlx::{Pool, Postgres};

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgress::welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://postgress::welcome@localhost/app";

