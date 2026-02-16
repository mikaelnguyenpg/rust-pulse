import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [count, setCount] = useState(0);
  const [pulse, setPulse] = useState<any>(null);

  useEffect(() => {
    const unlisten = listen("pulse-update", (event) => {
      const data = JSON.parse(event.payload as string);
      setPulse(data);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      {pulse ? (
        <div>
          <p>CPU: {pulse.total_cpu.toFixed(2)}%</p>
          <p>RAM: {(pulse.total_mem / 1024 / 1024 / 1024).toFixed(2)} GB</p>
        </div>
      ) : (
        <p>Waiting for pulse...</p>
      )}
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
