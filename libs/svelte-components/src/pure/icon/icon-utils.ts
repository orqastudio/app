/**
 * Icon utilities — used internally by the Icon component
 * and exported for consumers needing consistent icon resolution.
 *
 * ONLY import icons that are actually used across the codebase.
 * Run `grep -roh 'name="[a-z][-a-z0-9]*"' app/src devtools/src libs/svelte-components/src --include="*.svelte"`
 * to audit usage before adding or removing entries.
 */

import type { Component } from "svelte";

// Status icons
import ActivityIcon from "@lucide/svelte/icons/activity";
import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
import CircleDashedIcon from "@lucide/svelte/icons/circle-dashed";
import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
import CircleStopIcon from "@lucide/svelte/icons/circle-stop";
import CircleXIcon from "@lucide/svelte/icons/circle-x";
import CompassIcon from "@lucide/svelte/icons/compass";

// Navigation & UI
import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
import ArrowUpCircleIcon from "@lucide/svelte/icons/arrow-up-circle";
import ArrowUpDownIcon from "@lucide/svelte/icons/arrow-up-down";
import CheckIcon from "@lucide/svelte/icons/check";
import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
import CopyIcon from "@lucide/svelte/icons/copy";
import CornerDownRightIcon from "@lucide/svelte/icons/corner-down-right";
import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
import EyeIcon from "@lucide/svelte/icons/eye";
import FilterIcon from "@lucide/svelte/icons/filter";
import GripVerticalIcon from "@lucide/svelte/icons/grip-vertical";
import HistoryIcon from "@lucide/svelte/icons/history";
import HomeIcon from "@lucide/svelte/icons/home";
import InfoIcon from "@lucide/svelte/icons/info";
import KanbanIcon from "@lucide/svelte/icons/kanban";
import Link2OffIcon from "@lucide/svelte/icons/link-2-off";
import LinkIcon from "@lucide/svelte/icons/link";
import ListIcon from "@lucide/svelte/icons/list";
import MinusIcon from "@lucide/svelte/icons/minus";
import PencilIcon from "@lucide/svelte/icons/pencil";
import PlusIcon from "@lucide/svelte/icons/plus";
import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
import SaveIcon from "@lucide/svelte/icons/save";
import SearchIcon from "@lucide/svelte/icons/search";
import SettingsIcon from "@lucide/svelte/icons/settings";
import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
import TrashIcon from "@lucide/svelte/icons/trash-2";
import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
import XIcon from "@lucide/svelte/icons/x";
import XCircleIcon from "@lucide/svelte/icons/x-circle";

// Artifact & domain icons
import BookOpenIcon from "@lucide/svelte/icons/book-open";
import BrainIcon from "@lucide/svelte/icons/brain";
import CpuIcon from "@lucide/svelte/icons/cpu";
import DatabaseIcon from "@lucide/svelte/icons/database";
import FileTextIcon from "@lucide/svelte/icons/file-text";
import FolderIcon from "@lucide/svelte/icons/folder";
import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
import FolderXIcon from "@lucide/svelte/icons/folder-x";
import GitBranchIcon from "@lucide/svelte/icons/git-branch";
import GlobeIcon from "@lucide/svelte/icons/globe";
import ImageIcon from "@lucide/svelte/icons/image";
import LayersIcon from "@lucide/svelte/icons/layers";
import MapIcon from "@lucide/svelte/icons/map";
import NetworkIcon from "@lucide/svelte/icons/network";
import PackageIcon from "@lucide/svelte/icons/package";
import PuzzleIcon from "@lucide/svelte/icons/puzzle";
import RocketIcon from "@lucide/svelte/icons/rocket";
import ScaleIcon from "@lucide/svelte/icons/scale";
import ScanIcon from "@lucide/svelte/icons/scan";
import ScanSearchIcon from "@lucide/svelte/icons/scan-search";
import ShieldIcon from "@lucide/svelte/icons/shield";
import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
import TargetIcon from "@lucide/svelte/icons/target";
import TerminalIcon from "@lucide/svelte/icons/terminal";
import UnlinkIcon from "@lucide/svelte/icons/unlink";
import UploadIcon from "@lucide/svelte/icons/upload";
import WebhookIcon from "@lucide/svelte/icons/webhook";
import WorkflowIcon from "@lucide/svelte/icons/workflow";
import WrenchIcon from "@lucide/svelte/icons/wrench";

// Data & system icons
import CalendarIcon from "@lucide/svelte/icons/calendar";
import CalendarCheckIcon from "@lucide/svelte/icons/calendar-check";
import CalendarPlusIcon from "@lucide/svelte/icons/calendar-plus";
import ClockIcon from "@lucide/svelte/icons/clock";
import LoaderIcon from "@lucide/svelte/icons/loader";
import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";

