/**
 * Icon utilities — used internally by the Icon component
 * and exported for consumers needing consistent icon resolution.
 */

import type { Component } from "svelte";

// Status icons
import CircleIcon from "@lucide/svelte/icons/circle";
import CompassIcon from "@lucide/svelte/icons/compass";
import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
import CircleDotDashedIcon from "@lucide/svelte/icons/circle-dot-dashed";
import CircleStarIcon from "@lucide/svelte/icons/circle-star";
import CircleUserRoundIcon from "@lucide/svelte/icons/circle-user-round";
import CircleCheckBigIcon from "@lucide/svelte/icons/circle-check-big";
import CirclePauseIcon from "@lucide/svelte/icons/circle-pause";
import CircleStopIcon from "@lucide/svelte/icons/circle-stop";
import CircleMinusIcon from "@lucide/svelte/icons/circle-minus";
import CircleFadingArrowUpIcon from "@lucide/svelte/icons/circle-fading-arrow-up";
import ArchiveIcon from "@lucide/svelte/icons/archive";
import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
import CircleDashedIcon from "@lucide/svelte/icons/circle-dashed";
import CircleXIcon from "@lucide/svelte/icons/circle-x";

// Navigation & UI
import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
import HomeIcon from "@lucide/svelte/icons/home";
import SearchIcon from "@lucide/svelte/icons/search";
import SettingsIcon from "@lucide/svelte/icons/settings";
import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
import FilterIcon from "@lucide/svelte/icons/filter";
import ArrowUpDownIcon from "@lucide/svelte/icons/arrow-up-down";
import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
import LinkIcon from "@lucide/svelte/icons/link";
import GripVerticalIcon from "@lucide/svelte/icons/grip-vertical";
import XIcon from "@lucide/svelte/icons/x";
import PlusIcon from "@lucide/svelte/icons/plus";
import MinusIcon from "@lucide/svelte/icons/minus";
import CheckIcon from "@lucide/svelte/icons/check";
import CopyIcon from "@lucide/svelte/icons/copy";
import PencilIcon from "@lucide/svelte/icons/pencil";
import SaveIcon from "@lucide/svelte/icons/save";
import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
import ListIcon from "@lucide/svelte/icons/list";
import KanbanIcon from "@lucide/svelte/icons/kanban";
import HistoryIcon from "@lucide/svelte/icons/history";

// Artifact & domain icons
import FileTextIcon from "@lucide/svelte/icons/file-text";
import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
import UsersIcon from "@lucide/svelte/icons/users";
import ShieldIcon from "@lucide/svelte/icons/shield";
import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
import FolderIcon from "@lucide/svelte/icons/folder";
import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
import FolderXIcon from "@lucide/svelte/icons/folder-x";
import BookOpenIcon from "@lucide/svelte/icons/book-open";
import BookOpenCheckIcon from "@lucide/svelte/icons/book-open-check";
import BookMarkedIcon from "@lucide/svelte/icons/book-marked";
import ZapIcon from "@lucide/svelte/icons/zap";
import TargetIcon from "@lucide/svelte/icons/target";
import LayersIcon from "@lucide/svelte/icons/layers";
import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
import FlaskConicalIcon from "@lucide/svelte/icons/flask-conical";
import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
import GitBranchIcon from "@lucide/svelte/icons/git-branch";
import BotIcon from "@lucide/svelte/icons/bot";
import CheckSquareIcon from "@lucide/svelte/icons/check-square";
import CodeIcon from "@lucide/svelte/icons/code";
import LayoutIcon from "@lucide/svelte/icons/layout";
import PaletteIcon from "@lucide/svelte/icons/palette";
import BrainIcon from "@lucide/svelte/icons/brain";
import PackageIcon from "@lucide/svelte/icons/package";
import FlagIcon from "@lucide/svelte/icons/flag";
import RocketIcon from "@lucide/svelte/icons/rocket";
import WorkflowIcon from "@lucide/svelte/icons/workflow";
import NetworkIcon from "@lucide/svelte/icons/network";
import GraduationCapIcon from "@lucide/svelte/icons/graduation-cap";
import ScaleIcon from "@lucide/svelte/icons/scale";
import SparklesIcon from "@lucide/svelte/icons/sparkles";
import WrenchIcon from "@lucide/svelte/icons/wrench";
import CpuIcon from "@lucide/svelte/icons/cpu";
import TagIcon from "@lucide/svelte/icons/tag";
import GlobeIcon from "@lucide/svelte/icons/globe";
import AnchorIcon from "@lucide/svelte/icons/anchor";
import MapIcon from "@lucide/svelte/icons/map";
import MegaphoneIcon from "@lucide/svelte/icons/megaphone";

