import { Routes, Route, BrowserRouter } from 'react-router-dom';
import HomePage from './pages/home';
import SetupPage from './pages/setup';
import Authenticate from './pages/login/Authenticate';
import { AccessTokenWrapper } from '@calimero-is-near/calimero-p2p-sdk';
import { getNodeUrl } from './utils/node';
import LandingPage from './pages/landing-page';
import DashboardPage from './pages/dashboard-page';

export default function App() {
  return (
    <AccessTokenWrapper getNodeUrl={getNodeUrl}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<LandingPage />} />
          <Route path="/setup" element={<SetupPage />} />
          <Route path="/auth" element={<Authenticate />} />
          <Route path="/home" element={<HomePage />} />
          <Route path='/dashboard' element={<DashboardPage />} />
        </Routes>
      </BrowserRouter>
    </AccessTokenWrapper>
  );
}
