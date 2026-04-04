export interface OrqaError {
	readonly code:
		| "not_found"
		| "database"
		| "file_system"
		| "sidecar"
		| "validation"
		| "scan"
		| "serialization"
		| "permission_denied";
	readonly message: string;
}
