-- Create work_sessions table
CREATE TABLE work_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    duration_seconds INTEGER NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create tags table
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7), -- For hex color codes like #FF0000
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create session_tags junction table
CREATE TABLE session_tags (
    session_id UUID NOT NULL REFERENCES work_sessions(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (session_id, tag_id)
);

-- Create indexes
CREATE INDEX idx_work_sessions_created_at ON work_sessions(created_at);
CREATE INDEX idx_tags_name ON tags(name);
CREATE INDEX idx_session_tags_session_id ON session_tags(session_id);
CREATE INDEX idx_session_tags_tag_id ON session_tags(tag_id);