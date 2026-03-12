use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::governance::{
    GovernanceAnalysis, GovernanceScanResult, Recommendation, RecommendationPriority,
    RecommendationStatus,
};
use crate::error::OrqaError;

/// Persist a governance analysis to the database.
pub fn save_analysis(conn: &Connection, analysis: &GovernanceAnalysis) -> Result<(), OrqaError> {
    let scan_data_json = serde_json::to_string(&analysis.scan_data)?;
    let strengths_json = serde_json::to_string(&analysis.strengths)?;
    let gaps_json = serde_json::to_string(&analysis.gaps)?;

    conn.execute(
        "INSERT INTO governance_analyses \
         (project_id, scan_data, summary, strengths, gaps, session_id, created_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            analysis.project_id,
            scan_data_json,
            analysis.summary,
            strengths_json,
            gaps_json,
            analysis.session_id,
            analysis.created_at,
        ],
    )?;
    Ok(())
}

/// Retrieve the most recent governance analysis for a project.
pub fn get_latest_analysis(
    conn: &Connection,
    project_id: i64,
) -> Result<Option<GovernanceAnalysis>, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, scan_data, summary, strengths, gaps, session_id, created_at \
         FROM governance_analyses \
         WHERE project_id = ?1 \
         ORDER BY created_at DESC, id DESC \
         LIMIT 1",
        params![project_id],
        map_analysis_row,
    )
    .optional()
    .map_err(|e| OrqaError::Database(e.to_string()))
}

/// Persist a batch of recommendations in a single transaction.
pub fn save_recommendations(conn: &Connection, recs: &[Recommendation]) -> Result<(), OrqaError> {
    conn.execute("BEGIN", [])
        .map_err(|e| OrqaError::Database(format!("failed to begin transaction: {e}")))?;

    let result = insert_recommendations(conn, recs);

    match result {
        Ok(()) => {
            conn.execute("COMMIT", [])
                .map_err(|e| OrqaError::Database(format!("failed to commit transaction: {e}")))?;
            Ok(())
        }
        Err(e) => {
            let _ = conn.execute("ROLLBACK", []);
            Err(e)
        }
    }
}

/// Insert all recommendations without transaction management.
fn insert_recommendations(conn: &Connection, recs: &[Recommendation]) -> Result<(), OrqaError> {
    for rec in recs {
        conn.execute(
            "INSERT INTO governance_recommendations \
             (project_id, analysis_id, category, priority, title, description, \
              artifact_type, target_path, content, rationale, status, applied_at, \
              created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                rec.project_id,
                rec.analysis_id,
                rec.category,
                rec.priority.as_str(),
                rec.title,
                rec.description,
                rec.artifact_type,
                rec.target_path,
                rec.content,
                rec.rationale,
                rec.status.as_str(),
                rec.applied_at,
                rec.created_at,
                rec.updated_at,
            ],
        )?;
    }
    Ok(())
}

/// List all recommendations for a project, ordered by creation time.
pub fn list_recommendations(
    conn: &Connection,
    project_id: i64,
) -> Result<Vec<Recommendation>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, analysis_id, category, priority, title, description, \
                artifact_type, target_path, content, rationale, status, applied_at, \
                created_at, updated_at \
         FROM governance_recommendations \
         WHERE project_id = ?1 \
         ORDER BY created_at ASC, id ASC",
    )?;

    let rows = stmt.query_map(params![project_id], map_recommendation_row)?;

    let mut recs = Vec::new();
    for row in rows {
        recs.push(row?);
    }
    Ok(recs)
}

/// Get a single recommendation by its ID.
pub fn get_recommendation(conn: &Connection, id: i64) -> Result<Recommendation, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, analysis_id, category, priority, title, description, \
                artifact_type, target_path, content, rationale, status, applied_at, \
                created_at, updated_at \
         FROM governance_recommendations WHERE id = ?1",
        params![id],
        map_recommendation_row,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("recommendation {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}

/// Update the status of a recommendation and return the updated record.
pub fn update_recommendation_status(
    conn: &Connection,
    id: i64,
    status: &RecommendationStatus,
) -> Result<Recommendation, OrqaError> {
    let rows = conn.execute(
        "UPDATE governance_recommendations \
         SET status = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?2",
        params![status.as_str(), id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("recommendation {id}")));
    }
    get_recommendation(conn, id)
}

