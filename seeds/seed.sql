INSERT INTO "public"."canonical_feeds"("name", "url") 
VALUES
  ('CSS Tricks', 'https://css-tricks.com/feed/'),
  ('xkcd.com', 'https://xkcd.com/atom.xml');
INSERT INTO "public"."feeds"("name", "website", "last_updated", "canonical_feed")
VALUES
  ('XKCD', 'https://xkcd.com', '2020-02-29 00:00:00', 2),
  ('xkcd', 'https://xkcd.com', '2020-02-29 00:00:00', 2),
  ('CSSTricks', 'https://css-tricks.com', '2020-02-29 00:00:00', 1);