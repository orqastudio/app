-- Remove governance analysis SQLite tables (AD-032: SQLite is for conversation persistence only)
DROP TABLE IF EXISTS governance_recommendations;
DROP TABLE IF EXISTS governance_analyses;
