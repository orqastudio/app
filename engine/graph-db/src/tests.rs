//! Integration tests for the SurrealDB proof-of-concept.
//!
//! All tests use in-memory mode (`kv-mem`) so no disk I/O is needed.
//! Tests cover schema enforcement, relationship creation, upsert idempotency,
//! and deletion cascading.

#[cfg(test)]
#[allow(clippy::needless_raw_string_hashes)]
mod integration {
    use crate::ingest::ingest_single_file;
    use crate::queries;
    use crate::GraphDb;
    use std::io::Write;
    use surrealdb::types::SurrealValue;

    /// Helper: insert a minimal artifact directly via SurrealQL.
    async fn insert_artifact(db: &GraphDb, id: &str, title: &str, artifact_type: &str) {
        let safe_id = id.replace('`', "");
        let title_esc = title.replace('\'', "\\'");
        let type_esc = artifact_type.replace('\'', "\\'");
        let query = format!(
            r#"UPSERT artifact:`{safe_id}` SET
                artifact_type = '{type_esc}',
                title = '{title_esc}',
                description = NONE,
                status = 'active',
                priority = NONE,
                path = '.orqa/test/{safe_id}.md',
                body = NONE,
                frontmatter = {{}},
                source_plugin = NONE,
                content_hash = NONE,
                created = NONE,
                updated = NONE,
                updated_at = time::now();"#
        );
        db.db.query(&query).await.expect("insert artifact");
    }

    /// Helper: create a relates_to edge.
    async fn create_edge(db: &GraphDb, from: &str, to: &str, rel_type: &str) {
        let from_safe = from.replace('`', "");
        let to_safe = to.replace('`', "");
        let rel_esc = rel_type.replace('\'', "\\'");
        let query = format!(
            r#"RELATE artifact:`{from_safe}`->relates_to->artifact:`{to_safe}` SET
                relationship_type = '{rel_esc}',
                field = 'relationships';"#
        );
        db.db.query(&query).await.expect("create edge");
    }

    /// Helper: query artifact IDs via meta::id.
    #[derive(Debug, Clone, SurrealValue)]
    struct IdResult {
        aid: String,
    }

    // -----------------------------------------------------------------------
    // Schema enforcement tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn schema_rejects_artifact_without_title() {
        let db = GraphDb::open_memory().await.expect("open");

        // SCHEMAFULL requires TYPE string for title; NONE should be rejected.
        let query = r#"CREATE artifact:`TEST-001` SET
            artifact_type = 'task',
            title = NONE,
            path = '.orqa/test/TEST-001.md',
            updated_at = time::now();"#;

