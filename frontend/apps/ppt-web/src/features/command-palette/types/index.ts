/**
 * Command Palette Types
 * Epic 129: Command Palette
 */

export type CommandCategory = 'navigation' | 'action' | 'search' | 'settings' | 'create' | 'recent';

export interface Command {
  id: string;
  label: string;
  description?: string;
  category: CommandCategory;
  keywords: string[];
  icon?: string;
  shortcut?: string;
  action: () => void | Promise<void>;
  disabled?: boolean;
  hidden?: boolean;
}

export interface CommandGroup {
  category: CommandCategory;
  label: string;
  commands: Command[];
}

export interface CommandPaletteState {
  isOpen: boolean;
  query: string;
  selectedIndex: number;
  recentCommandIds: string[];
}

export interface CommandPaletteContextValue {
  isOpen: boolean;
  open: () => void;
  close: () => void;
  toggle: () => void;
  registerCommand: (command: Command) => void;
  unregisterCommand: (commandId: string) => void;
  commands: Command[];
  recentCommands: Command[];
  addToRecent: (commandId: string) => void;
}
