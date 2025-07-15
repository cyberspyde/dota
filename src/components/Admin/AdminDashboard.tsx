import React, { useState } from 'react';
import { HeroManager } from './HeroManager';
import { BuildManager } from './BuildManager';
import { Shield, Swords, LayoutDashboard } from 'lucide-react';

type AdminTab = 'heroes' | 'builds';

export const AdminDashboard: React.FC = () => {
  const [activeTab, setActiveTab] = useState<AdminTab>('heroes');

  return (
    <div className="admin-layout">
      <div className="admin-dashboard">
        <header className="admin-header">
          <div className="admin-title-section">
            <div className="admin-icon">
              <LayoutDashboard size={24} />
            </div>
            <h1 className="admin-dashboard-title">Admin Dashboard</h1>
          </div>
          
          <div className="admin-tabs">
            <button
              onClick={() => setActiveTab('heroes')}
              className={`admin-tab-button ${activeTab === 'heroes' ? 'active' : ''}`}
              data-state={activeTab === 'heroes' ? 'active' : 'inactive'}
            >
              <Shield size={18} />
              <span>Heroes</span>
            </button>
            <button
              onClick={() => setActiveTab('builds')}
              className={`admin-tab-button ${activeTab === 'builds' ? 'active' : ''}`}
              data-state={activeTab === 'builds' ? 'active' : 'inactive'}
            >
              <Swords size={18} />
              <span>Builds</span>
            </button>
          </div>
        </header>

        <main className="admin-content">
          {activeTab === 'heroes' && <HeroManager />}
          {activeTab === 'builds' && <BuildManager />}
        </main>
      </div>
    </div>
  );
};