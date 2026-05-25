/**
 * ErrorBoundary — Catches React render errors and shows a fallback UI.
 *
 * Wraps child components in a try-catch for render errors.
 * Shows a fallback UI with error message and "Try again" button.
 * Resets error state when "Try again" is clicked.
 * Logs error to console for debugging.
 */

import { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    // Log error to console for debugging
    console.error('ErrorBoundary caught an error:', error);
    console.error('Component stack:', errorInfo.componentStack);
  }

  handleReset = (): void => {
    this.setState({ hasError: false, error: null });
  };

  render(): ReactNode {
    if (this.state.hasError) {
      // Use custom fallback or default fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="flex items-center justify-center min-h-screen bg-gray-950">
          <div className="text-center max-w-md mx-auto p-6 rounded-xl bg-gray-900 border border-gray-800 shadow-xl">
            {/* Error icon */}
            <div className="text-5xl mb-4">⚠️</div>

            <h2 className="text-xl font-semibold text-gray-200 mb-2">
              Something went wrong
            </h2>

            <p className="text-gray-400 mb-4 text-sm">
              {this.state.error?.message || 'An unexpected error occurred'}
            </p>

            {/* Stack trace for debugging (collapsible) */}
            {process.env.NODE_ENV === 'development' && this.state.error?.stack && (
              <details className="mb-4 text-left">
                <summary className="text-xs text-gray-500 cursor-pointer hover:text-gray-400">
                  View error details
                </summary>
                <pre className="text-xs text-red-400 mt-2 p-3 bg-gray-950 rounded border border-gray-800 overflow-auto max-h-48">
                  {this.state.error.stack}
                </pre>
              </details>
            )}

            <button
              onClick={this.handleReset}
              className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors text-sm font-medium"
              aria-label="Try again"
            >
              Try again
            </button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
