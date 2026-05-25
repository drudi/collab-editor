/**
 * SettingsPage — Tabbed settings page for account, editor, and notification preferences.
 *
 * Three tabs:
 * 1. Account: username (read-only), change password, delete account
 * 2. Editor: font size, tab width, theme toggle
 * 3. Notifications: placeholder "Coming soon"
 *
 * All settings persisted to localStorage with key `collab-editor-settings`.
 */

import { useEffect, useState, useCallback } from 'react';

// ─── Types ───────────────────────────────────────────────────────

export type SettingsTab = 'account' | 'editor' | 'notifications';

export interface EditorSettings {
  fontSize: number;   // 12-20px, default 14
  tabWidth: number;   // 2 or 4, default 2
  theme: 'light' | 'dark' | 'system';
}

export interface AccountSettings {
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
}

export interface AppSettings {
  editor: EditorSettings;
  account: AccountSettings;
  notifications: Record<string, boolean>;
}

const DEFAULT_SETTINGS: AppSettings = {
  editor: {
    fontSize: 14,
    tabWidth: 2,
    theme: 'system',
  },
  account: {
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
  },
  notifications: {},
};

const STORAGE_KEY = 'collab-editor-settings';

// ─── Settings Hook ──────────────────────────────────────────────

function useAppSettings(): [AppSettings, (updates: Partial<AppSettings>) => void] {
  const [settings, setSettings] = useState<AppSettings>(() => {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved);
        // Merge with defaults to handle missing keys
        return {
          editor: { ...DEFAULT_SETTINGS.editor, ...parsed.editor },
          account: DEFAULT_SETTINGS.account,
          notifications: parsed.notifications || {},
        };
      }
    } catch {
      // Invalid JSON — use defaults
    }
    return DEFAULT_SETTINGS;
  });

  // Persist to localStorage on change
  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  }, [settings]);

  // Apply editor settings to the DOM immediately
  useEffect(() => {
    // Apply font size to the editor
    document.body.style.fontSize = `${settings.editor.fontSize}px`;
    // Apply theme
    if (settings.editor.theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else if (settings.editor.theme === 'light') {
      document.documentElement.classList.remove('dark');
    } else {
      // System preference — rely on media query
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    }
  }, [settings.editor.fontSize, settings.editor.theme]);

  const updateSettings = useCallback((updates: Partial<AppSettings>) => {
    setSettings((prev) => {
      const prevEditor = prev.editor;
      const editorUpdates = updates.editor;
      let newEditor: EditorSettings;
      if (editorUpdates) {
        newEditor = {
          fontSize: editorUpdates.fontSize ?? prevEditor.fontSize,
          tabWidth: editorUpdates.tabWidth ?? prevEditor.tabWidth,
          theme: editorUpdates.theme ?? prevEditor.theme,
        };
      } else {
        newEditor = prevEditor;
      }
      return {
        ...prev,
        editor: newEditor,
        account: { ...prev.account, ...updates.account },
        notifications: { ...prev.notifications, ...updates.notifications },
      };
    });
  }, []);

  return [settings, updateSettings];
}

// ─── Account Tab ────────────────────────────────────────────────

interface AccountTabProps {
  username: string;
  onChangePassword: (current: string, newPass: string) => Promise<boolean>;
  onDeleteAccount: () => Promise<boolean>;
}