// Data & system icons
import DatabaseIcon from "@lucide/svelte/icons/database";
import ScanIcon from "@lucide/svelte/icons/scan";
import ScanSearchIcon from "@lucide/svelte/icons/scan-search";
import UnlinkIcon from "@lucide/svelte/icons/unlink";
import TerminalIcon from "@lucide/svelte/icons/terminal";
import MonitorIcon from "@lucide/svelte/icons/monitor";
import KeyboardIcon from "@lucide/svelte/icons/keyboard";
import ImageIcon from "@lucide/svelte/icons/image";
import UploadIcon from "@lucide/svelte/icons/upload";
import LoaderIcon from "@lucide/svelte/icons/loader";
import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
import ClockIcon from "@lucide/svelte/icons/clock";

// Communication
import SendIcon from "@lucide/svelte/icons/send";
import MessageSquareIcon from "@lucide/svelte/icons/message-square";
import InfoIcon from "@lucide/svelte/icons/info";
import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
import ArrowUpCircleIcon from "@lucide/svelte/icons/arrow-up-circle";
import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
import EyeIcon from "@lucide/svelte/icons/eye";
import LogInIcon from "@lucide/svelte/icons/log-in";
import LogOutIcon from "@lucide/svelte/icons/log-out";
import SquareIcon from "@lucide/svelte/icons/square";
import SquareCheckIcon from "@lucide/svelte/icons/square-check";
import SquarePlusIcon from "@lucide/svelte/icons/square-plus";
import CalendarIcon from "@lucide/svelte/icons/calendar";
import CalendarCheckIcon from "@lucide/svelte/icons/calendar-check";
import CalendarPlusIcon from "@lucide/svelte/icons/calendar-plus";
import Link2OffIcon from "@lucide/svelte/icons/link-2-off";
import ActivityIcon from "@lucide/svelte/icons/activity";
import ArrowLeftRightIcon from "@lucide/svelte/icons/arrow-left-right";
import TrashIcon from "@lucide/svelte/icons/trash-2";
import CheckCircleIcon from "@lucide/svelte/icons/check-circle";

