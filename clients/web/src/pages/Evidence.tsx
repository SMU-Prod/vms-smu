import { Component, For, Show, createSignal } from 'solid-js';
import { Plus, Search, Download, Archive, FileText, Clock, Paperclip, X, ChevronRight } from 'lucide-solid';
import type { Evidence, EvidenceStatus } from '../types';

const mockEvidence: Evidence[] = [
  { id: '1', caseNumber: 'EVD-20251213-0042', title: 'Invasão noturna no estacionamento', description: 'Pessoa não autorizada...', status: 'active', priority: 'high', createdBy: 'Admin', createdAt: new Date(), updatedAt: new Date(), attachments: [], custodyChain: [], relatedEvents: [], tags: [] },
  { id: '2', caseNumber: 'EVD-20251212-0038', title: 'Placa suspeita detectada', description: 'Veículo com placa...', status: 'exported', priority: 'medium', createdBy: 'Admin', createdAt: new Date(Date.now() - 86400000), updatedAt: new Date(), attachments: [], custodyChain: [], relatedEvents: [], tags: [] },
  { id: '3', caseNumber: 'EVD-20251211-0035', title: 'Pessoa não autorizada', description: 'Tentativa de acesso...', status: 'archived', priority: 'low', createdBy: 'Admin', createdAt: new Date(Date.now() - 172800000), updatedAt: new Date(), attachments: [], custodyChain: [], relatedEvents: [], tags: [] },
];

const statusConfig: Record<EvidenceStatus, { color: string; label: string }> = {
  active: { color: 'green', label: 'Ativo' },
  exported: { color: 'blue', label: 'Exportado' },
  archived: { color: 'slate', label: 'Arquivado' },
  deleted: { color: 'red', label: 'Excluído' },
};

const EvidencePage: Component = () => {
  const [selectedEvidence, setSelectedEvidence] = createSignal<Evidence | null>(null);
  const [searchQuery, setSearchQuery] = createSignal('');

  const filteredEvidence = () => mockEvidence.filter(e => 
    e.title.toLowerCase().includes(searchQuery().toLowerCase()) ||
    e.caseNumber.toLowerCase().includes(searchQuery().toLowerCase())
  );

  return (
    <div class="h-full flex">
      <div class="flex-1 flex flex-col">
        <div class="flex items-center justify-between p-4 bg-slate-800/50 border-b border-slate-700">
          <div class="flex items-center gap-3">
            <FileText class="w-5 h-5 text-vms-accent" />
            <h1 class="text-lg font-semibold text-white">Ocorrências</h1>
          </div>
          <div class="flex items-center gap-3">
            <div class="relative">
              <Search class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-400" />
              <input type="text" placeholder="Buscar..." value={searchQuery()} onInput={(e) => setSearchQuery(e.currentTarget.value)} class="input pl-9 w-64" />
            </div>
            <button class="btn btn-primary flex items-center gap-2"><Plus class="w-4 h-4" /> Nova Ocorrência</button>
          </div>
        </div>

        <div class="flex-1 overflow-auto p-4">
          <div class="space-y-3">
            <For each={filteredEvidence()}>
              {(evidence) => (
                <div class={`card cursor-pointer hover:border-vms-accent/50 transition-colors ${selectedEvidence()?.id === evidence.id ? 'border-vms-accent' : ''}`} onClick={() => setSelectedEvidence(evidence)}>
                  <div class="flex items-start justify-between">
                    <div class="flex-1">
                      <div class="flex items-center gap-2 mb-1">
                        <span class="text-xs text-slate-500 font-mono">{evidence.caseNumber}</span>
                        <span class={`px-2 py-0.5 rounded-full text-xs bg-${statusConfig[evidence.status].color}-500/20 text-${statusConfig[evidence.status].color}-400`}>{statusConfig[evidence.status].label}</span>
                      </div>
                      <h3 class="text-white font-medium">{evidence.title}</h3>
                      <p class="text-slate-400 text-sm mt-1 line-clamp-2">{evidence.description}</p>
                      <div class="flex items-center gap-4 mt-2 text-xs text-slate-500">
                        <span class="flex items-center gap-1"><Clock class="w-3 h-3" />{evidence.createdAt.toLocaleDateString('pt-BR')}</span>
                        <span class="flex items-center gap-1"><Paperclip class="w-3 h-3" />{evidence.attachments.length} anexos</span>
                      </div>
                    </div>
                    <ChevronRight class="w-5 h-5 text-slate-600" />
                  </div>
                </div>
              )}
            </For>
          </div>
        </div>
      </div>

      <Show when={selectedEvidence()}>
        {(evidence) => (
          <div class="w-96 border-l border-slate-700 bg-slate-800/50 flex flex-col">
            <div class="flex items-center justify-between p-4 border-b border-slate-700">
              <h2 class="font-semibold text-white">Detalhes</h2>
              <button class="p-1 text-slate-400 hover:text-white rounded" onClick={() => setSelectedEvidence(null)}><X class="w-5 h-5" /></button>
            </div>
            <div class="flex-1 overflow-auto p-4 space-y-4">
              <div><label class="text-xs text-slate-500">Caso</label><p class="text-white font-mono">{evidence().caseNumber}</p></div>
              <div><label class="text-xs text-slate-500">Título</label><p class="text-white">{evidence().title}</p></div>
              <div><label class="text-xs text-slate-500">Status</label><p class={`text-${statusConfig[evidence().status].color}-400`}>{statusConfig[evidence().status].label}</p></div>
              <div><label class="text-xs text-slate-500">Descrição</label><p class="text-slate-300 text-sm">{evidence().description}</p></div>
              <div><label class="text-xs text-slate-500">Criado por</label><p class="text-white">{evidence().createdBy} - {evidence().createdAt.toLocaleString('pt-BR')}</p></div>
              <div><label class="text-xs text-slate-500 mb-2 block">Cadeia de Custódia</label><div class="bg-slate-900 rounded-lg p-3 text-sm text-slate-400">Nenhuma ação registrada</div></div>
            </div>
            <div class="p-4 border-t border-slate-700 space-y-2">
              <button class="w-full btn btn-primary text-sm"><Download class="w-4 h-4 mr-2" />Exportar ZIP</button>
              <button class="w-full btn btn-secondary text-sm"><Archive class="w-4 h-4 mr-2" />Arquivar</button>
            </div>
          </div>
        )}
      </Show>
    </div>
  );
};

export default EvidencePage;
