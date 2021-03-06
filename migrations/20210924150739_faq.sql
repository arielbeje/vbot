CREATE TABLE IF NOT EXISTS "faq_titles" (
	"id"	INTEGER NOT NULL,
	"guild"	BIGINT NOT NULL,
	"title"	TEXT NOT NULL COLLATE NOCASE,
	"content_id"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("content_id") REFERENCES "faq_content"("id")
);
CREATE TABLE IF NOT EXISTS "faq_content" (
	"id"	INTEGER NOT NULL,
	"guild"	BIGINT NOT NULL,
	"content"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
