create table if not exists recipes (
	id integer primary key autoincrement not null,
	title varchar(255) not null,
	summary varchar(255),
	created numeric not null
);

create table if not exists ingredients (
	id integer primary key autoincrement not null,
	position integer not null,
	title varchar(255) not null,
	quantity varchar(255) not null
);

create table if not exists recipe_ingredients_map (
	id integer primary key autoincrement not null,
	recipe_id integer not null,
	ingredient_id integer not null,
	foreign key (recipe_id) references recipes (id),
	foreign key (ingredient_id) references ingredients (id)
);

create table if not exists steps (
	id integer primary key autoincrement not null,
	position integer not null,
	text varchar(255) not null
);

create table if not exists recipe_steps_map (
	id integer primary key autoincrement not null,
	recipe_id integer not null,
	step_id integer not null,
	foreign key (recipe_id) references recipes (id),
	foreign key (step_id) references steps (id)
);