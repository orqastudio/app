export type ThemeMode = "light" | "dark" | "system";

class SettingsStore {
	themeMode = $state<ThemeMode>("system");
	defaultModel = $state<string>("auto");
	fontSize = $state<number>(14);
	activeSection = $state<string>("provider");

	setThemeMode(mode: ThemeMode) {
		this.themeMode = mode;
	}

	setDefaultModel(model: string) {
		this.defaultModel = model;
	}

	setFontSize(size: number) {
		this.fontSize = Math.max(12, Math.min(20, size));
	}

	setActiveSection(section: string) {
		this.activeSection = section;
	}
}

export const settingsStore = new SettingsStore();
