CREATE TYPE algorithm AS ENUM ('rsa-sha256', 'hs2019');

CREATE TABLE instances (
	i_id			uuid NOT NULL PRIMARY KEY UNIQUE,
	domain			TEXT NOT NULL UNIQUE,
	blocked			BOOLEAN NOT NULL DEFAULT false,
	reason			TEXT NULL,
	allowlisted		BOOLEAN NOT NULL DEFAULT false,
);

CREATE TABLE users (
	-- we will generate a uuid for all users
	uid					uuid NOT NULL PRIMARY KEY UNIQUE,
	activitypub_id		TEXT NOT NULL UNIQUE,
	-- used for the actual webpage for the user
	url					TEXT NOT NULL,
	domain				TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	username			TEXT NOT NULL,
	display_name		TEXT NULL,

	public_key_pem		TEXT NOT NULL,
	public_key_id		TEXT NOT NULL,

	banned				BOOLEAN NOT NULL DEFAULT false,
	reason				TEXT NULL,

	inbox				TEXT NOT NULL,
	-- if shared inbox is null in the actor then set the shared inbox to be the inbox
	-- we do this just because then we can have an easy select distinct on it
	shared_inbox		TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	fetched_at			BIGINT NULL,

	-- admins and moderators will be able to ping the special moderation account to perform actions
	-- support for sign on with mastodon for dashboard planned
	is_admin			BOOLEAN NOT NULL DEFAULT false,
	site_moderator		BOOLEAN NOT NULL DEFAULT false,

	UNIQUE (domain, username)
);

-- we're just gonna reuse this for all the accounts
CREATE TABLE ap_instance_actor (
	private_key_pem		TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL,
	algorithm			algorithm NOT NULL
);

CREATE TABLE tags (
	tag_id	BIGSERIAL PRIMARY KEY,
	-- all lowercase, used as uname
	tag 	TEXT NOT NULL UNIQUE,
	-- the capatalization of the tag when initiated
	-- down the road make admin able to configure
	-- important for the visually impared or just ease of reading
	display_name	TEXT NULL,
	-- defaults to 'boosts follower's posts that contain #tag'
	-- plan to allow being set to something else by admins
	bio TEXT NULL,
	-- used to allow admins to ban tags for moderation purposes 
	banned BOOLEAN NOT NOT DEFAULT false
);

CREATE TABLE user_tags (
	-- user follow id (this is the id of this user following this tag)
	-- this will be used for the ending of the id of the follow request coming from us
	ufid				uuid NOT NULL UNIQUE,
	-- used to allow the user to undo following a tag
	user_follow_activitypub_id		TEXT NOT NULL UNIQUE,
	-- the user that is following
	user		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- the user that is being followed
	tag			BIGINT NOT NULL REFERENCES tags(tag_id) ON DELETE CASCADE,
	published		BIGINT NOT NULL,
	PRIMARY KEY(follower, target_user)
);

CREATE TABLE posts (
	-- pid will be used to create the id of the share activity 
	pid 		uuid NOT NULL PRIMARY KEY UNIQUE,
	-- used to deduplicate and allow for users to delete posts
	activitypub_id		TEXT NOT NULL UNIQUE,
	domain		TEXT NOT NULL REFERENCES instances(i_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,

	fetched_at			BIGINT NULL,
	actor	uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE
);

CREATE TABLE post_tags (
	pid 		uuid  NOT NULL REFERENCES posts(pid) ON DELETE CASCADE,
	tag			BIGINT NOT NULL REFERENCES tags(tag_id) ON DELETE CASCADE,
	PRIMARY KEY(pid, tag)
);