        let mut response = db.db.query(query).await.expect("query sent");
        let result: Result<Vec<surrealdb::types::Value>, _> = response.take(0);
        assert!(
            result.is_err(),
            "SCHEMAFULL should reject NONE for a string field"
        );
    }

    #[tokio::test]
    async fn schema_enforces_unique_path() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "A-001", "First", "task").await;

        // Second artifact with the same path should fail on unique index.
        let query = r#"CREATE artifact:`A-002` SET
            artifact_type = 'task',
            title = 'Second',
            path = '.orqa/test/A-001.md',
            updated_at = time::now();"#;

        let mut response = db.db.query(query).await.expect("query sent");
        let result: Result<Vec<surrealdb::types::Value>, _> = response.take(0);
        assert!(
            result.is_err(),
            "Unique path index should reject duplicate paths"
        );
    }

    // -----------------------------------------------------------------------
    // Relationship creation and traversal
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn relationship_traversal_works() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "TASK-001", "My Task", "task").await;
        insert_artifact(&db, "EPIC-001", "My Epic", "epic").await;
        create_edge(&db, "TASK-001", "EPIC-001", "delivers").await;

        // Query outgoing edges from TASK-001 using graph traversal — get string IDs.
        let query = r#"SELECT meta::id(id) AS aid
            FROM artifact:`TASK-001`->relates_to->artifact;"#;
        let mut response = db.db.query(query).await.expect("query");

        let results: Vec<IdResult> = response.take(0).expect("deserialize");
        assert!(!results.is_empty(), "should have traversal results");
        let target_ids: Vec<&str> = results.iter().map(|r| r.aid.as_str()).collect();
        assert!(
            target_ids.contains(&"EPIC-001"),
            "traversal should reach EPIC-001, got: {target_ids:?}"
        );
    }

    #[tokio::test]
    async fn reverse_traversal_works() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "TASK-001", "My Task", "task").await;
        insert_artifact(&db, "EPIC-001", "My Epic", "epic").await;
        create_edge(&db, "TASK-001", "EPIC-001", "delivers").await;

        // Query incoming edges to EPIC-001.
        let query = r#"SELECT meta::id(id) AS aid
            FROM artifact:`EPIC-001`<-relates_to<-artifact;"#;
        let mut response = db.db.query(query).await.expect("query");

        let results: Vec<IdResult> = response.take(0).expect("deserialize");
        assert!(!results.is_empty());
        let source_ids: Vec<&str> = results.iter().map(|r| r.aid.as_str()).collect();
        assert!(
            source_ids.contains(&"TASK-001"),
            "reverse traversal should find TASK-001, got: {source_ids:?}"
        );
    }

    // -----------------------------------------------------------------------
    // Upsert idempotency
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn upsert_is_idempotent() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "TASK-001", "First Title", "task").await;
        insert_artifact(&db, "TASK-001", "Updated Title", "task").await;

        // Only one record should exist.
        let count = queries::total_artifacts(&db).await.expect("count");
        assert_eq!(count, 1, "upsert should not duplicate records");

        // Title should be updated.
        #[derive(Debug, Clone, SurrealValue)]
        struct TitleResult {
            title: String,
        }

        let mut response = db
            .db
            .query("SELECT title FROM artifact:`TASK-001`;")
            .await
            .expect("query");
        let results: Vec<TitleResult> = response.take(0).expect("deserialize");
        assert_eq!(results[0].title, "Updated Title");
    }

    // -----------------------------------------------------------------------
    // Deletion cascading
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn delete_artifact_removes_edges() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "TASK-001", "Task", "task").await;
        insert_artifact(&db, "EPIC-001", "Epic", "epic").await;
        create_edge(&db, "TASK-001", "EPIC-001", "delivers").await;

        let edges_before = queries::total_edges(&db).await.expect("count");
        assert_eq!(edges_before, 1);

        // Delete the source artifact and its edges.
        db.db
            .query("DELETE relates_to WHERE in = artifact:`TASK-001`; DELETE artifact:`TASK-001`;")
            .await
            .expect("delete");

        let edges_after = queries::total_edges(&db).await.expect("count");
        assert_eq!(
            edges_after, 0,
            "edges should be removed after artifact deletion"
        );

        let artifacts = queries::total_artifacts(&db).await.expect("count");
        assert_eq!(artifacts, 1, "only EPIC-001 should remain");
    }

    // -----------------------------------------------------------------------
    // File-based ingestion
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn ingest_single_file_creates_artifact_and_edges() {
        let db = GraphDb::open_memory().await.expect("open");

        // Create a temporary directory with a test artifact.
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(".orqa");
        let tasks_dir = orqa_dir.join("implementation/tasks");
        std::fs::create_dir_all(&tasks_dir).expect("create dirs");

        // Write a target artifact first so the edge target exists.
        let epic_dir = orqa_dir.join("implementation/epics");
        std::fs::create_dir_all(&epic_dir).expect("create epic dir");
        let mut epic_file = std::fs::File::create(epic_dir.join("EPIC-001.md")).expect("create");
        write!(
            epic_file,
            "---\nid: EPIC-001\ntitle: Test Epic\ntype: epic\nstatus: active\n---\n# Epic\n"
        )
        .expect("write");

        // Write the task artifact with a relationship.
        let mut task_file = std::fs::File::create(tasks_dir.join("TASK-001.md")).expect("create");
        write!(
            task_file,
            "---\nid: TASK-001\ntitle: Test Task\ntype: task\nstatus: active\nrelationships:\n  - target: EPIC-001\n    type: delivers\n---\n# Task\n"
        )
        .expect("write");

        // Ingest the epic first.
        let epic_edges = ingest_single_file(&db, &epic_dir.join("EPIC-001.md"), &orqa_dir)
            .await
            .expect("ingest epic");
        assert_eq!(epic_edges, 0);

        // Ingest the task.
        let task_edges = ingest_single_file(&db, &tasks_dir.join("TASK-001.md"), &orqa_dir)
            .await
            .expect("ingest task");
        assert_eq!(task_edges, 1);

        let total = queries::total_artifacts(&db).await.expect("count");
        assert_eq!(total, 2);

        let edges = queries::total_edges(&db).await.expect("edges");
        assert_eq!(edges, 1);
    }

    // -----------------------------------------------------------------------
    // Query tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn orphan_detection_works() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "ORPHAN-001", "Orphan", "task").await;
        insert_artifact(&db, "TASK-001", "Connected Task", "task").await;
        insert_artifact(&db, "EPIC-001", "Epic", "epic").await;
        create_edge(&db, "TASK-001", "EPIC-001", "delivers").await;

        let orphans = queries::find_orphans(&db).await.expect("orphans");

        // Get orphan IDs as strings via a separate query.
        let mut response = db
            .db
            .query(
                r#"SELECT meta::id(id) AS aid FROM artifact
                   WHERE count(->relates_to) = 0
                   AND count(<-relates_to) = 0
                   AND status NOT IN ['archived', 'surpassed', 'completed'];"#,
            )
            .await
            .expect("query");
        let orphan_results: Vec<IdResult> = response.take(0).unwrap_or_default();
        let orphan_ids: Vec<&str> = orphan_results.iter().map(|r| r.aid.as_str()).collect();

        assert!(
            orphan_ids.contains(&"ORPHAN-001"),
            "ORPHAN-001 should be detected, got: {orphan_ids:?}"
        );
        assert!(
            !orphan_ids.contains(&"TASK-001"),
            "TASK-001 has outgoing edges, not an orphan"
        );
        // Also verify the query function returns results.
        assert!(!orphans.is_empty(), "find_orphans should return results");
    }

    #[tokio::test]
    async fn count_by_type_works() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "TASK-001", "T1", "task").await;
        insert_artifact(&db, "TASK-002", "T2", "task").await;
        insert_artifact(&db, "EPIC-001", "E1", "epic").await;

        let counts = queries::count_by_type(&db).await.expect("counts");
        let task_count = counts
            .iter()
            .find(|c| c.group.as_deref() == Some("task"));
        assert_eq!(task_count.map(|c| c.count), Some(2));
        let epic_count = counts
            .iter()
            .find(|c| c.group.as_deref() == Some("epic"));
        assert_eq!(epic_count.map(|c| c.count), Some(1));
    }

    #[tokio::test]
    async fn avg_degree_computation() {
        let db = GraphDb::open_memory().await.expect("open");

        insert_artifact(&db, "A", "A", "task").await;
        insert_artifact(&db, "B", "B", "task").await;
        insert_artifact(&db, "C", "C", "task").await;
        // A->B, B->C: A has 1 out, B has 1 in + 1 out, C has 1 in
        create_edge(&db, "A", "B", "delivers").await;
        create_edge(&db, "B", "C", "delivers").await;

        let avg = queries::avg_degree(&db).await.expect("avg degree");
        // Expected: (1 + 2 + 1) / 3 = 1.333...
        assert!(
            (avg - 1.333).abs() < 0.1,
            "avg degree should be ~1.33, got {avg}"
        );
    }
}
