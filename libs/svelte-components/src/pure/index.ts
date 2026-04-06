// Component inventory:
// ── Interaction ── Button, Input, Textarea, SearchInput, SelectMenu
// ── Layout ── Stack, HStack, Grid, Center, Spacer, Toolbar, Separator, ScrollArea, Resizable*
// ── Typography ── Heading, Text, Label, Caption, Code, Kbd, Prose
// ── Data Display ── Table*, Badge, SmallBadge, Icon, Status, MetadataRow, MetricCell, Sparkline, ProgressBar, PipelineStages
// ── Feedback ── LoadingSpinner, ErrorDisplay, EmptyState, ConnectionIndicator
// ── Overlay ── Dialog*, AlertDialog*, ConfirmDialog, DropdownMenu*, Tooltip*, Popover*
// ── Navigation ── Tabs*, Breadcrumb, NavItem, Link
// ── Container ── Card*, SimpleCard, FormCard, ListCard, DashboardCard, ViewContainer, Collapsible*
// ── Form ── FormGroup, FormSection, Checkbox, Switch, RadioGroup
// ── Utility ── VisuallyHidden, ThinkingBlock

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
export { Toolbar } from "./toolbar/index.js";
export { Panel } from "./panel/index.js";
export { SectionHeader, SectionFooter } from "./section-header/index.js";
export { Callout } from "./callout/index.js";
export { Stack, HStack, Grid, Spacer, Center, Box } from "./layout/index.js";
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
export { ChatBubble, ChatInput, StreamingDots, ChatContainer } from "./chat/index.js";

// Form controls
export { Checkbox } from "./checkbox/index.js";
export { Switch } from "./switch/index.js";
export { RadioGroup, RadioGroupItem } from "./radio-group/index.js";

// Small utility primitives
export { Dot } from "./dot/index.js";
export { CountBadge } from "./count-badge/index.js";
export { VerticalText } from "./vertical-text/index.js";
