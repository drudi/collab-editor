/**
 * LoadingSpinner — Centered spinner for loading states.
 *
 * Supports labeled variant: "Loading rooms...", "Connecting...", etc.
 * Uses Tailwind animation utilities.
 * Tailwind-styled with dark mode support.
 */

interface LoadingSpinnerProps {
  /** Optional label text displayed below the spinner */
  label?: string;
  /** Color of the spinner accent (default: indigo-500) */
  color?: 'blue' | 'indigo' | 'green' | 'yellow' | 'red';
  /** Size of the spinner in rem (default: 2) */
  size?: number;
}

const COLOR_MAP: Record<string, string> = {
  blue: 'border-blue-400 border-t-transparent',
  indigo: 'border-indigo-500 border-t-transparent',
  green: 'border-green-500 border-t-transparent',
  yellow: 'border-yellow-500 border-t-transparent',
  red: 'border-red-500 border-t-transparent',
};

const SIZE_MAP: Record<number, string> = {
  1: 'h-4 w-4 border-2',
  2: 'h-8 w-8 border-4',
  3: 'h-12 w-12 border-4',
};

export function LoadingSpinner({
  label,
  color = 'indigo',
  size = 2,
}: LoadingSpinnerProps): JSX.Element {
  const borderClass = COLOR_MAP[color] || COLOR_MAP.indigo;
  const spinnerClass = SIZE_MAP[size] || SIZE_MAP[2];

  return (
    <div
      className="flex flex-col items-center justify-center min-h-screen"
      role="status"
      aria-label={label ? `Loading: ${label}` : undefined}
    >
      <div
        className={`rounded-full animate-spin ${borderClass} ${spinnerClass}`}
      />
      {label && (
        <p className="text-gray-400 mt-4 text-sm">{label}</p>
      )}
    </div>
  );
}
