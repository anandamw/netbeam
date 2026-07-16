import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { Server, Wifi, Send, MessageSquare, Monitor, Zap, CheckCircle2, AlertCircle, Play, FileIcon } from 'lucide-react';

type NetworkMessage = [string, string];

interface ProgressEvent {
  transfer_id: string;
  file_name: string;
  sent_bytes: number;
  total_bytes: number;
  percentage: number;
  speed_mbps: number;
  is_done: boolean;
}

interface DeviceInfo {
  device_name: string;
  ip: string;
  port: number;
}

function App() {
  const [serverPort, setServerPort] = useState<number>(3000);
  const [isServerRunning, setIsServerRunning] = useState(false);
  const [actualPort, setActualPort] = useState<number | null>(null);
  const [myDeviceName, setMyDeviceName] = useState('My PC');

  const [targetIp, setTargetIp] = useState('');
  const [targetPort, setTargetPort] = useState<number>(3000);
  const [targetName, setTargetName] = useState('');

  const [msgToSend, setMsgToSend] = useState('');
  const [chatLog, setChatLog] = useState<{ sender: string, msg: string, time: string }[]>([]);
  const chatEndRef = useRef<HTMLDivElement>(null);

  const [progresses, setProgresses] = useState<Record<string, ProgressEvent>>({});
  const [discoveredDevices, setDiscoveredDevices] = useState<Record<string, DeviceInfo>>({});

  // Phase 6: Transfer Queue & Drag Drop
  const [transferQueue, setTransferQueue] = useState<string[]>([]);
  const [isTransferring, setIsTransferring] = useState(false);
  const [isDragging, setIsDragging] = useState(false);

  // Phase 6: Pause, Resume, Retry
  const [isQueuePaused, setIsQueuePaused] = useState(false);
  const [failedTransfers, setFailedTransfers] = useState<string[]>([]);

  // Phase 7: Transfer Confirmation
  const [transferRequests, setTransferRequests] = useState<{ transfer_id: string, file_name: string, file_size: number, sender: string }[]>([]);

  // Phase 8: History and Recent Devices
  const [history, setHistory] = useState<any[]>([]);
  const [recentDevices, setRecentDevices] = useState<any[]>([]);

  useEffect(() => {
    const loadStore = async () => {
      try {
        const hist = await invoke<any[]>('get_history');
        const devs = await invoke<any[]>('get_recent_devices');
        setHistory(hist);
        setRecentDevices(devs);
      } catch (e) {
        console.error("Store error:", e);
      }
    };
    loadStore();
  }, []);

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [chatLog]);

  useEffect(() => {
    // Original listeners
    const unlistenServer = listen<number>('server-started', async (event) => {
      setIsServerRunning(true);
      setActualPort(event.payload);
      addLog('System', `Server online on port ${event.payload}`);

      try {
        await invoke('start_discovery', { deviceName: myDeviceName, tcpPort: event.payload });
        addLog('System', `Radar activated, broadcasting presence...`);
      } catch (e) {
        addLog('Error', `Failed to start radar: ${e}`);
      }
    });

    const unlistenMessage = listen<NetworkMessage>('network-message', (event) => {
      const [peer, msg] = event.payload;
      addLog(peer, msg);
    });

    const unlistenProgress = listen<ProgressEvent>('transfer-progress', (event) => {
      setProgresses(prev => ({ ...prev, [event.payload.transfer_id]: event.payload }));
      if (event.payload.is_done) {
        setIsTransferring(false);
        const record = {
          file_name: event.payload.file_name,
          file_size: event.payload.total_bytes,
          timestamp: new Date().toISOString(),
          peer: 'Remote',
          direction: 'completed',
          status: 'success'
        };
        invoke('add_history_record', { record }).catch(console.error);
        setHistory(prev => [record, ...prev].slice(0, 100));
      }
    });

    const unlistenDiscovery = listen<DeviceInfo>('device-discovered', (event) => {
      setDiscoveredDevices(prev => ({
        ...prev,
        [`${event.payload.ip}:${event.payload.port}`]: event.payload
      }));
    });

    const unlistenTransferRequest = listen<{ transfer_id: string, file_name: string, file_size: number, sender: string }>('transfer-request', (event) => {
      setTransferRequests(prev => [...prev, event.payload]);
    });

    // Tauri window drag and drop listeners
    const unlistenDragEnter = listen('tauri://drag-enter', () => {
      setIsDragging(true);
    });

    const unlistenDragLeave = listen('tauri://drag-leave', () => {
      setIsDragging(false);
    });

    const unlistenDragDrop = listen<{ paths: string[] }>('tauri://drag-drop', (event) => {
      setIsDragging(false);

      let paths: string[] = [];
      if (event.payload && Array.isArray(event.payload.paths)) {
        paths = event.payload.paths;
      } else if (Array.isArray(event.payload)) {
        paths = event.payload as unknown as string[];
      }

      if (paths.length > 0) {
        setTransferQueue(prev => [...prev, ...paths]);
        addLog('System', `Added ${paths.length} file(s) via Drag & Drop.`);
      }
    });

    return () => {
      unlistenServer.then(f => f());
      unlistenMessage.then(f => f());
      unlistenProgress.then(f => f());
      unlistenDiscovery.then(f => f());
      unlistenTransferRequest.then(f => f());
      unlistenDragEnter.then(f => f());
      unlistenDragLeave.then(f => f());
      unlistenDragDrop.then(f => f());
    };
  }, [myDeviceName]);

  // Queue Processing Engine
  useEffect(() => {
    if (!isTransferring && transferQueue.length > 0 && targetIp && !isQueuePaused) {
      const nextFile = transferQueue[0];
      setTransferQueue(prev => prev.slice(1));
      setIsTransferring(true);

      const sendNext = async () => {
        try {
          addLog('System', `Sending: ${nextFile.split('\\').pop() || nextFile.split('/').pop()}`);
          await invoke<string>('send_file', {
            ip: targetIp,
            port: Number(targetPort),
            filePath: nextFile
          });
        } catch (e) {
          console.error(e);
          addLog('Error', `Send failed for ${nextFile.split('\\').pop() || nextFile.split('/').pop()}: ${e}`);
          setFailedTransfers(prev => [...prev, nextFile]);
          setIsTransferring(false); // Move to next on error
        }
      };

      sendNext();
    }
  }, [isTransferring, transferQueue, targetIp, targetPort, isQueuePaused]);

  const addLog = (sender: string, msg: string) => {
    const time = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    setChatLog(prev => [...prev, { sender, msg, time }]);
  };

  const handleStartServer = async () => {
    try {
      await invoke('start_server', { port: Number(serverPort) });
    } catch (e) {
      console.error(e);
      addLog('Error', `${e}`);
    }
  };

  const handleConnect = async () => {
    try {
      const res = await invoke<string>('connect_client', { ip: targetIp, port: Number(targetPort) });
      addLog('System', res);

      const newDev = { ip: targetIp, port: Number(targetPort), last_seen: new Date().toISOString() };
      invoke('add_recent_device', { device: newDev }).catch(console.error);
      setRecentDevices(prev => {
        const list = prev.filter(d => d.ip !== targetIp);
        return [newDev, ...list].slice(0, 20);
      });

    } catch (e) {
      console.error(e);
      addLog('Error', `${e}`);
    }
  };

  const handleSendMessage = async () => {
    if (!msgToSend.trim()) return;
    try {
      await invoke('send_message', { ip: targetIp, msg: msgToSend });
      addLog('Me', `-> ${targetIp}: ${msgToSend}`);
      setMsgToSend('');
    } catch (e) {
      console.error(e);
      addLog('Error', `${e}`);
    }
  };

  const handleSendFile = async () => {
    if (!targetIp) return;
    try {
      const selected = await open({
        multiple: true,
        title: 'Select file(s) to send'
      });
      if (selected === null) {
        return;
      }

      const files = Array.isArray(selected) ? selected : [selected];
      setTransferQueue(prev => [...prev, ...files]);
      addLog('System', `Added ${files.length} file(s) to transfer queue.`);

    } catch (e) {
      console.error(e);
      addLog('Error', `File picker error: ${e}`);
    }
  };

  const selectDevice = (device: DeviceInfo) => {
    setTargetIp(device.ip);
    setTargetPort(device.port);
    setTargetName(device.device_name);
  };

  return (
    <div className="min-h-screen p-6 md:p-10 max-w-7xl mx-auto flex flex-col gap-8 relative z-10">

      {/* Drag & Drop Overlay */}
      {isDragging && (
        <div className="fixed inset-0 z-50 bg-primary/10 backdrop-blur-sm border-[6px] border-dashed border-primary flex items-center justify-center pointer-events-none transition-all duration-300">
          <div className="bg-black/80 px-10 py-8 rounded-2xl flex flex-col items-center gap-4 shadow-2xl shadow-primary/20">
            <div className="w-20 h-20 bg-primary/20 rounded-full flex items-center justify-center animate-bounce">
              <FileIcon className="w-10 h-10 text-primary" />
            </div>
            <h2 className="text-3xl font-bold text-white">Drop File(s) Here</h2>
            <p className="text-gray-400">Release to add to the transfer queue</p>
          </div>
        </div>
      )}

      {/* Transfer Request Overlay */}
      {transferRequests.length > 0 && (
        <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
          <div className="bg-gray-900 border border-primary/30 rounded-2xl p-6 shadow-2xl max-w-sm w-full animate-in fade-in zoom-in duration-200">
            <div className="flex justify-center mb-4 text-primary animate-pulse">
              <AlertCircle className="w-12 h-12" />
            </div>
            <h3 className="text-xl font-bold text-white text-center mb-2">Incoming File</h3>
            <p className="text-gray-300 text-center mb-6">
              <strong className="text-white">{transferRequests[0].sender}</strong> wants to send you <br />
              <span className="text-primary font-medium break-all">{transferRequests[0].file_name}</span> <br />
              <span className="text-sm text-gray-400">({(transferRequests[0].file_size / 1048576).toFixed(2)} MB)</span>
            </p>
            <div className="flex gap-3">
              <button
                onClick={async () => {
                  const req = transferRequests[0];
                  try {
                    await invoke('reject_transfer', { transferId: req.transfer_id });
                    addLog('System', `Rejected transfer of ${req.file_name}`);
                  } catch (e) {
                    console.error(e);
                  }
                  setTransferRequests(prev => prev.slice(1));
                }}
                className="flex-1 bg-red-500/10 hover:bg-red-500/20 text-red-500 font-semibold py-2.5 rounded-xl transition-colors border border-red-500/20"
              >
                Reject
              </button>
              <button
                onClick={async () => {
                  const req = transferRequests[0];
                  try {
                    await invoke('accept_transfer', { transferId: req.transfer_id });
                    addLog('System', `Accepted transfer of ${req.file_name}`);
                  } catch (e) {
                    console.error(e);
                  }
                  setTransferRequests(prev => prev.slice(1));
                }}
                className="flex-1 bg-primary text-black font-bold py-2.5 rounded-xl hover:bg-primary-hover transition-colors shadow-lg shadow-primary/20"
              >
                Accept
              </button>
            </div>
          </div>
        </div>
      )}

      <header className="flex items-center justify-between pb-4 border-b border-white/5">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-primary to-primary-hover flex items-center justify-center shadow-lg shadow-primary/20 animate-float">
            <Zap className="text-[#121212] w-6 h-6" />
          </div>
          <div>
            <h1 className="text-3xl font-bold tracking-tight text-white" style={{ textShadow: '0 0 20px rgba(250, 204, 21, 0.4)' }}>LAN Share</h1>
            <p className="text-gray-400 text-sm font-medium">Lightning Fast Local Transfer</p>
          </div>
        </div>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">

        {/* Left Sidebar */}
        <div className="lg:col-span-4 flex flex-col gap-6">

          {/* Identity & Server */}
          <div className="glass rounded-2xl p-6 relative overflow-hidden group">
            <div className="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity text-primary">
              <Monitor className="w-24 h-24" />
            </div>

            <h2 className="text-lg font-semibold text-white mb-5 flex items-center gap-2">
              <Server className="w-5 h-5 text-primary" /> My Station
            </h2>

            <div className="space-y-4 relative z-10">
              <div>
                <label className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-1 block">Device Name</label>
                <input
                  type="text"
                  value={myDeviceName}
                  onChange={(e) => setMyDeviceName(e.target.value)}
                  disabled={isServerRunning}
                  className="w-full px-4 py-2.5 bg-black/40 border border-white/10 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/50 text-white placeholder-gray-500 transition-all"
                  placeholder="e.g. John's Mac"
                />
              </div>
              <div className="flex gap-3">
                <div className="flex-1">
                  <label className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-1 block">Port</label>
                  <input
                    type="number"
                    value={serverPort}
                    onChange={(e) => setServerPort(Number(e.target.value))}
                    disabled={isServerRunning}
                    className="w-full px-4 py-2.5 bg-black/40 border border-white/10 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/50 text-white transition-all"
                  />
                </div>
                <div className="flex-1 flex items-end">
                  <button
                    onClick={handleStartServer}
                    disabled={isServerRunning}
                    className="w-full h-[46px] bg-primary hover:bg-primary-hover text-[#121212] rounded-xl font-bold disabled:opacity-50 disabled:bg-gray-700 disabled:text-gray-400 transition-all flex items-center justify-center gap-2 shadow-[0_0_15px_rgba(250,204,21,0.2)]"
                  >
                    {isServerRunning ? <><CheckCircle2 className="w-4 h-4 text-[#121212]" /> Online</> : <><Play className="w-4 h-4" /> Start</>}
                  </button>
                </div>
              </div>
            </div>
          </div>

          {/* Radar */}
          <div className="glass rounded-2xl p-0 flex-1 flex flex-col overflow-hidden min-h-[300px]">
            <div className="p-5 border-b border-white/5 bg-black/20 flex items-center justify-between">
              <h2 className="text-lg font-semibold text-white flex items-center gap-2">
                <Wifi className="w-5 h-5 text-primary" /> Radar
              </h2>
              {isServerRunning ? (
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-primary"></span>
                </span>
              ) : (
                <span className="w-2 h-2 rounded-full bg-gray-600"></span>
              )}
            </div>

            <div className="p-3 flex-1 overflow-y-auto space-y-2 relative">
              {!isServerRunning && (
                <div className="absolute inset-0 flex items-center justify-center text-gray-500 text-sm text-center p-6">
                  Start your server to activate the radar.
                </div>
              )}
              {isServerRunning && Object.values(discoveredDevices).length === 0 && (
                <div className="absolute inset-0 flex flex-col items-center justify-center text-gray-500 text-sm">
                  <div className="w-16 h-16 border-2 border-dashed border-gray-600 rounded-full animate-[spin_4s_linear_infinite] mb-4"></div>
                  Scanning network...
                </div>
              )}
              {Object.values(discoveredDevices).map((device, i) => (
                <button
                  key={i}
                  onClick={() => selectDevice(device)}
                  className={`w-full text-left p-4 rounded-xl border transition-all duration-300 flex items-center gap-4 ${targetIp === device.ip && targetPort === device.port
                    ? 'bg-primary/20 border-primary/50 shadow-[0_0_15px_rgba(250,204,21,0.2)]'
                    : 'bg-black/30 border-white/5 hover:bg-black/50 hover:border-white/10'
                    }`}
                >
                  <div className="w-10 h-10 rounded-full bg-gradient-to-br from-gray-800 to-gray-900 border border-white/10 flex items-center justify-center shrink-0">
                    <Monitor className={`w-5 h-5 ${targetIp === device.ip ? 'text-primary' : 'text-gray-400'}`} />
                  </div>
                  <div className="overflow-hidden">
                    <div className="font-semibold text-white truncate">{device.device_name}</div>
                    <div className="text-xs text-gray-400 mt-0.5">{device.ip}:{device.port}</div>
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Right Column */}
        <div className="lg:col-span-8 flex flex-col gap-6">

          {/* Target & Actions */}
          <div className="glass rounded-2xl p-6">
            <h2 className="text-lg font-semibold text-white mb-5 flex items-center gap-2">
              <Send className="w-5 h-5 text-primary" /> Transfer Target
            </h2>

            <div className="flex flex-col sm:flex-row gap-4 mb-6">
              <div className="flex-[2]">
                <input
                  type="text"
                  value={targetIp}
                  onChange={(e) => setTargetIp(e.target.value)}
                  className="w-full px-4 py-3 bg-black/40 border border-white/10 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/50 text-white placeholder-gray-600"
                  placeholder="Target IP (Select from Radar)"
                />
              </div>
              <div className="flex-1">
                <input
                  type="number"
                  value={targetPort}
                  onChange={(e) => setTargetPort(Number(e.target.value))}
                  className="w-full px-4 py-3 bg-black/40 border border-white/10 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/50 text-white"
                  placeholder="Port"
                />
              </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-4">
              <button
                onClick={handleConnect}
                disabled={!targetIp}
                className="flex-1 py-3 px-4 bg-gray-800 hover:bg-gray-700 text-white rounded-xl font-semibold transition-all disabled:opacity-50 border border-white/5 flex justify-center items-center gap-2"
              >
                <CheckCircle2 className="w-4 h-4 text-gray-400" /> Ping / Connect
              </button>
              <button
                onClick={handleSendFile}
                disabled={!targetIp}
                className="flex-[2] py-3 px-4 bg-gradient-to-r from-primary to-primary-hover text-[#121212] rounded-xl font-bold transition-all shadow-[0_0_20px_rgba(250,204,21,0.3)] hover:shadow-[0_0_30px_rgba(250,204,21,0.5)] disabled:opacity-50 disabled:shadow-none flex justify-center items-center gap-2 transform active:scale-[0.98]"
              >
                <FileIcon className="w-5 h-5" /> Select & Send File
              </button>
            </div>
            
            {recentDevices.length > 0 && (
              <div className="mt-6 pt-5 border-t border-white/5 flex flex-wrap gap-2 items-center">
                <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider mr-2">Recent:</span>
                {recentDevices.map(dev => (
                  <button
                    key={dev.ip}
                    onClick={() => { setTargetIp(dev.ip); setTargetPort(dev.port.toString()); }}
                    className="bg-black/30 hover:bg-primary/20 text-gray-300 hover:text-primary px-3 py-1.5 rounded-lg text-xs font-medium transition-colors border border-white/10 hover:border-primary/50"
                  >
                    {dev.ip}:{dev.port}
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Transfers in Progress */}
          {(Object.values(progresses).length > 0 || transferQueue.length > 0 || failedTransfers.length > 0) && (
            <div className="glass rounded-2xl p-6">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-lg font-semibold text-white">Active Transfers</h2>
                <div className="flex items-center gap-2">
                  {transferQueue.length > 0 && (
                    <span className={`px-3 py-1 rounded-full text-xs font-semibold transition-colors ${isQueuePaused ? 'bg-orange-500/20 text-orange-400' : 'bg-primary/20 text-primary animate-pulse'}`}>
                      {transferQueue.length} file(s) in queue
                    </span>
                  )}
                  {transferQueue.length > 0 && (
                    <button
                      onClick={() => setIsQueuePaused(!isQueuePaused)}
                      className={`px-3 py-1 rounded-full text-xs font-semibold border transition-all flex items-center gap-1 ${isQueuePaused ? 'bg-green-500/20 text-green-400 border-green-500/30 hover:bg-green-500/30' : 'bg-gray-800 text-gray-300 border-white/10 hover:bg-gray-700'}`}
                    >
                      {isQueuePaused ? '▶ Resume' : '⏸ Pause'}
                    </button>
                  )}
                </div>
              </div>
              <div className="space-y-4">
                {Object.values(progresses).map(p => (
                  <div key={p.transfer_id} className="bg-black/40 p-4 rounded-xl border border-white/5 relative overflow-hidden">
                    {/* Animated background glow for active transfers */}
                    {!p.is_done && (
                      <div className="absolute top-0 right-0 w-32 h-32 bg-primary/10 blur-2xl rounded-full animate-pulse"></div>
                    )}

                    <div className="flex justify-between items-center mb-3 relative z-10">
                      <div className="flex items-center gap-3 overflow-hidden">
                        <div className={`w-8 h-8 rounded-lg flex items-center justify-center shrink-0 ${p.is_done ? 'bg-green-500/20 text-green-400' : 'bg-primary/20 text-primary'}`}>
                          {p.is_done ? <CheckCircle2 className="w-4 h-4" /> : <Monitor className="w-4 h-4 animate-bounce" />}
                        </div>
                        <span className="font-medium text-gray-200 truncate">{p.file_name}</span>
                      </div>
                      <span className={`text-sm font-bold whitespace-nowrap ml-4 ${p.is_done ? 'text-green-400' : 'text-primary'}`}>
                        {p.is_done ? 'Completed' : `${p.speed_mbps.toFixed(1)} MB/s`}
                      </span>
                    </div>

                    <div className="w-full bg-gray-900 rounded-full h-2.5 overflow-hidden border border-black relative z-10">
                      <div
                        className={`h-full transition-all duration-300 ease-out relative ${p.is_done ? 'bg-green-500' : 'bg-gradient-to-r from-primary to-primary-hover'}`}
                        style={{ width: `${p.percentage}%` }}
                      >
                        {!p.is_done && (
                          <div className="absolute top-0 right-0 bottom-0 w-20 bg-gradient-to-r from-transparent to-white/30 -skew-x-12 animate-[translateX_2s_infinite]"></div>
                        )}
                      </div>
                    </div>

                    <div className="flex justify-between text-xs text-gray-400 mt-2 font-mono relative z-10">
                      <span>{(p.sent_bytes / 1048576).toFixed(1)} / {(p.total_bytes / 1048576).toFixed(1)} MB</span>
                      <span>{p.percentage.toFixed(1)}%</span>
                    </div>
                  </div>
                ))}

                {failedTransfers.length > 0 && failedTransfers.map((path, idx) => {
                  const fileName = path.split('\\').pop() || path.split('/').pop();
                  return (
                    <div key={`failed-${idx}`} className="bg-red-500/10 p-4 rounded-xl border border-red-500/20 relative overflow-hidden flex justify-between items-center">
                      <div className="flex items-center gap-3 overflow-hidden">
                        <div className="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 bg-red-500/20 text-red-400">
                          <AlertCircle className="w-4 h-4" />
                        </div>
                        <div>
                          <span className="font-medium text-gray-200 block truncate">{fileName}</span>
                          <span className="text-xs text-red-400">Transfer failed</span>
                        </div>
                      </div>
                      <button
                        onClick={() => {
                          setFailedTransfers(prev => prev.filter((_, i) => i !== idx));
                          setTransferQueue(prev => [...prev, path]);
                          addLog('System', `Retrying file: ${fileName}`);
                        }}
                        className="px-3 py-1.5 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded-lg text-xs font-semibold flex items-center gap-1 transition-colors border border-red-500/20 shrink-0"
                      >
                        <Play className="w-3 h-3" /> Retry
                      </button>
                    </div>
                  );
                })}
              </div>
            </div>
          )}

          {/* Chat & Logs */}
          <div className="glass rounded-2xl flex-1 flex flex-col min-h-[300px] overflow-hidden">
            <div className="p-4 border-b border-white/5 bg-black/20">
              <h2 className="text-lg font-semibold text-white flex items-center gap-2">
                <MessageSquare className="w-5 h-5 text-gray-400" /> Event Log & Chat
              </h2>
            </div>

            <div className="flex-1 p-4 overflow-y-auto space-y-3 bg-black/10">
              {chatLog.length === 0 ? (
                <div className="h-full flex flex-col items-center justify-center text-gray-500 opacity-50">
                  <AlertCircle className="w-8 h-8 mb-2" />
                  <p className="text-sm">Activity log is empty</p>
                </div>
              ) : (
                chatLog.map((log, i) => {
                  const isSystem = log.sender === 'System' || log.sender === 'Error';
                  const isMe = log.sender === 'Me';

                  return (
                    <div key={i} className={`flex flex-col max-w-[85%] ${isMe ? 'ml-auto items-end' : 'mr-auto items-start'}`}>
                      <div className="text-[10px] text-gray-500 mb-1 flex items-center gap-1.5 px-1">
                        <span className="font-semibold text-gray-400">{log.sender}</span>
                        <span>•</span>
                        <span>{log.time}</span>
                      </div>
                      <div className={`px-4 py-2.5 rounded-2xl text-sm ${log.sender === 'Error' ? 'bg-red-500/10 text-red-400 border border-red-500/20 rounded-tl-sm' :
                        log.sender === 'System' ? 'bg-gray-800/50 text-gray-300 border border-white/5 rounded-tl-sm' :
                          isMe ? 'bg-primary text-[#121212] rounded-tr-sm font-medium' :
                            'bg-gray-800 text-gray-100 rounded-tl-sm'
                        }`}>
                        {log.msg}
                      </div>
                    </div>
                  );
                })
              )}
              <div ref={chatEndRef} />
            </div>

            <div className="p-3 bg-black/30 border-t border-white/5 flex gap-2">
              <input
                type="text"
                value={msgToSend}
                onChange={(e) => setMsgToSend(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
                disabled={!targetIp}
                className="flex-1 px-4 py-3 bg-gray-900 border border-white/10 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/50 text-white placeholder-gray-600 disabled:opacity-50 text-sm"
                placeholder="Type a message..."
              />
              <button
                onClick={handleSendMessage}
                disabled={!targetIp || !msgToSend.trim()}
                className="w-12 h-12 bg-primary hover:bg-primary-hover text-[#121212] rounded-xl flex items-center justify-center transition-colors disabled:opacity-50 shrink-0"
              >
                <Send className="w-5 h-5" />
              </button>
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}

export default App;
