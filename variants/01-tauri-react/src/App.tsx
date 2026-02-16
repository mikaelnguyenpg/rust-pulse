import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

const PULSE_EVENT = import.meta.env.VITE_EVENT_PULSE_UPDATE || "";

interface ProcessPulse {
  pid: number;
  name: string;
  cpu_usage: number;
  mem_usage: number;
}

interface SystemPulse {
  total_cpu: number;
  total_mem: number;
  free_mem: number;
  processes: ProcessPulse[];
}

function App() {
  const [count, setCount] = useState(0);
  const [pulse, setPulse] = useState<SystemPulse | null>(null);

  console.log("PULSE_EVENT: ", PULSE_EVENT);

  useEffect(() => {
    // Lắng nghe event từ Rust
    const unlisten = listen(PULSE_EVENT, (event) => {
      const data: SystemPulse = JSON.parse(event.payload as string);
      console.log("Frontend received pulse: ", data);
      setPulse(data);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <>
      <div className="container">
        <header className="stats-header">
          <div className="card">
            <button onClick={() => setCount((count) => count + 1)}>
              Test count is {count}
            </button>
          </div>

          <h1>
            Rust Pulse <span className="badge">Baseline: React</span>
          </h1>

          {pulse && (
            <div className="global-stats">
              <div className="stat-card">
                <label>Total CPU</label>
                <div className="value">{pulse.total_cpu.toFixed(2)}%</div>
              </div>

              <div className="stat-card">
                <label>RAM Usage</label>

                <div className="value">
                  {(
                    (pulse.total_mem - pulse.free_mem) /
                    1024 /
                    1024 /
                    1024
                  ).toFixed(2)}{" "}
                  GB
                  <span>
                    {" "}
                    / {(pulse.total_mem / 1024 / 1024 / 1024).toFixed(2)} GB
                  </span>
                </div>
              </div>
            </div>
          )}
        </header>

        <main className="process-list">
          <table>
            <thead>
              <tr>
                <th>PID</th>
                <th>Process Name</th>
                <th className="right">CPU (%)</th>
                <th className="right">Mem (MB)</th>
              </tr>
            </thead>
            <tbody>
              {pulse?.processes.map((p) => (
                <tr key={p.pid}>
                  <td>{p.pid}</td>
                  <td className="proc-name">{p.name}</td>
                  <td className="right bold">{p.cpu_usage.toFixed(2)}</td>
                  <td className="right">
                    {(p.mem_usage / 1024 / 1024).toFixed(1)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </main>
      </div>

      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
