CREATE TABLE public.project
(
    id serial NOT NULL,
    parent_project_id integer,
    name text NOT NULL,
    due_date timestamp with time zone,
    CONSTRAINT project_pkey PRIMARY KEY (id)
);

CREATE TABLE public.task
(
    id serial NOT NULL,
    project_id integer NOT NULL,
    description text NOT NULL,
    due_date timestamp with time zone,
    completed boolean NOT NULL,
    CONSTRAINT task_pkey PRIMARY KEY (id)
);


CREATE TABLE public.task_sequence
(
    before_task_id integer NOT NULL,
    after_task_id integer NOT NULL,
    CONSTRAINT task_sequence_pkey PRIMARY KEY (before_task_id, after_task_id)
);
