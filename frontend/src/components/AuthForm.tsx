import { useState } from 'react';
import { Link } from 'react-router-dom';

interface AuthFormProps {
  title: string;
  mode: 'login' | 'register';
  onSubmit: (username: string, password: string) => Promise<void>;
  error: string | null;
}

export function AuthForm({ title, mode, onSubmit, error }: AuthFormProps) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [validationError, setValidationError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const validate = (): string | null => {
    if (username.length < 3) return 'Username must be at least 3 characters';
    if (username.length > 32) return 'Username must be at most 32 characters';
    if (!/^[a-zA-Z0-9_]+$/.test(username)) {
      return 'Username must be alphanumeric (letters, numbers, underscores only)';
    }
    if (password.length < 6) return 'Password must be at least 6 characters';
    return null;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setValidationError(null);

    const validationErr = validate();
    if (validationErr) {
      setValidationError(validationErr);
      return;
    }

    setIsSubmitting(true);
    try {
      await onSubmit(username, password);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="w-full max-w-md mx-auto">
      <h1 className="text-3xl font-bold text-center mb-8 text-white dark:text-gray-100">
        {title}
      </h1>

      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Username */}
        <div>
          <label htmlFor="username" className="block text-sm font-medium text-gray-300 mb-1">
            Username
          </label>
          <input
            id="username"
            type="text"
            autoComplete="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            className="w-full px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter your username"
          />
        </div>

        {/* Password */}
        <div>
          <label htmlFor="password" className="block text-sm font-medium text-gray-300 mb-1">
            Password
          </label>
          <input
            id="password"
            type="password"
            autoComplete={mode === 'login' ? 'current-password' : 'new-password'}
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="Enter your password"
          />
        </div>

        {/* Error display */}
        {(error || validationError) && (
          <div className="p-3 rounded-lg bg-red-900/50 border border-red-800 text-red-400 text-sm">
            {error || validationError}
          </div>
        )}

        {/* Submit */}
        <button
          type="submit"
          disabled={isSubmitting}
          className="w-full py-2.5 px-4 rounded-lg bg-blue-600 hover:bg-blue-700 disabled:bg-blue-800 disabled:opacity-50 text-white font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-900"
        >
          {isSubmitting ? 'Submitting...' : title}
        </button>

        {/* Switch link */}
        <p className="text-center text-gray-400">
          {mode === 'login' ? (
            <>
              Don&apos;t have an account?{' '}
              <Link to="/register" className="text-blue-400 hover:text-blue-300 font-medium">
                Register
              </Link>
            </>
          ) : (
            <>
              Already have an account?{' '}
              <Link to="/login" className="text-blue-400 hover:text-blue-300 font-medium">
                Login
              </Link>
            </>
          )}
        </p>
      </form>
    </div>
  );
}
