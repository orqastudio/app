// Component inventory:
// ── Interaction ── Button, Input, Textarea, SearchInput, SelectMenu
// ── Layout ── Stack, HStack, Grid, Center, Spacer, Toolbar, Separator, ScrollArea, Resizable*
// ── Typography ── Heading, Text, Label, Caption, Code, Kbd, Prose
// ── Data Display ── Table*, Badge, SmallBadge, Icon, Status, MetadataRow, MetricCell, Sparkline, ProgressBar, PipelineStages, IssueRow
// ── Feedback ── LoadingSpinner, ErrorDisplay, EmptyState, ConnectionIndicator
// ── Overlay ── Dialog*, AlertDialog*, ConfirmDialog, DropdownMenu*, Tooltip*, Popover*
// ── Navigation ── Tabs*, Breadcrumb, NavItem, Link
// ── Container ── Card*, SimpleCard, FormCard, ListCard, DashboardCard, ViewContainer, Collapsible*
// ── Form ── FormGroup, FormSection, Checkbox, Switch, RadioGroup, IssueFilters
// ── Utility ── VisuallyHidden, ThinkingBlock
// ── DevTools ── StackFrameList, EventDrawer, ContextTable, RawJson, AiExplainButton, TraceTimeline

// Typography primitives
export { Heading, Text, Label, Caption, Code } from "./typography/index.js";

// Single-export primitives
export {
	Button,
	buttonVariants,
	type ButtonProps,
	type ButtonVariant,
	type ButtonSize,
} from "./button/index.js";
export { Badge, badgeVariants, type BadgeVariant } from "./badge/index.js";
export { Input } from "./input/index.js";
export { Textarea } from "./textarea/index.js";
export { Separator } from "./separator/index.js";
export { ScrollArea } from "./scroll-area/index.js";

// Composed single-exports (with parts available for edge cases)
export {
	Tooltip,
	TooltipRoot,
	TooltipTrigger,
	TooltipContent,
	TooltipProvider,
	TooltipPortal,
} from "./tooltip/index.js";
export {
	Popover,
	PopoverRoot,
	PopoverTrigger,
	PopoverContent,
	PopoverClose,
	PopoverPortal,
} from "./popover/index.js";
export {
	Collapsible,
	CollapsibleRoot,
	CollapsibleTrigger,
	CollapsibleContent,
	TreeCollapsibleTrigger,
	CollapsibleGroupHeader,
	CollapsibleSection,
} from "./collapsible/index.js";

// Composed primitives (simple props or custom snippets, parts available)
export {
	Card,
	type CardProps,
	CardRoot,
	CardHeader,
	CardTitle,
	CardDescription,
	CardContent,
	CardFooter,
	CardAction,
} from "./card/index.js";
export {
	Dialog,
	type DialogProps,
	DialogRoot,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogDescription,
	DialogFooter,
	DialogOverlay,
	DialogPortal,
} from "./dialog/index.js";
export {
	AlertDialog,
	type AlertDialogProps,
	AlertDialogRoot,
	AlertDialogContent,
	AlertDialogHeader,
	AlertDialogTitle,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogOverlay,
	AlertDialogPortal,
	AlertDialogTrigger,
} from "./alert-dialog/index.js";
export {
	DropdownMenu,
	type DropdownMenuProps,
	type DropdownMenuItemDef,
	type DropdownMenuEntry,
	DropdownMenuRoot,
	DropdownMenuContent,
	DropdownMenuTrigger,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuGroup,
	DropdownMenuGroupHeading,
	DropdownMenuLabel,
	DropdownMenuShortcut,
	DropdownMenuSub,
	DropdownMenuSubContent,
	DropdownMenuSubTrigger,
	DropdownMenuCheckboxGroup,
	DropdownMenuCheckboxItem,
	DropdownMenuRadioGroup,
	DropdownMenuRadioItem,
	DropdownMenuPortal,
	MenuBarTrigger,
} from "./dropdown-menu/index.js";
export {
	Tabs,
	type TabsProps,
	type TabDef,
	TabsRoot,
	TabsList,
	TabsTrigger,
	TabsContent,
} from "./tabs/index.js";
export {
	Resizable,
	type ResizableProps,
	ResizablePaneGroup,
	ResizableHandle,
	ResizablePane,
} from "./resizable/index.js";

