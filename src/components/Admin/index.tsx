import React, { useState } from 'react';
import { AdminDashboard } from './AdminDashboard';
import './admin-panel.css';

export const Admin: React.FC = () => {
  const [password, setPassword] = useState('');
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [error, setError] = useState('');

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    // IMPORTANT: This is a simple, insecure password check.
    // In a real application, use a proper authentication service.
    if (password === 'ilhomcha') {
      setIsAuthenticated(true);
      setError('');
    } else {
      setError('Invalid password');
    }
  };

  if (isAuthenticated) {
    return <AdminDashboard />;
  }

  return (
    <div className="admin-layout">
      <div className="admin-login-screen">
        <div className="admin-login-card">
          <h1 className="admin-login-title">Dota Guide Admin</h1>
          <p className="admin-login-subtitle">Enter your credentials to access the dashboard</p>
          <form onSubmit={handleLogin} className="space-y-6">
            <div>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Enter admin password"
                className="admin-login-input"
                autoFocus
              />
              {error && <p className="admin-login-error">{error}</p>}
            </div>
            <button type="submit" className="admin-login-button">
              Access Dashboard
            </button>
          </form>
        </div>
      </div>
    </div>
  );
};