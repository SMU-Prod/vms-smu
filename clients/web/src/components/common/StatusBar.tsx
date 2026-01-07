import { Component } from 'solid-js';
import { HardDrive, Cpu, MemoryStick, Monitor, Activity } from 'lucide-solid';
import { useConfig } from '../../stores';

const formatBytes = (bytes: number): string => {
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  let unitIndex = 0;
  let value = bytes;
  
  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex++;
  }
  
  return `${value.toFixed(1)} ${units[unitIndex]}`;
};

export const StatusBar: Component = () => {
  const { state } = useConfig();
  const status = () => state().systemStatus;

  const storagePercent = () => 
    Math.round((status().storageUsed / status().storageTotal) * 100);

  return (
    <footer class="h-8 bg-slate-900 border-t border-slate-700 flex items-center justify-between px-4 text-xs glass">
      {/* Left - Camera & Event Status */}
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-2">
          <Activity class="w-3.5 h-3.5 text-green-400" />
          <span class="text-slate-400">
            <span class="text-green-400 font-medium">{status().camerasOnline}</span>
            /{status().camerasTotal} câmeras online
          </span>
        </div>
        
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 bg-yellow-400 rounded-full animate-pulse"></span>
          <span class="text-slate-400">
            <span class="text-yellow-400 font-medium">{status().eventsUnacknowledged}</span> eventos pendentes
          </span>
        </div>
      </div>

      {/* Right - System Resources */}
      <div class="flex items-center gap-6">
        {/* Storage */}
        <div class="flex items-center gap-2">
          <HardDrive class="w-3.5 h-3.5 text-slate-400" />
          <div class="flex items-center gap-1">
            <span class="text-slate-400">Storage:</span>
            <span class={`font-medium ${storagePercent() > 85 ? 'text-red-400' : 'text-slate-300'}`}>
              {formatBytes(status().storageUsed)}
            </span>
            <span class="text-slate-500">/</span>
            <span class="text-slate-400">{formatBytes(status().storageTotal)}</span>
            <span class={`text-xs ${storagePercent() > 85 ? 'text-red-400' : 'text-slate-500'}`}>
              ({storagePercent()}%)
            </span>
          </div>
        </div>

        {/* CPU */}
        <div class="flex items-center gap-2">
          <Cpu class="w-3.5 h-3.5 text-slate-400" />
          <span class={`font-medium ${status().cpuUsage > 80 ? 'text-red-400' : 'text-slate-300'}`}>
            {status().cpuUsage}%
          </span>
        </div>

        {/* Memory */}
        <div class="flex items-center gap-2">
          <MemoryStick class="w-3.5 h-3.5 text-slate-400" />
          <span class={`font-medium ${status().memoryUsage > 85 ? 'text-red-400' : 'text-slate-300'}`}>
            {status().memoryUsage}%
          </span>
        </div>

        {/* GPU */}
        {status().gpuUsage !== undefined && (
          <div class="flex items-center gap-2">
            <Monitor class="w-3.5 h-3.5 text-slate-400" />
            <span class={`font-medium ${status().gpuUsage! > 90 ? 'text-yellow-400' : 'text-slate-300'}`}>
              {status().gpuUsage}%
            </span>
          </div>
        )}

        {/* Time */}
        <div class="text-slate-500 pl-4 border-l border-slate-700">
          {new Date().toLocaleTimeString('pt-BR', { hour: '2-digit', minute: '2-digit' })}
        </div>
      </div>
    </footer>
  );
};