// Status + Icon (components + utilities)
export {
	Status,
	resolveStatus,
	statusLabel,
	statusIconName,
	statusColor,
	statusColorClass,
	statusIsSpinning,
	STATUS_COLOR_CLASSES,
	DEFAULT_STATUSES,
	type StatusConfig,
} from "./status/index.js";
export { Icon, resolveIcon, DEFAULT_ICON_MAP } from "./icon/index.js";

// Data visualisation
export {
	Sparkline,
	TimingChartSvg,
	sparklinePath,
	trendPercent,
	formatTrend,
	trendArrow,
	trendColorClass,
} from "./sparkline/index.js";
export { MetricCell } from "./metric-cell/index.js";

// Shared pure components
export { EmptyState } from "./empty-state/index.js";
export { ErrorDisplay } from "./error-display/index.js";
export { LoadingSpinner } from "./loading-spinner/index.js";
export { SearchInput } from "./search-input/index.js";
export { SelectMenu } from "./select-menu/index.js";
export { MetadataRow } from "./metadata-row/index.js";
export { SmallBadge } from "./small-badge/index.js";
export { PipelineStages } from "./pipeline-stages/index.js";
export type { PipelineStage, PipelineEdge } from "./pipeline-stages/index.js";
export { ThinkingBlock } from "./thinking-block/index.js";
export { ConfirmDialog } from "./confirm-dialog/index.js";
export { Breadcrumb, type BreadcrumbItem } from "./breadcrumb/index.js";

// Form primitives
export { FormGroup, FormSection } from "./form-group/index.js";
export { Link } from "./link/index.js";

// Pattern abstractions
export { FormCard } from "./form-card/index.js";
export { ListCard } from "./list-card/index.js";
export { DashboardCard } from "./dashboard-card/index.js";
export { ProgressBar } from "./progress-bar/index.js";
export { ViewContainer } from "./view-container/index.js";
export { Toolbar, WindowTitleBar, WindowControls } from "./toolbar/index.js";
export { Panel } from "./panel/index.js";
export { SectionHeader, SectionFooter } from "./section-header/index.js";
export { Callout } from "./callout/index.js";
export {
	Stack,
	HStack,
	Grid,
	GridCell,
	Spacer,
	Center,
	Box,
	BackgroundImage,
} from "./layout/index.js";
export { NavItem } from "./nav-item/index.js";
export { ConnectionIndicator, type ConnectionState } from "./connection-indicator/index.js";
export { Kbd } from "./kbd/index.js";
export { Prose } from "./prose/index.js";
export { VisuallyHidden } from "./visually-hidden/index.js";

// Table
export {
	Table,
	TableHeader,
	TableBody,
	TableRow,
	TableHead,
	TableCell,
	TableCaption,
} from "./table/index.js";

// Chat primitives
export {
	ChatBubble,
	ChatInput,
	StreamingDots,
	StreamingText,
	ChatContainer,
} from "./chat/index.js";

// Form controls
export { Checkbox, CheckIndicator } from "./checkbox/index.js";
export { Switch } from "./switch/index.js";
export { RadioGroup, RadioGroupItem } from "./radio-group/index.js";

// Issue management components
export { IssueRow } from "./issue-row/index.js";
export { IssueFilters } from "./issue-filters/index.js";

// Category badge (dynamic color from plugin-declared categories)
export { CategoryBadge } from "./category-badge/index.js";

// Sidebar layout (fixed-width panel with border divider)
export { Sidebar } from "./sidebar/index.js";

// FieldLabel — fixed-width muted label for two-column key-value rows
export { FieldLabel } from "./field-label/index.js";

