CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE SCHEDULE_STATUS AS ENUM(
	'ACTIVE',
	'PAUSED',
	'INACTIVE',
	'CLOSED',
	'FAILED'
);

CREATE TYPE SCHEDULE_ACTION AS ENUM ('PAUSE', 'SKIP');
CREATE TYPE SCHEDULE_ACTION_STATUS AS ENUM ('PENDING', 'APPLIED', 'IGNORED');

CREATE TABLE SCHEDULES (
	ID uuid DEFAULT uuid_generate_v4 (),
	BUCKET text NOT NULL,
	REFERENCE text NOT NULL,
	NAME text NOT NULL,
	ONE_OFF boolean NOT NULL,
	RUN_SPEC jsonb NOT NULL,
	JOB_CODES text [],
	DEPENDENCIES text [],
	STATUS SCHEDULE_STATUS NOT NULL,
	CREATED timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	LAST_UPDATED timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	UNIQUE (BUCKET, REFERENCE, NAME),
	PRIMARY KEY (ID)
);

CREATE FUNCTION update_last_updated_schedules() RETURNS TRIGGER AS $$ BEGIN NEW.LAST_UPDATED = now();
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_schedules_last_updated BEFORE
UPDATE
	ON SCHEDULES FOR EACH ROW EXECUTE PROCEDURE update_last_updated_schedules();

CREATE TABLE SCHEDULE_ACTIONS (
	ID uuid DEFAULT uuid_generate_v4 (),
	SCHEDULE_ID uuid NOT NULL,
	ACTION SCHEDULE_ACTION NOT NULL,
	START_AT timestamp,
	END_AT timestamp,
	STATUS SCHEDULE_ACTION_STATUS NOT NULL,
	CREATED timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	LAST_UPDATED timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
	FOREIGN KEY (SCHEDULE_ID) REFERENCES SCHEDULES (ID),
	PRIMARY KEY (ID)
);

CREATE FUNCTION update_last_updated_schedule_actions() RETURNS TRIGGER AS $$ BEGIN NEW.LAST_UPDATED = now();
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_schedule_actions_last_updated BEFORE
UPDATE
	ON SCHEDULE_ACTIONS FOR EACH ROW EXECUTE PROCEDURE update_last_updated_schedule_actions();
