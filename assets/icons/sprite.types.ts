export type IconName =
  | "nav-home"
  | "nav-focus"
  | "nav-rules"
  | "nav-insights"
  | "nav-connectors"
  | "nav-settings"
  | "focus-strict"
  | "focus-moderate"
  | "focus-light"
  | "focus-break"
  | "focus-sleep"
  | "rule-app"
  | "rule-time"
  | "rule-penalty"
  | "rule-reward"
  | "rule-allowlist"
  | "connector-canvas"
  | "connector-slack"
  | "connector-gmail"
  | "status-active"
  | "status-blocked"
  | "status-warning"
  | "achievement-streak"
  | "achievement-milestone"
  | "action-add"
  | "action-delete"
  | "action-edit"
  | "mascot-happy"
  | "mascot-thinking"
  | "mascot-celebrating"
  | "system-icon-01"
  | "system-icon-02"
  | "system-icon-03"
  | "system-icon-04"
  | "system-icon-05"
  | "system-icon-06"
  | "system-icon-07"
  | "system-icon-08"
  | "system-icon-09"
  | "system-icon-10"
  | "system-icon-11"
  | "system-icon-12"
  | "system-icon-13"
  | "system-icon-14"
  | "system-icon-15"
  | "system-icon-16"
  | "system-icon-17"
  | "system-icon-18"
  | "system-icon-19"
  | "system-icon-20"
  | "system-icon-21"
  | "system-icon-22"
  | "system-icon-23"
  | "system-icon-24"
  | "system-icon-25"
  | "system-icon-26"
  | "system-icon-27"
  | "system-icon-28"
  | "system-icon-29"
  | "system-icon-30"
  | "system-icon-31"
  | "system-icon-32"
  | "system-icon-33";

export interface IconProps {
  name: IconName;
  title?: string;
  className?: string;
}

export const iconNames: IconName[] = [
  "nav-home",
  "nav-focus",
  "nav-rules",
  "nav-insights",
  "nav-connectors",
  "nav-settings",
  "focus-strict",
  "focus-moderate",
  "focus-light",
  "focus-break",
  "focus-sleep",
  "rule-app",
  "rule-time",
  "rule-penalty",
  "rule-reward",
  "rule-allowlist",
  "connector-canvas",
  "connector-slack",
  "connector-gmail",
  "status-active",
  "status-blocked",
  "status-warning",
  "achievement-streak",
  "achievement-milestone",
  "action-add",
  "action-delete",
  "action-edit",
  "mascot-happy",
  "mascot-thinking",
  "mascot-celebrating",
  "system-icon-01",
  "system-icon-02",
  "system-icon-03",
  "system-icon-04",
  "system-icon-05",
  "system-icon-06",
  "system-icon-07",
  "system-icon-08",
  "system-icon-09",
  "system-icon-10",
  "system-icon-11",
  "system-icon-12",
  "system-icon-13",
  "system-icon-14",
  "system-icon-15",
  "system-icon-16",
  "system-icon-17",
  "system-icon-18",
  "system-icon-19",
  "system-icon-20",
  "system-icon-21",
  "system-icon-22",
  "system-icon-23",
  "system-icon-24",
  "system-icon-25",
  "system-icon-26",
  "system-icon-27",
  "system-icon-28",
  "system-icon-29",
  "system-icon-30",
  "system-icon-31",
  "system-icon-32",
  "system-icon-33",
] as IconName[];