// PipelineStepper — horizontal progress indicator for artifact lifecycle stages
export { PipelineStepper, type PipelineStepperStage } from "./pipeline-stepper/index.js";

// Small utility primitives
export { Dot } from "./dot/index.js";
export { GlowDot } from "./glow-dot/index.js";
export { ColorDot } from "./color-dot/index.js";
export { CountBadge } from "./count-badge/index.js";
export { VerticalText } from "./vertical-text/index.js";
export { IndentedBlock } from "./indented-block/index.js";
export { TreeGuideLine, TreeIndentIcon, TreeIndent } from "./tree-guide-line/index.js";
export { HighlightWrapper } from "./highlight-wrapper/index.js";
export { PreText } from "./pre-text/index.js";
export { LoadingOverlay } from "./loading-overlay/index.js";

// DevTools Step 4 components
export { StackFrameList } from "./stack-frame-list/index.js";
export { EventDrawer } from "./event-drawer/index.js";
export { ContextTable, type ContextEntry } from "./context-table/index.js";
export { RawJson } from "./raw-json/index.js";

// DevTools Step 5 components
export {
	AiExplainButton,
	buildExplainPrompt,
	type AiExplainButtonProps,
	type ExplainEvent,
} from "./ai-explain-button/index.js";

// DevTools Step 6b components
export { TraceTimeline, type TraceEvent, type TraceTimelineProps } from "./trace-timeline/index.js";

// Side panel shell (fixed-width slide-out drawer with border, bg, shadow)
export { SidePanel } from "./side-panel/index.js";

// Log row primitives for virtualised devtools log tables
export { LogRowShell, LogRowActions, LogRowMetadata } from "./log-row-shell/index.js";
export { LogColumn } from "./log-column/index.js";
export { LogLevelBadge } from "./log-level-badge/index.js";
export { LogViewport, LogSpacer } from "./log-viewport/index.js";
export { LogColLabel } from "./log-col-label/index.js";

// SelectPanel — compact inline picker dropdown (session picker, option lists)
export { SelectPanel, SelectRow, ContextMenu, PickerShell } from "./select-panel/index.js";

// SurfaceBox — rounded raised-surface container for chart wrappers and placeholders
export { SurfaceBox } from "./surface-box/index.js";

// Backdrop — fixed full-viewport overlay for command palettes and custom modals
export { Backdrop } from "./backdrop/index.js";

// SearchOverlay primitives — SearchCard, SearchBarInput, SearchResultItem
export { SearchCard, SearchBarInput, SearchResultItem } from "./search-overlay/index.js";

// AppIcon — application logo image with fixed size and pointer-events disabled
export { AppIcon } from "./app-icon/index.js";

// ActivityBar — icon-only square navigation buttons for the vertical activity bar
export { ActivityBarButton } from "./activity-bar/index.js";

// ColorSwatch — rectangular color preview with a native sr-only color input for settings panels
export { ColorSwatch } from "./color-swatch/index.js";

// StatCard — styled muted stat cell for dashboard metric grids, wraps TooltipTrigger
export { StatCard } from "./stat-card/index.js";

// DynamicGrid — CSS grid with a runtime column count and minWidth (for plugin-driven kanban boards)
export { DynamicGrid } from "./dynamic-grid/index.js";

// SparklineYAxis — fixed-height y-axis scale label column for custom SVG sparklines
export { SparklineYAxis } from "./sparkline-y-axis/index.js";

// MetricGridCell — grid cell with optional per-edge borders for 2×2 metric panels
export { MetricGridCell } from "./metric-grid-cell/index.js";

// TwoByTwoGrid — 2-column 2-row fill-height grid for dashboard improvement trend panels
export { TwoByTwoGrid } from "./two-by-two-grid/index.js";

// Iframe — typed library wrapper around <iframe> so app/devtools code stays free of raw HTML
export { Iframe } from "./iframe/index.js";