function AccountTab({
  username,
  onChangePassword,
  onDeleteAccount,
}: AccountTabProps): React.JSX.Element {
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [status, setStatus] = useState<'idle' | 'success' | 'error'>('idle');
  const [statusMessage, setStatusMessage] = useState('');

  const handlePasswordChange = async (e: React.FormEvent) => {
    e.preventDefault();

    if (newPassword !== confirmPassword) {
      setStatus('error');
      setStatusMessage('Passwords do not match');
      return;
    }

    if (newPassword.length < 6) {
      setStatus('error');
      setStatusMessage('New password must be at least 6 characters');
      return;
    }

    const success = await onChangePassword(currentPassword, newPassword);
    if (success) {
      setStatus('success');
      setStatusMessage('Password changed successfully');
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
      setTimeout(() => setStatus('idle'), 3000);
    } else {
      setStatus('error');
      setStatusMessage('Failed to change password');
    }
  };

  const handleDelete = async () => {
    if (window.confirm('Are you sure you want to delete your account? This action cannot be undone.')) {
      const success = await onDeleteAccount();
      if (success) {
        setStatus('success');
        setStatusMessage('Account deleted');
      } else {
        setStatus('error');
        setStatusMessage('Failed to delete account');
      }
    }
  };

  return (
    <div className="space-y-6 max-w-lg">
      <h3 className="text-lg font-semibold text-gray-200">Account</h3>

      {/* Current username (read-only) */}
      <div className="space-y-1">
        <label className="text-sm text-gray-400">Username</label>
        <div className="px-3 py-2 bg-gray-800 border border-gray-700 rounded text-gray-300">
          {username}
        </div>
      </div>

      {/* Change password form */}
      <form onSubmit={handlePasswordChange} className="space-y-3">
        <h4 className="text-sm font-medium text-gray-300">Change Password</h4>

        <div className="space-y-1">
          <label htmlFor="current-pass" className="text-xs text-gray-400">Current Password</label>
          <input
            id="current-pass"
            type="password"
            value={currentPassword}
            onChange={(e) => setCurrentPassword(e.target.value)}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-gray-200 text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500"
            placeholder="Enter current password"
          />
        </div>

        <div className="space-y-1">
          <label htmlFor="new-pass" className="text-xs text-gray-400">New Password</label>
          <input
            id="new-pass"
            type="password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-gray-200 text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500"
            placeholder="Enter new password (6+ chars)"
          />
        </div>

        <div className="space-y-1">
          <label htmlFor="confirm-pass" className="text-xs text-gray-400">Confirm New Password</label>
          <input
            id="confirm-pass"
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-gray-200 text-sm focus:outline-none focus:ring-1 focus:ring-indigo-500"
            placeholder="Confirm new password"
          />
        </div>

        <button
          type="submit"
          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm rounded transition-colors"
        >
          Change Password
        </button>

        {status !== 'idle' && (
          <p className={`text-sm ${status === 'success' ? 'text-green-400' : 'text-red-400'}`}>
            {statusMessage}
          </p>
        )}
      </form>

      {/* Delete account */}
      <div className="pt-4 border-t border-gray-700">
        <h4 className="text-sm font-medium text-red-400 mb-2">Danger Zone</h4>
        <button
          onClick={handleDelete}
          className="px-4 py-2 bg-red-600/20 hover:bg-red-600/30 text-red-400 border border-red-800 text-sm rounded transition-colors"
        >
          Delete Account
        </button>
      </div>
    </div>
  );
}

// ─── Editor Tab ─────────────────────────────────────────────────

interface EditorTabProps {
  settings: EditorSettings;
  onUpdate: (updates: Partial<EditorSettings>) => void;
}