/** Default icon registry — maps string keys to Lucide icon components. */
export const DEFAULT_ICON_MAP: Record<string, Component> = {
	// Status
	circle: CircleIcon,
	compass: CompassIcon,
	"circle-dot": CircleDotIcon,
	"circle-dot-dashed": CircleDotDashedIcon,
	"circle-star": CircleStarIcon,
	"circle-user-round": CircleUserRoundIcon,
	"circle-check-big": CircleCheckBigIcon,
	"circle-pause": CirclePauseIcon,
	"circle-stop": CircleStopIcon,
	"circle-minus": CircleMinusIcon,
	"circle-fading-arrow-up": CircleFadingArrowUpIcon,
	"circle-alert": CircleAlertIcon,
	"circle-check": CircleCheckIcon,
	"circle-dashed": CircleDashedIcon,
	"circle-x": CircleXIcon,
	archive: ArchiveIcon,

	// Navigation & UI
	"chevron-down": ChevronDownIcon,
	"chevron-right": ChevronRightIcon,
	home: HomeIcon,
	search: SearchIcon,
	settings: SettingsIcon,
	"layout-dashboard": LayoutDashboardIcon,
	filter: FilterIcon,
	"arrow-up-down": ArrowUpDownIcon,
	"arrow-right": ArrowRightIcon,
	"external-link": ExternalLinkIcon,
	link: LinkIcon,
	"link-2-off": Link2OffIcon,
	"grip-vertical": GripVerticalIcon,
	x: XIcon,
	plus: PlusIcon,
	minus: MinusIcon,
	check: CheckIcon,
	copy: CopyIcon,
	pencil: PencilIcon,
	save: SaveIcon,
	"refresh-cw": RefreshCwIcon,
	"sliders-horizontal": SlidersHorizontalIcon,
	list: ListIcon,
	kanban: KanbanIcon,
	history: HistoryIcon,
	"trash-2": TrashIcon,

	// Artifact & domain
	"file-text": FileTextIcon,
	"clipboard-list": ClipboardListIcon,
	users: UsersIcon,
	shield: ShieldIcon,
	"shield-check": ShieldCheckIcon,
	"shield-alert": ShieldAlertIcon,
	folder: FolderIcon,
	"folder-open": FolderOpenIcon,
	"folder-code": FolderCodeIcon,
	"folder-plus": FolderPlusIcon,
	"folder-x": FolderXIcon,
	"book-open": BookOpenIcon,
	"book-open-check": BookOpenCheckIcon,
	"book-marked": BookMarkedIcon,
	zap: ZapIcon,
	target: TargetIcon,
	layers: LayersIcon,
	lightbulb: LightbulbIcon,
	"flask-conical": FlaskConicalIcon,
	"scroll-text": ScrollTextIcon,
	"git-branch": GitBranchIcon,
	bot: BotIcon,
	"check-square": CheckSquareIcon,
	code: CodeIcon,
	layout: LayoutIcon,
	palette: PaletteIcon,
	brain: BrainIcon,
	package: PackageIcon,
	flag: FlagIcon,
	rocket: RocketIcon,
	workflow: WorkflowIcon,
	network: NetworkIcon,
	"graduation-cap": GraduationCapIcon,
	scale: ScaleIcon,
	sparkles: SparklesIcon,
	wrench: WrenchIcon,
	cpu: CpuIcon,
	tag: TagIcon,
	globe: GlobeIcon,
	anchor: AnchorIcon,
	map: MapIcon,
	megaphone: MegaphoneIcon,

	// Data & system
	database: DatabaseIcon,
	scan: ScanIcon,
	"scan-search": ScanSearchIcon,
	unlink: UnlinkIcon,
	terminal: TerminalIcon,
	monitor: MonitorIcon,
	keyboard: KeyboardIcon,
	image: ImageIcon,
	upload: UploadIcon,
	loader: LoaderIcon,
	"loader-circle": LoaderCircleIcon,
	clock: ClockIcon,

	// Communication & feedback
	send: SendIcon,
	"message-square": MessageSquareIcon,
	info: InfoIcon,
	"triangle-alert": TriangleAlertIcon,
	"alert-triangle": AlertTriangleIcon,
	"arrow-up-circle": ArrowUpCircleIcon,
	"check-circle-2": CheckCircle2Icon,
	"check-circle": CheckCircleIcon,
	"trending-up": TrendingUpIcon,
	eye: EyeIcon,
	"log-in": LogInIcon,
	"log-out": LogOutIcon,
	square: SquareIcon,
	"square-check": SquareCheckIcon,
	"square-plus": SquarePlusIcon,
	calendar: CalendarIcon,
	"calendar-check": CalendarCheckIcon,
	"calendar-plus": CalendarPlusIcon,
	activity: ActivityIcon,
	"arrow-left-right": ArrowLeftRightIcon,
};

/**
 * Resolve an icon name to a Lucide component.
 * Checks the custom registry first (if provided), then the default map.
 */
export function resolveIcon(
	name: string | undefined,
	customRegistry?: Record<string, Component>,
): Component {
	if (!name) return CircleIcon;
	if (customRegistry && name in customRegistry) return customRegistry[name];
	if (name in DEFAULT_ICON_MAP) return DEFAULT_ICON_MAP[name];
	return CircleIcon;
}
