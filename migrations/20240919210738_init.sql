CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table blog_posts(
	id uuid not null primary key default uuid_generate_v4(),
	title text not null,
	lede text not null,
	published timestamp not null default now(),
	content text not null
)
