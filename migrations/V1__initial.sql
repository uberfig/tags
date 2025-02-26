CREATE TABLE instances (
	i_id			uuid NOT NULL PRIMARY KEY UNIQUE,
	domain			TEXT NOT NULL UNIQUE,
	blocked			BOOLEAN NOT NULL DEFAULT false,
	reason			TEXT NULL,
	allowlisted		BOOLEAN NOT NULL DEFAULT false
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
	summary				TEXT NULL, -- used as a user's bio
	public_key_pem		TEXT NOT NULL,
	public_key_id		TEXT NOT NULL,
	manual_followers	BOOLEAN NOT NULL DEFAULT false, -- manually approves followers

	banned				BOOLEAN NOT NULL DEFAULT false,
	reason				TEXT NULL,

	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	fetched_at			BIGINT NULL,

	UNIQUE (domain, username)
);

-- we're just gonna reuse this for all the accounts
CREATE TABLE ap_instance_actor (
	private_key_pem		TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL,
	algorithm			TEXT NOT NULL
);

CREATE TABLE user_tags (

	ufid				uuid NOT NULL UNIQUE,
	activitypub_id		TEXT NOT NULL UNIQUE,
	-- the user that is following
	follower		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- the user that is being followed
	target_user		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	published		BIGINT NOT NULL,
	PRIMARY KEY(follower, target_user)
);

CREATE TABLE posts (
	-- pid is generated locally and used by the api to 
	-- fetch user posts
	pid 		uuid NOT NULL PRIMARY KEY UNIQUE,
	activitypub_id		TEXT NOT NULL UNIQUE,
	domain		TEXT NOT NULL REFERENCES instances(i_id) ON DELETE CASCADE,

	surtype		TEXT NOT NULL,
	subtype		TEXT NOT NULL,
	category	TEXT NOT NULL,

	likes		BIGINT NOT NULL DEFAULT 0,
	boosts		BIGINT NOT NULL DEFAULT 0,
	reactions	TEXT NULL,

	federation_level	federation_level NOT NULL DEFAULT 'federated',
	visibility			post_visibility NOT NULL DEFAULT 'public',
	in_group		uuid NULL REFERENCES groups(group_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,

	fetched_at			BIGINT NULL,
	actor	uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE
);

CREATE TABLE tags (
	-- all lowercase, used as uname
	tag 	TEXT NOT NULL PRIMARY KEY UNIQUE,
	-- the capatalization of the tag when initiated
	-- down the road make admin able to configure
	-- important for the visually impared or just ease of reading
	display	TEXT NULL,
	-- defaults to 'boosts follower's posts that contain #tag'
	-- plan to allow being set to something else by admins
	bio TEXT NULL
);
