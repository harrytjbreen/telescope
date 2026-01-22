import React from 'react';
import { useTelescope } from './useTelescope';

function App() {
  const { state, error, connected, goto, startTracking, stopTracking, reconnect } = useTelescope();

  if (error) {
    return (
      <div style={{ padding: '20px', color: 'red', border: '1px solid red' }}>
        <h3>Connection Error</h3>
        <p>{error}</p>
        <button onClick={reconnect}>Retry Connection</button>
      </div>
    );
  }

  if (!connected) {
    return <div style={{ padding: '20px' }}>Connecting to telescope...</div>;
  }

  if (!state) {
    return <div style={{ padding: '20px' }}>Waiting for initial state...</div>;
  }

  return (
    <div className="App" style={{ padding: '20px' }}>
      <h1>Telescope Controller</h1>
      <div className="status">
        <p>Status: <strong>{state.mount_status}</strong></p>
        <p>Current RA: {state.current_ra?.toFixed(4) ?? 'N/A'}</p>
        <p>Current Dec: {state.current_dec?.toFixed(4) ?? 'N/A'}</p>
      </div>

      <div className="controls">
        <button onClick={startTracking}>Start Tracking</button>
        <button onClick={stopTracking}>Stop Tracking</button>
        <button onClick={() => goto(10.5, 45.0)}>Goto Sample Coordinates</button>
      </div>
    </div>
  );
}

export default App;
