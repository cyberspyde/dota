import React, { useState } from 'react';
import { Build } from '../types';
import { useLanguage } from '../contexts/LanguageContext';
import { Clock, DollarSign, CheckCircle, XCircle, Lightbulb, Target } from 'lucide-react';

interface BuildGuideProps {
  build: Build;
}

const phaseColors = {
  Early: 'bg-green-500/20 text-green-600 dark:text-green-400',
  Mid: 'bg-yellow-500/20 text-yellow-600 dark:text-yellow-400',
  Late: 'bg-red-500/20 text-red-600 dark:text-red-400'
};

const priorityColors = {
  Core: 'bg-blue-500/20 text-blue-600 dark:text-blue-400',
  Situational: 'bg-purple-500/20 text-purple-600 dark:text-purple-400',
  Luxury: 'bg-orange-500/20 text-orange-600 dark:text-orange-400'
};

export const BuildGuide: React.FC<BuildGuideProps> = ({ build }) => {
  const [activeTab, setActiveTab] = useState<'items' | 'playstyle' | 'gameplan'>('items');
  const { t } = useLanguage();

  const tabs = [
    { id: 'items', label: t('build.items'), icon: DollarSign },
    { id: 'playstyle', label: t('build.playstyle'), icon: Target },
    { id: 'gameplan', label: t('build.gameplan'), icon: Clock }
  ];

  return (
    <div className="bg-white/80 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg p-4 lg:p-6 backdrop-blur-sm mx-4 lg:mx-0">
      <div className="flex flex-col sm:flex-row space-y-1 sm:space-y-0 sm:space-x-1 mb-6 bg-gray-100 dark:bg-gray-900/50 p-1 rounded-lg">
        {tabs.map(tab => {
          const Icon = tab.icon;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex-1 flex items-center justify-center space-x-2 py-2 px-2 lg:px-4 rounded-md transition-colors text-sm lg:text-base ${
                activeTab === tab.id
                  ? 'bg-blue-500/20 text-blue-600 dark:text-blue-400'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-300'
              }`}
            >
              <Icon size={16} className="lg:w-[18px] lg:h-[18px]" />
              <span className="font-medium">{tab.label}</span>
            </button>
          );
        })}
      </div>

      {activeTab === 'items' && (
        <div className="space-y-4">
          <h3 className="text-lg lg:text-xl font-bold text-gray-800 dark:text-white mb-4">{t('build.progression')}</h3>
          <div className="grid gap-3">
            {build.items.map((item, index) => (
              <div key={item.id} className="bg-gray-50 dark:bg-gray-900/50 p-3 lg:p-4 rounded-lg">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-2 space-y-2 sm:space-y-0">
                  <div className="flex items-center space-x-3">
                    <span className="text-lg lg:text-2xl font-bold text-gray-500">#{index + 1}</span>
                    <h4 className="text-base lg:text-lg font-semibold text-gray-800 dark:text-white">{item.name}</h4>
                  </div>
                  <div className="flex flex-wrap items-center gap-2">
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${phaseColors[item.phase]}`}>
                      {t(`phase.${item.phase}`)}
                    </span>
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${priorityColors[item.priority]}`}>
                      {t(`priority.${item.priority}`)}
                    </span>
                    <span className="text-xs lg:text-sm text-yellow-600 dark:text-yellow-400 font-semibold">{item.cost}g</span>
                  </div>
                </div>
                <p className="text-gray-700 dark:text-gray-300 text-sm">{item.description}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'playstyle' && (
        <div className="space-y-6">
          <div>
            <h3 className="text-lg lg:text-xl font-bold text-green-600 dark:text-green-400 mb-3 flex items-center">
              <CheckCircle size={18} className="mr-2 lg:w-5 lg:h-5" />
              {t('build.dos')}
            </h3>
            <ul className="space-y-2">
              {build.playstyle.dos.map((item, index) => (
                <li key={index} className="text-sm lg:text-base text-gray-700 dark:text-gray-300 flex items-start">
                  <span className="text-green-600 dark:text-green-400 mr-2">•</span>
                  {item}
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="text-lg lg:text-xl font-bold text-red-600 dark:text-red-400 mb-3 flex items-center">
              <XCircle size={18} className="mr-2 lg:w-5 lg:h-5" />
              {t('build.donts')}
            </h3>
            <ul className="space-y-2">
              {build.playstyle.donts.map((item, index) => (
                <li key={index} className="text-sm lg:text-base text-gray-700 dark:text-gray-300 flex items-start">
                  <span className="text-red-600 dark:text-red-400 mr-2">•</span>
                  {item}
                </li>
              ))}
            </ul>
          </div>

          <div>
            <h3 className="text-lg lg:text-xl font-bold text-blue-600 dark:text-blue-400 mb-3 flex items-center">
              <Lightbulb size={18} className="mr-2 lg:w-5 lg:h-5" />
              {t('build.tips')}
            </h3>
            <ul className="space-y-2">
              {build.playstyle.tips.map((item, index) => (
                <li key={index} className="text-sm lg:text-base text-gray-700 dark:text-gray-300 flex items-start">
                  <span className="text-blue-600 dark:text-blue-400 mr-2">•</span>
                  {item}
                </li>
              ))}
            </ul>
          </div>
        </div>
      )}

      {activeTab === 'gameplan' && (
        <div className="space-y-6">
          <div>
            <h3 className="text-lg lg:text-xl font-bold text-green-600 dark:text-green-400 mb-3">{t('build.early')}</h3>
            <p className="text-sm lg:text-base text-gray-700 dark:text-gray-300 bg-gray-50 dark:bg-gray-900/50 p-3 lg:p-4 rounded-lg">{build.gameplan.early}</p>
          </div>

          <div>
            <h3 className="text-lg lg:text-xl font-bold text-yellow-600 dark:text-yellow-400 mb-3">{t('build.mid')}</h3>
            <p className="text-sm lg:text-base text-gray-700 dark:text-gray-300 bg-gray-50 dark:bg-gray-900/50 p-3 lg:p-4 rounded-lg">{build.gameplan.mid}</p>
          </div>

          <div>
            <h3 className="text-lg lg:text-xl font-bold text-red-600 dark:text-red-400 mb-3">{t('build.late')}</h3>
            <p className="text-sm lg:text-base text-gray-700 dark:text-gray-300 bg-gray-50 dark:bg-gray-900/50 p-3 lg:p-4 rounded-lg">{build.gameplan.late}</p>
          </div>
        </div>
      )}
    </div>
  );
};