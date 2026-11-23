import React, { useState, useEffect } from 'react';
import { useLanguage } from '../contexts/LanguageContext';
import { apiService } from '../services/api';
import { Wifi, WifiOff, RefreshCw, Database, Users, BookOpen } from 'lucide-react';

export const ConnectionStatus: React.FC = () => {
  const [status, setStatus] = useState<'connected' | 'disconnected' | 'checking'>('checking');
  const [heroCount, setHeroCount] = useState<number>(0);
  const [buildCount, setBuildCount] = useState<number>(0);
  const { t } = useLanguage();

  const checkConnection = async () => {
    try {
      setStatus('checking');
      
      const [heroesResult, builds] = await Promise.all([
        apiService.getHeroes(1000, 0), // Get first 1000 heroes to get total count
        apiService.getBuilds()
      ]);
      
      setHeroCount(heroesResult.total);
      setBuildCount(builds.length);
      setStatus('connected');
    } catch (error) {
      console.error('Connection check failed:', error);
      setStatus('disconnected');
      setHeroCount(0);
      setBuildCount(0);
    }
  };

  useEffect(() => {
    checkConnection();
    const interval = setInterval(checkConnection, 30000);
    return () => clearInterval(interval);
  }, []);

  const StatusIcon = () => {
    switch (status) {
      case 'connected': return <Database size={16} style={{ color: '#00ff88' }} />;
      case 'disconnected': return <WifiOff size={16} style={{ color: '#ff4444' }} />;
      case 'checking': return <RefreshCw size={16} style={{ color: '#ffff00' }} />;
      default: return <WifiOff size={16} style={{ color: 'rgba(128, 128, 128, 0.5)' }} />;
    }
  };

  return (
    <div className="connection-status">
      <div className="status-content">
        <div className="status-left">
          <div className="status-indicator">
            <div className={`status-dot ${status}`}></div>
            <StatusIcon />
            <span className="status-text">
              {status === 'connected' ? 'NEURAL NETWORK ONLINE' : 
               status === 'disconnected' ? 'CONNECTION LOST' : 
               'ESTABLISHING LINK...'}
            </span>
          </div>
          
          {status === 'connected' && (
            <div className="status-stats">
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                <Users size={12} style={{ color: '#00ff88' }} />
                <span style={{ color: '#00ff88' }}>{heroCount}</span>
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                <BookOpen size={12} style={{ color: '#8b5cf6' }} />
                <span style={{ color: '#8b5cf6' }}>{buildCount}</span>
              </div>
            </div>
          )}
        </div>
        
        <div className="status-right">
          <button
            onClick={checkConnection}
            className="btn sync-btn"
            disabled={status === 'checking'}
          >
            <RefreshCw size={12} style={{ animation: status === 'checking' ? 'spin 1s linear infinite' : 'none' }} />
            <span>SYNC</span>
          </button>
          
          {status === 'disconnected' && (
            <div style={{ 
              color: '#ff4444',
              fontSize: '0.75rem',
              fontFamily: 'Fira Code, monospace'
            }}>
              DATABASE OFFLINE
            </div>
          )}
          
          {status === 'connected' && heroCount === 0 && (
            <div style={{ 
              color: '#ffff00',
              fontSize: '0.75rem',
              fontFamily: 'Fira Code, monospace'
            }}>
              RUN: npm run db:populate
            </div>
          )}
        </div>
      </div>
    </div>
  );
};