function EditorTab({ settings, onUpdate }: EditorTabProps): React.JSX.Element {
  return (
    <div className="space-y-6 max-w-lg">
      <h3 className="text-lg font-semibold text-gray-200">Editor</h3>

      {/* Font size slider (12-20px, default 14) */}
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <label htmlFor="font-size" className="text-sm text-gray-300">Font Size</label>
          <span className="text-sm text-gray-400">{settings.fontSize}px</span>
        </div>
        <input
          id="font-size"
          type="range"
          min={12}
          max={20}
          step={1}
          value={settings.fontSize}
          onChange={(e) => onUpdate({ fontSize: parseInt(e.target.value, 10) })}
          className="w-full accent-indigo-500"
        />
        <div className="flex justify-between text-xs text-gray-500">
          <span>12px</span>
          <span>20px</span>
        </div>
      </div>

      {/* Tab width selector (2 or 4, default 2) */}
      <div className="space-y-2">
        <label className="text-sm text-gray-300">Tab Width</label>
        <div className="flex gap-2">
          {[2, 4].map((width) => (
            <button
              key={width}
              onClick={() => onUpdate({ tabWidth: width })}
              className={`px-4 py-2 text-sm rounded border transition-colors ${
                settings.tabWidth === width
                  ? 'bg-indigo-600 border-indigo-500 text-white'
                  : 'bg-gray-800 border-gray-700 text-gray-300 hover:border-gray-600'
              }`}
            >
              {width} spaces
            </button>
          ))}
        </div>
      </div>

      {/* Theme toggle (light/dark/system) */}
      <div className="space-y-2">
        <label className="text-sm text-gray-300">Theme</label>
        <div className="flex gap-2">
          {([
            { value: 'light', label: 'Light' },
            { value: 'dark', label: 'Dark' },
            { value: 'system', label: 'System' },
          ] as const).map(({ value, label }) => (
            <button
              key={value}
              onClick={() => onUpdate({ theme: value })}
              className={`px-4 py-2 text-sm rounded border transition-colors ${
                settings.theme === value
                  ? 'bg-indigo-600 border-indigo-500 text-white'
                  : 'bg-gray-800 border-gray-700 text-gray-300 hover:border-gray-600'
              }`}
            >
              {label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

// ─── Notifications Tab ──────────────────────────────────────────

function NotificationsTab(): React.JSX.Element {
  return (
    <div className="flex flex-col items-center justify-center max-w-md py-16 text-center">
      <div className="text-4xl mb-4 opacity-50">🔔</div>
      <h3 className="text-lg font-semibold text-gray-300 mb-2">Notifications</h3>
      <p className="text-gray-400 text-sm">
        Coming soon — notification preferences will be available in a future update.
      </p>
    </div>
  );
}

// ─── Settings Page Component ────────────────────────────────────

export interface SettingsPageProps {
  username: string;
  onChangePassword: (current: string, newPass: string) => Promise<boolean>;
  onDeleteAccount: () => Promise<boolean>;
  onNavigateHome: () => void;
}

export function SettingsPage({
  username,
  onChangePassword,
  onDeleteAccount,
  onNavigateHome,
}: SettingsPageProps): React.JSX.Element {
  const [activeTab, setActiveTab] = useState<SettingsTab>('account');
  const [settings, updateSettings] = useAppSettings();

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100">
      {/* Top bar */}
      <div className="flex items-center justify-between px-6 py-3 border-b border-gray-800 bg-gray-900/50">
        <div className="flex items-center gap-4">
          <button
            onClick={onNavigateHome}
            className="text-sm text-gray-400 hover:text-gray-200 transition-colors"
          >
            ← Back
          </button>
          <h1 className="text-lg font-semibold text-gray-200">Settings</h1>
        </div>
      </div>

      {/* Tab navigation */}
      <div className="px-6 border-b border-gray-800">
        <nav className="flex gap-1 -mb-px">
          {(
            [
              { key: 'account', label: 'Account' },
              { key: 'editor', label: 'Editor' },
              { key: 'notifications', label: 'Notifications' },
            ] as const
          ).map(({ key, label }) => (
            <button
              key={key}
              onClick={() => setActiveTab(key)}
              className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === key
                  ? 'border-indigo-500 text-indigo-400'
                  : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-700'
              }`}
            >
              {label}
            </button>
          ))}
        </nav>
      </div>

      {/* Tab content */}
      <div className="px-6 py-8">
        {activeTab === 'account' && (
          <AccountTab
            username={username}
            onChangePassword={onChangePassword}
            onDeleteAccount={onDeleteAccount}
          />
        )}
        {activeTab === 'editor' && (
          <EditorTab
            settings={settings.editor}
            onUpdate={(updates) => {
              updateSettings({ editor: { ...updates } as EditorSettings });
            }}
          />
        )}
        {activeTab === 'notifications' && <NotificationsTab />}
      </div>
    </div>
  );
}
