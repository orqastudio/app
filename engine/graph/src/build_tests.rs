//! Additional edge-case unit tests for `engine/graph/src/build.rs`.
//!
//! Covers:
//! - Malformed YAML frontmatter: file skipped gracefully, graph continues building
//! - Broken reference: ref recorded but target absent from graph (verified via graph_stats)
//! - `graph_stats` on various graph shapes: orphan detection, edge count, broken-ref count
//! - `infer_artifact_type`: path-based heuristic for well-known directory names
//! - `extract_frontmatter`: edge cases (no markers, empty body, multiple `---` blocks)
//! - `humanize_stem`: additional casing variants

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::build::{
        build_artifact_graph, extract_frontmatter, graph_stats, humanize_stem, infer_artifact_type,
    };

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write_file(dir: &std::path::Path, name: &str, content: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(dir.join(name), content).expect("write file");
    }

    // -------------------------------------------------------------------------
    // build_artifact_graph — malformed YAML
    // -------------------------------------------------------------------------

    #[test]
    fn malformed_yaml_frontmatter_skips_bad_file_but_keeps_good_ones() {
        // A file with invalid YAML must be skipped without aborting the scan.
        // A valid file in the same directory must still appear in the graph.
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/implementation/epics");

        write_file(&dir, "BAD.md", "---\n: this is: not valid yaml: [\n---\n# Body\n");
        write_file(
            &dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Good Epic\nstatus: draft\n---\n# Body\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        // The bad file has no parseable `id`, so only EPIC-001 makes it in.
        assert_eq!(graph.nodes.len(), 1, "bad file must be skipped; good file must survive");
        assert!(graph.nodes.contains_key("EPIC-001"));
    }

    #[test]
    fn file_with_no_frontmatter_markers_is_skipped() {
        // A `.md` file without `---` markers has no frontmatter and should produce no node.
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/implementation/tasks");
        write_file(&dir, "TASK-001.md", "Just plain body text, no YAML.\n");

        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty(), "file without frontmatter must produce no node");
    }

    // -------------------------------------------------------------------------
    // build_artifact_graph — broken references
    // -------------------------------------------------------------------------

    #[test]
    fn artifact_with_ref_to_missing_target_records_ref_but_target_absent() {
        // TASK-001 references EPIC-MISSING which does not exist in the project.
        // The ref must be recorded on TASK-001's references_out, but EPIC-MISSING
        // must not appear as a node in the graph.
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/implementation/tasks");
        write_file(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-MISSING\n    type: delivers\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.contains_key("TASK-001"), "source node must be in graph");
        assert!(!graph.nodes.contains_key("EPIC-MISSING"), "missing target must NOT be in graph");

        let task = graph.nodes.get("TASK-001").expect("node");
        let ref_targets: Vec<&str> = task.references_out.iter().map(|r| r.target_id.as_str()).collect();
        assert!(ref_targets.contains(&"EPIC-MISSING"),
            "the broken ref must still be recorded in references_out");
    }

    // -------------------------------------------------------------------------
    // graph_stats — various graph shapes
    // -------------------------------------------------------------------------

    #[test]
    fn graph_stats_empty_graph_returns_all_zeros() {
        let tmp = make_project();
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
        assert_eq!(stats.orphan_count, 0);
        assert_eq!(stats.broken_ref_count, 0);
    }

    #[test]
    fn graph_stats_single_isolated_node_is_orphan() {
        // A node with no relationships is an orphan (unless its type is "doc").
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/implementation/tasks");
        write_file(
            &dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Lone Task\nstatus: active\n---\n# Body\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.edge_count, 0);
        assert_eq!(stats.orphan_count, 1, "an isolated task must be counted as an orphan");
        assert_eq!(stats.broken_ref_count, 0);
    }

    #[test]
    fn graph_stats_connected_nodes_not_orphans() {
        // TASK-001 → EPIC-001: neither node is an orphan (both have at least one edge).
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/implementation/tasks");
        let epics_dir = tmp.path().join(".orqa/implementation/epics");

        write_file(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Connected Task\nrelationships:\n  - target: EPIC-001\n    type: delivers\n---\n",
        );
        write_file(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Connected Epic\nstatus: active\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1, "one directed edge TASK-001→EPIC-001");
        assert_eq!(stats.orphan_count, 0, "both nodes are connected — no orphans");
    }

    #[test]
    fn graph_stats_broken_ref_counted() {
        // TASK-001 references EPIC-MISSING which does not exist → broken_ref_count=1.
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/implementation/tasks");
        write_file(
            &dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nrelationships:\n  - target: EPIC-MISSING\n    type: delivers\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.broken_ref_count, 1,
            "reference to a non-existent target must increment broken_ref_count");
    }

    #[test]
    fn graph_stats_doc_type_not_counted_as_orphan() {
        // "doc" type nodes are excluded from orphan analysis by design.
        let tmp = make_project();
        let dir = tmp.path().join(".orqa/docs");
        write_file(
            &dir,
            "DOC-001.md",
            "---\nid: DOC-001\ntype: doc\ntitle: A Doc\n---\n# Content\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.orphan_count, 0, "doc-type nodes must not count as orphans");
    }

    #[test]
    fn graph_stats_multiple_broken_refs_counted_individually() {
        // TASK-001 has two broken refs and one valid one.
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/implementation/tasks");
        let epics_dir = tmp.path().join(".orqa/implementation/epics");

        write_file(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nrelationships:\n  - target: EPIC-001\n    type: delivers\n  - target: GONE-1\n    type: delivers\n  - target: GONE-2\n    type: delivers\n---\n",
        );
        write_file(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Epic\nstatus: active\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.broken_ref_count, 2,
            "two of three refs point to missing targets → broken_ref_count=2");
    }

    // -------------------------------------------------------------------------
    // infer_artifact_type — path heuristic
    // -------------------------------------------------------------------------

    #[test]
    fn infer_type_tasks_path_returns_task() {
        let t = infer_artifact_type(".orqa/implementation/tasks/TASK-001.md", &Vec::new(), None, "TASK-001", &[]);
        assert_eq!(t, "task", "path under 'tasks' must infer type 'task'");
    }

    #[test]
    fn infer_type_epics_path_returns_epic() {
        let t = infer_artifact_type(".orqa/implementation/epics/EPIC-001.md", &Vec::new(), None, "EPIC-001", &[]);
        assert_eq!(t, "epic");
    }

    #[test]
    fn infer_type_frontmatter_type_takes_precedence_over_path() {
        // Explicit `type:` in frontmatter beats all other inference.
        let t = infer_artifact_type(".orqa/implementation/tasks/T-1.md", &Vec::new(), Some("pillar"), "T-1", &[]);
        assert_eq!(t, "pillar",
            "frontmatter type must override path-based inference");
    }

    #[test]
    fn infer_type_unknown_path_falls_back_to_doc() {
        let t = infer_artifact_type(".orqa/unknown-subdir/XYZ-001.md", &Vec::new(), None, "XYZ-001", &[]);
        assert_eq!(t, "doc", "path with no known prefix must fall back to 'doc'");
    }

    // -------------------------------------------------------------------------
    // extract_frontmatter — edge cases
    // -------------------------------------------------------------------------

    #[test]
    fn extract_frontmatter_no_markers_returns_none_and_full_content() {
        let content = "# Just a heading\n\nSome body text.";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_none(), "content without --- markers has no frontmatter");
        assert_eq!(body, content, "body must be the full original content");
    }

    #[test]
    fn extract_frontmatter_empty_body_returns_empty_string() {
        let content = "---\nid: EPIC-001\n---\n";
        let (fm, body) = extract_frontmatter(content);
        assert_eq!(fm.as_deref(), Some("id: EPIC-001"));
        assert_eq!(body, "", "body must be empty string when nothing follows the closing ---");
    }

    #[test]
    fn extract_frontmatter_trims_yaml_whitespace() {
        // YAML text between markers with leading/trailing blank lines must be trimmed.
        let content = "---\n\nid: TASK-001\ntitle: My Task\n\n---\n# Body\n";
        let (fm, _body) = extract_frontmatter(content);
        let fm = fm.expect("frontmatter");
        assert!(!fm.starts_with('\n'), "frontmatter must not start with a newline");
        assert!(!fm.ends_with('\n'), "frontmatter must not end with a newline");
    }

    #[test]
    fn extract_frontmatter_body_content_preserved() {
        // Body after the closing --- must be returned verbatim (minus leading newline).
        let content = "---\nid: EPIC-001\n---\n## Description\n\nSome text here.\n";
        let (_fm, body) = extract_frontmatter(content);
        assert!(body.contains("## Description"), "body content must be preserved");
        assert!(body.contains("Some text here."));
    }

    // -------------------------------------------------------------------------
    // humanize_stem — additional cases
    // -------------------------------------------------------------------------

    #[test]
    fn humanize_stem_single_word_title_cased() {
        let path = PathBuf::from("epic.md");
        assert_eq!(humanize_stem(&path), "Epic");
    }

    #[test]
    fn humanize_stem_multiple_words_each_title_cased() {
        let path = PathBuf::from("my-great-epic.md");
        assert_eq!(humanize_stem(&path), "My Great Epic");
    }

    #[test]
    fn humanize_stem_already_titled_unchanged() {
        let path = PathBuf::from("TASK-042.md");
        assert_eq!(humanize_stem(&path), "TASK-042");
    }
}
