/* @refresh reload */
import { render } from 'solid-js/web';
import { Router, Route } from '@solidjs/router';
import { AuthProvider, ConfigProvider, CameraProvider, EventsProvider } from './stores';
import { LivePage, PlaybackPage, EventsPage, EvidencePage, ConfigPage, LoginPage } from './App';
import './index.css';

const root = document.getElementById('root');

if (!root) {
  throw new Error('Root element not found');
}

render(
  () => (
    <AuthProvider>
      <ConfigProvider>
        <CameraProvider>
          <EventsProvider>
            <Router>
              <Route path="/login" component={LoginPage} />
              <Route path="/live" component={LivePage} />
              <Route path="/playback" component={PlaybackPage} />
              <Route path="/events" component={EventsPage} />
              <Route path="/evidence" component={EvidencePage} />
              <Route path="/config/*" component={ConfigPage} />
              <Route path="/" component={LivePage} />
            </Router>
          </EventsProvider>
        </CameraProvider>
      </ConfigProvider>
    </AuthProvider>
  ),
  root
);
