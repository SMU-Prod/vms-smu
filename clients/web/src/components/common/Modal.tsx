import { Component, JSX, Show } from 'solid-js';
import { X } from 'lucide-solid';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  children: JSX.Element;
}

export const Modal: Component<ModalProps> = (props) => {
  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-lg',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
    full: 'max-w-[90vw] max-h-[90vh]',
  };

  const handleBackdropClick = (e: MouseEvent) => {
    if (e.target === e.currentTarget) {
      props.onClose();
    }
  };

  return (
    <Show when={props.isOpen}>
      <div 
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm animate-fadeIn"
        onClick={handleBackdropClick}
      >
        <div 
          class={`
            ${sizeClasses[props.size || 'md']} 
            w-full mx-4 bg-slate-800 border border-slate-700 rounded-xl shadow-2xl animate-slideUp
          `}
        >
          {/* Header */}
          <Show when={props.title}>
            <div class="flex items-center justify-between px-6 py-4 border-b border-slate-700">
              <h2 class="text-lg font-semibold text-white">{props.title}</h2>
              <button
                class="p-1 text-slate-400 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
                onClick={props.onClose}
              >
                <X class="w-5 h-5" />
              </button>
            </div>
          </Show>

          {/* Content */}
          <div class="p-6">
            {props.children}
          </div>
        </div>
      </div>
    </Show>
  );
};

// Add animation keyframes to index.css
// @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
// @keyframes slideUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