/// Mark a recommendation as applied and record the timestamp.
pub fn mark_recommendation_applied(
    conn: &Connection,
    id: i64,
) -> Result<Recommendation, OrqaError> {
    let rows = conn.execute(
        "UPDATE governance_recommendations \
         SET status = 'applied', \
             applied_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?1",
        params![id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("recommendation {id}")));
    }
    get_recommendation(conn, id)
}

/// Map a database row to a `GovernanceAnalysis`.
fn map_analysis_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<GovernanceAnalysis> {
    let scan_data_str: String = row.get(2)?;
    let strengths_str: String = row.get(4)?;
    let gaps_str: String = row.get(5)?;

    let scan_data: GovernanceScanResult = serde_json::from_str(&scan_data_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let strengths: Vec<String> = serde_json::from_str(&strengths_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let gaps: Vec<String> = serde_json::from_str(&gaps_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, Box::new(e))
    })?;

    Ok(GovernanceAnalysis {
        id: row.get(0)?,
        project_id: row.get(1)?,
        scan_data,
        summary: row.get(3)?,
        strengths,
        gaps,
        session_id: row.get(6)?,
        created_at: row.get(7)?,
    })
}

/// Map a database row to a `Recommendation`.
fn map_recommendation_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Recommendation> {
    let priority_str: String = row.get(4)?;
    let status_str: String = row.get(11)?;

    let priority = RecommendationPriority::parse(&priority_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            4,
            rusqlite::types::Type::Text,
            format!("invalid priority: {priority_str}").into(),
        )
    })?;

    let status = RecommendationStatus::parse(&status_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            11,
            rusqlite::types::Type::Text,
            format!("invalid status: {status_str}").into(),
        )
    })?;

    Ok(Recommendation {
        id: row.get(0)?,
        project_id: row.get(1)?,
        analysis_id: row.get(2)?,
        category: row.get(3)?,
        priority,
        title: row.get(5)?,
        description: row.get(6)?,
        artifact_type: row.get(7)?,
        target_path: row.get(8)?,
        content: row.get(9)?,
        rationale: row.get(10)?,
        status,
        applied_at: row.get(12)?,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::domain::governance::{GovernanceArea, RecommendationPriority, RecommendationStatus};

    fn test_scan_result() -> GovernanceScanResult {
        GovernanceScanResult {
            areas: vec![GovernanceArea {
                name: "claude_rules".to_string(),
                source: "claude".to_string(),
                files: vec![],
                covered: false,
            }],
            coverage_ratio: 0.0,
        }
    }

    fn make_project(conn: &Connection) -> i64 {
        conn.execute(
            "INSERT INTO projects (name, path) VALUES ('test', '/test/path')",
            [],
        )
        .expect("insert project");
        conn.last_insert_rowid()
    }

    fn make_analysis(_conn: &Connection, project_id: i64) -> GovernanceAnalysis {
        GovernanceAnalysis {
            id: 0,
            project_id,
            scan_data: test_scan_result(),
            summary: "Test summary".to_string(),
            strengths: vec!["Has rules".to_string()],
            gaps: vec!["No hooks".to_string()],
            session_id: None,
            created_at: "2026-01-01T00:00:00.000Z".to_string(),
        }
    }

    #[test]
    fn save_and_get_latest_analysis() {
        let conn = init_memory_db().expect("db init");
        let project_id = make_project(&conn);
        let analysis = make_analysis(&conn, project_id);

        save_analysis(&conn, &analysis).expect("save");

        let fetched = get_latest_analysis(&conn, project_id)
            .expect("get")
            .expect("should exist");

        assert_eq!(fetched.project_id, project_id);
        assert_eq!(fetched.summary, "Test summary");
        assert_eq!(fetched.strengths, vec!["Has rules"]);
        assert_eq!(fetched.gaps, vec!["No hooks"]);
        assert!(fetched.session_id.is_none());
    }

    #[test]
    fn get_latest_analysis_empty_returns_none() {
        let conn = init_memory_db().expect("db init");
        let project_id = make_project(&conn);
        let result = get_latest_analysis(&conn, project_id).expect("get");
        assert!(result.is_none());
    }

    #[test]
    fn save_and_list_recommendations() {
        let conn = init_memory_db().expect("db init");
        let project_id = make_project(&conn);
        let analysis = make_analysis(&conn, project_id);
        save_analysis(&conn, &analysis).expect("save analysis");
        let analysis_id = conn.last_insert_rowid();

        let rec = Recommendation {
            id: 0,
            project_id,
            analysis_id,
            category: "rule".to_string(),
            priority: RecommendationPriority::Critical,
            title: "Add error handling rule".to_string(),
            description: "Enforce Result types".to_string(),
            artifact_type: "markdown".to_string(),
            target_path: ".orqa/governance/rules/error-handling.md".to_string(),
            content: "# Error Handling\n\nUse Result types.".to_string(),
            rationale: "No error handling rule exists.".to_string(),
            status: RecommendationStatus::Pending,
            applied_at: None,
            created_at: "2026-01-01T00:00:00.000Z".to_string(),
            updated_at: "2026-01-01T00:00:00.000Z".to_string(),
        };

        save_recommendations(&conn, &[rec]).expect("save recs");

        let recs = list_recommendations(&conn, project_id).expect("list");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].title, "Add error handling rule");
        assert_eq!(recs[0].priority, RecommendationPriority::Critical);
        assert_eq!(recs[0].status, RecommendationStatus::Pending);
    }

    #[test]
    fn update_recommendation_status_persists() {
        let conn = init_memory_db().expect("db init");
        let project_id = make_project(&conn);
        let analysis = make_analysis(&conn, project_id);
        save_analysis(&conn, &analysis).expect("save");
        let analysis_id = conn.last_insert_rowid();

        let rec = Recommendation {
            id: 0,
            project_id,
            analysis_id,
            category: "hook".to_string(),
            priority: RecommendationPriority::Recommended,
            title: "Add pre-commit hook".to_string(),
            description: "Run clippy before commit".to_string(),
            artifact_type: "shell".to_string(),
            target_path: ".orqa/governance/hooks/pre-commit.sh".to_string(),
            content: "#!/bin/bash\ncargo clippy".to_string(),
            rationale: "Prevents clippy warnings from landing.".to_string(),
            status: RecommendationStatus::Pending,
            applied_at: None,
            created_at: "2026-01-01T00:00:00.000Z".to_string(),
            updated_at: "2026-01-01T00:00:00.000Z".to_string(),
        };
        save_recommendations(&conn, &[rec]).expect("save");
        let rec_id = conn.last_insert_rowid();

        let updated = update_recommendation_status(&conn, rec_id, &RecommendationStatus::Approved)
            .expect("update");
        assert_eq!(updated.status, RecommendationStatus::Approved);
    }

    #[test]
    fn mark_recommendation_applied_sets_applied_at() {
        let conn = init_memory_db().expect("db init");
        let project_id = make_project(&conn);
        let analysis = make_analysis(&conn, project_id);
        save_analysis(&conn, &analysis).expect("save");
        let analysis_id = conn.last_insert_rowid();

        let rec = Recommendation {
            id: 0,
            project_id,
            analysis_id,
            category: "rule".to_string(),
            priority: RecommendationPriority::Optional,
            title: "Test rec".to_string(),
            description: "desc".to_string(),
            artifact_type: "markdown".to_string(),
            target_path: ".orqa/governance/rules/test.md".to_string(),
            content: "# Test".to_string(),
            rationale: "reason".to_string(),
            status: RecommendationStatus::Approved,
            applied_at: None,
            created_at: "2026-01-01T00:00:00.000Z".to_string(),
            updated_at: "2026-01-01T00:00:00.000Z".to_string(),
        };
        save_recommendations(&conn, &[rec]).expect("save");
        let rec_id = conn.last_insert_rowid();

        let applied = mark_recommendation_applied(&conn, rec_id).expect("apply");
        assert_eq!(applied.status, RecommendationStatus::Applied);
        assert!(applied.applied_at.is_some());
    }

    #[test]
    fn get_recommendation_not_found() {
        let conn = init_memory_db().expect("db init");
        let result = get_recommendation(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn update_nonexistent_recommendation_fails() {
        let conn = init_memory_db().expect("db init");
        let result = update_recommendation_status(&conn, 999, &RecommendationStatus::Approved);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }
}
