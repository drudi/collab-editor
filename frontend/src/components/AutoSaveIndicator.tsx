/**
 * AutoSaveIndicator — Shows the current save status of the collaborative document.
 *
 * States:
 * - "unsaved" — document has local changes not yet persisted (orange dot)
 * - "saving" — save in progress (spinning icon)
 * - "saved" — last save completed successfully (green checkmark, auto-fades after 2s)
 *
 * Position: bottom-right of the editor area, small and unobtrusive.
 * Tailwind-styled with dark mode support.
 */

import { useEffect, useState } from 'react';

export type SaveStatus = 'unsaved' | 'saving' | 'saved';

interface AutoSaveIndicatorProps {
  status: SaveStatus;
}

export function AutoSaveIndicator({ status }: AutoSaveIndicatorProps): JSX.Element {
  const [autoFaded, setAutoFaded] = useState(false);

  // When status is 'saved', schedule auto-fade back after 2 seconds (P4-T02, AC6)
  // Only set state when status is 'saved' — when it changes away, visualState derives correctly
  useEffect(() => {
    if (status === 'saved') {
      const timer = setTimeout(() => {
        setAutoFaded(true);
      }, 2000);
      return () => clearTimeout(timer);
    }
    // No setState here — when status is not 'saved', visualState will derive as non-faded
  }, [status]);

  // Derived state: only show as 'unsaved' when status is 'saved' AND auto-fade timer fired
  const visualState = (autoFaded && status === 'saved') ? 'unsaved' : status;

  // Determine icon based on visual state
  const getIcon = (): string => {
    switch (visualState) {
      case 'unsaved':
        return '●'; // Orange dot for unsaved
      case 'saving':
        return '⟳'; // Spinning icon for saving
      case 'saved':
        return '✓'; // Green checkmark for saved
    }
  };

  // Determine text color based on visual state
  const getColor = (): string => {
    switch (visualState) {
      case 'unsaved':
        return 'text-amber-400'; // Orange/amber for unsaved
      case 'saving':
        return 'text-blue-400'; // Blue for saving
      case 'saved':
        return 'text-green-400'; // Green for saved
    }
  };

  // Determine label based on visual state
  const getLabel = (): string => {
    switch (visualState) {
      case 'unsaved':
        return 'Unsaved';
      case 'saving':
        return 'Saving...';
      case 'saved':
        return 'Saved';
    }
  };

  // Spinning ring needs animate-spin class only when saving
  const isSpinning = visualState === 'saving';

  return (
    <div
      className="absolute bottom-4 right-4 flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-opacity duration-300 select-none"
      style={{
        background: 'rgba(0, 0, 0, 0.6)',
        backdropFilter: 'blur(8px)',
        opacity: autoFaded ? 0.6 : 0.85,
      }}
    >
      {/* Icon */}
      <div className={`w-3 h-3 ${isSpinning ? 'animate-spin' : ''}`}>
        {isSpinning ? (
          // Spinning ring (Tailwind animate-spin)
          <div
            className="w-3 h-3 rounded-full border-2 border-current border-t-transparent"
            aria-label="Saving"
          />
        ) : (
          // Unicode character for unsaved/saved state
          <span
            className={`${getColor()}`}
            style={{ fontSize: '14px', lineHeight: '1' }}
          >
            {getIcon()}
          </span>
        )}
      </div>

      {/* Label */}
      <span
        className={`${getColor()}`}
        style={{ fontSize: '11px', fontWeight: 500 }}
      >
        {getLabel()}
      </span>
    </div>
  );
}