// Communication
import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
import LogInIcon from "@lucide/svelte/icons/log-in";
import LogOutIcon from "@lucide/svelte/icons/log-out";
import MessageSquareIcon from "@lucide/svelte/icons/message-square";
import SendIcon from "@lucide/svelte/icons/send";
import SquareIcon from "@lucide/svelte/icons/square";
import SquareCheckIcon from "@lucide/svelte/icons/square-check";
import SquarePlusIcon from "@lucide/svelte/icons/square-plus";
import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";

/** Default icon registry — maps string keys to Lucide icon components. */
export const DEFAULT_ICON_MAP: Record<string, Component> = {
	// Status
	activity: ActivityIcon,
	"circle-alert": CircleAlertIcon,
	"circle-check": CircleCheckIcon,
	"circle-dashed": CircleDashedIcon,
	"circle-dot": CircleDotIcon,
	"circle-stop": CircleStopIcon,
	"circle-x": CircleXIcon,
	compass: CompassIcon,

	// Navigation & UI
	"arrow-left": ArrowLeftIcon,
	"arrow-up-circle": ArrowUpCircleIcon,
	"arrow-up-down": ArrowUpDownIcon,
	check: CheckIcon,
	"check-circle": CheckCircleIcon,
	"check-circle-2": CheckCircle2Icon,
	"chevron-down": ChevronDownIcon,
	"chevron-left": ChevronLeftIcon,
	"chevron-right": ChevronRightIcon,
	copy: CopyIcon,
	"corner-down-right": CornerDownRightIcon,
	"external-link": ExternalLinkIcon,
	eye: EyeIcon,
	filter: FilterIcon,
	"grip-vertical": GripVerticalIcon,
	history: HistoryIcon,
	home: HomeIcon,
	info: InfoIcon,
	kanban: KanbanIcon,
	link: LinkIcon,
	"link-2-off": Link2OffIcon,
	list: ListIcon,
	minus: MinusIcon,
	pencil: PencilIcon,
	plus: PlusIcon,
	"refresh-cw": RefreshCwIcon,
	save: SaveIcon,
	search: SearchIcon,
	settings: SettingsIcon,
	"sliders-horizontal": SlidersHorizontalIcon,
	"trash-2": TrashIcon,
	"trending-up": TrendingUpIcon,
	x: XIcon,
	"x-circle": XCircleIcon,

	// Artifact & domain
	"book-open": BookOpenIcon,
	brain: BrainIcon,
	cpu: CpuIcon,
	database: DatabaseIcon,
	"file-text": FileTextIcon,
	folder: FolderIcon,
	"folder-code": FolderCodeIcon,
	"folder-open": FolderOpenIcon,
	"folder-plus": FolderPlusIcon,
	"folder-x": FolderXIcon,
	"git-branch": GitBranchIcon,
	globe: GlobeIcon,
	image: ImageIcon,
	layers: LayersIcon,
	map: MapIcon,
	network: NetworkIcon,
	package: PackageIcon,
	puzzle: PuzzleIcon,
	rocket: RocketIcon,
	scale: ScaleIcon,
	scan: ScanIcon,
	"scan-search": ScanSearchIcon,
	shield: ShieldIcon,
	"shield-alert": ShieldAlertIcon,
	"shield-check": ShieldCheckIcon,
	target: TargetIcon,
	terminal: TerminalIcon,
	unlink: UnlinkIcon,
	upload: UploadIcon,
	webhook: WebhookIcon,
	workflow: WorkflowIcon,
	wrench: WrenchIcon,

	// Data & system
	calendar: CalendarIcon,
	"calendar-check": CalendarCheckIcon,
	"calendar-plus": CalendarPlusIcon,
	clock: ClockIcon,
	loader: LoaderIcon,
	"loader-circle": LoaderCircleIcon,

	// Communication
	"alert-triangle": AlertTriangleIcon,
	"log-in": LogInIcon,
	"log-out": LogOutIcon,
	"message-square": MessageSquareIcon,
	send: SendIcon,
	square: SquareIcon,
	"square-check": SquareCheckIcon,
	"square-plus": SquarePlusIcon,
	"triangle-alert": TriangleAlertIcon,
};

/**
 * Resolve an icon name to a Lucide component.
 * Checks the custom registry first (if provided), then the default map.
 * Returns a fallback circle icon if the name is not found.
 * @param name - Icon name to resolve, or undefined to get the fallback.
 * @param customRegistry - Optional plugin-provided icon registry to search first.
 * @returns A Lucide Svelte component for the named icon, or CircleDotIcon as fallback.
 */
export function resolveIcon(
	name: string | undefined,
	customRegistry?: Readonly<Record<string, Component>>,
): Component {
	if (!name) return CircleDotIcon;
	if (customRegistry && name in customRegistry) return customRegistry[name];
	if (name in DEFAULT_ICON_MAP) return DEFAULT_ICON_MAP[name];
	return CircleDotIcon;
}
