
import React, { useState } from 'react';
import { HeroManager } from './HeroManager';
import { BuildManager } from './BuildManager';
import AppBar from '@mui/material/AppBar';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import DashboardIcon from '@mui/icons-material/Dashboard';
import PeopleIcon from '@mui/icons-material/People';
import BuildIcon from '@mui/icons-material/Build';

type AdminTab = 'heroes' | 'builds';

export const AdminDashboard: React.FC = () => {
  const [activeTab, setActiveTab] = useState<AdminTab>('heroes');

  return (
    <Box sx={{ bgcolor: 'background.default', minHeight: '100vh' }}>
      <AppBar position="static" color="default" elevation={1}>
        <Container maxWidth="lg" sx={{ display: 'flex', alignItems: 'center', py: 2 }}>
          <DashboardIcon sx={{ mr: 2 }} color="primary" fontSize="large" />
          <Typography variant="h5" sx={{ flexGrow: 1, fontWeight: 700 }}>
            Admin Dashboard
          </Typography>
          <Tabs
            value={activeTab}
            onChange={(_, val) => setActiveTab(val)}
            indicatorColor="primary"
            textColor="primary"
          >
            <Tab value="heroes" icon={<PeopleIcon />} label="Heroes" />
            <Tab value="builds" icon={<BuildIcon />} label="Builds" />
          </Tabs>
        </Container>
      </AppBar>
      <Container maxWidth="lg" sx={{ py: 4 }}>
        {activeTab === 'heroes' && <HeroManager />}
        {activeTab === 'builds' && <BuildManager />}
      </Container>
    </Box>
  );
};