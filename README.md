# 环境
Diesel CLI (cargo install diesel_cli --no-default-features --features postgres)
Cargo Watch (cargo install cargo-watch)
Systemfd (cargo install systemfd)



export DATABASE_URL=postgres://postgres:1@localhost/postgres
set DATABASE_URL=postgres://postgres:1@localhost/postgres
diesel setup

# 创建迁移文件
diesel migration generate create_posts
这将在 migrations 目录下生成两个文件：
up.sql：用于创建表的 SQL 语句。
down.sql：用于删除表的 SQL 语句。

# Test
# 打开 up.sql 文件，编写创建表的 SQL 语句。例如，创建一个名为 posts 的表
# -- migrations/xxxxxx_create_posts/up.sql
CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title VARCHAR NOT NULL,
body TEXT NOT NULL,
published BOOLEAN NOT NULL DEFAULT FALSE
);

# 打开 down.sql 文件，编写删除表的 SQL 语句：
DROP TABLE posts;

# 运行迁移
diesel migration run

# pqsl插入数据
psql -h localhost -U postgres -d postgres -f data.sql

