import React, { useState } from 'react';
import { Build } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Clock, DollarSign, CheckCircle, XCircle, Lightbulb, Target, ChevronRight } from 'lucide-react';

interface BuildGuideProps {
  build: Build;
}

export const BuildGuide: React.FC<BuildGuideProps> = ({ build }) => {
  const [activeTab, setActiveTab] = useState<'items' | 'playstyle' | 'gameplan'>('items');
  const { t } = useLanguage();

  const tabs = [
    { id: 'items', label: t('build.items'), icon: DollarSign },
    { id: 'playstyle', label: t('build.playstyle'), icon: Target },
    { id: 'gameplan', label: t('build.gameplan'), icon: Clock }
  ];

  return (
    <div className="build-guide card">
      {/* Tab Navigation */}
      <div className="tab-nav">
        {tabs.map(tab => {
          const Icon = tab.icon;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`tab-button ${activeTab === tab.id ? 'active' : ''}`}
            >
              <Icon size={16} />
              <span>{tab.label.toUpperCase()}</span>
            </button>
          );
        })}
      </div>

      {activeTab === 'items' && (
        <div className="build-section items-section">
          <h3>
            <DollarSign size={24} />
            {t('build.progression').toUpperCase()}
          </h3>
          
          <div className="item-list">
            {build.items.map((item, index) => (
              <div key={item.id} className="item-card">
                <div className="item-header">
                  <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                    <div style={{
                      background: 'rgba(0, 212, 255, 0.2)',
                      border: '1px solid #00d4ff',
                      borderRadius: '50%',
                      width: '2rem',
                      height: '2rem',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center'
                    }}>
                      <span style={{
                        fontSize: '0.875rem',
                        fontWeight: 'bold',
                        color: '#00d4ff',
                        fontFamily: 'Fira Code, monospace'
                      }}>
                        {index + 1}
                      </span>
                    </div>
                    <h4 className="item-name">{item.name}</h4>
                  </div>
                  
                  <div className="item-badges">
                    <span className={`badge ${item.phase.toLowerCase()}`}>
                      {t(`phase.${item.phase}`).toUpperCase()}
                    </span>
                    <span className={`badge ${item.priority.toLowerCase()}`}>
                      {t(`priority.${item.priority}`).toUpperCase()}
                    </span>
                    <span className="item-cost">{item.cost}G</span>
                  </div>
                </div>
                
                <p className="item-description">{item.description}</p>
                
                {/* Connection line to next item */}
                {index < build.items.length - 1 && (
                  <div style={{
                    position: 'absolute',
                    bottom: '-0.5rem',
                    left: '50%',
                    transform: 'translateX(-50%)'
                  }}>
                    <ChevronRight size={16} style={{ color: '#00d4ff', transform: 'rotate(90deg)' }} />
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'playstyle' && (
        <div className="build-section playstyle-section">
          <div className="playstyle-list">
            <div className="playstyle-group dos">
              <h4>
                <CheckCircle size={24} />
                {t('build.dos').toUpperCase()}
              </h4>
              <div className="playstyle-items">
                {build.playstyle.dos.map((item, index) => (
                  <div key={index} className="playstyle-item">
                    <div className="playstyle-text">
                      <span className="playstyle-marker">+</span>
                      {item}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div className="playstyle-group donts">
              <h4>
                <XCircle size={24} />
                {t('build.donts').toUpperCase()}
              </h4>
              <div className="playstyle-items">
                {build.playstyle.donts.map((item, index) => (
                  <div key={index} className="playstyle-item">
                    <div className="playstyle-text">
                      <span className="playstyle-marker">-</span>
                      {item}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div className="playstyle-group tips">
              <h4>
                <Lightbulb size={24} />
                {t('build.tips').toUpperCase()}
              </h4>
              <div className="playstyle-items">
                {build.playstyle.tips.map((item, index) => (
                  <div key={index} className="playstyle-item">
                    <div className="playstyle-text">
                      <span className="playstyle-marker">?</span>
                      {item}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'gameplan' && (
        <div className="build-section gameplan-section">
          <div className="gameplan-list">
            <div className="gameplan-phase gameplan-early">
              <h4>{t('build.early').toUpperCase()}</h4>
              <div className="gameplan-content">
                <p className="gameplan-text">{build.gameplan.early}</p>
              </div>
            </div>

            <div className="gameplan-phase gameplan-mid">
              <h4>{t('build.mid').toUpperCase()}</h4>
              <div className="gameplan-content">
                <p className="gameplan-text">{build.gameplan.mid}</p>
              </div>
            </div>

            <div className="gameplan-phase gameplan-late">
              <h4>{t('build.late').toUpperCase()}</h4>
              <div className="gameplan-content">
                <p className="gameplan-text">{build.gameplan.late}</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};