import { create } from 'zustand';

type UIStore = {
  showModelOverlay: boolean;
  selectedMode: 'heuristic' | 'classic' | 'ml' | 'watch' | null;
  aiSourceP1: 'heuristic' | 'client' | 'ml' | null;
  aiSourceP2: 'heuristic' | 'client' | 'ml';
  soundEnabled: boolean;
  diagnosticsPanelOpen: boolean;
  howToPlayOpen: boolean;
  errorModal: {
    isOpen: boolean;
    error: string;
  };
  actions: {
    setShowModelOverlay: (show: boolean) => void;
    setSelectedMode: (mode: 'heuristic' | 'classic' | 'ml' | 'watch' | null) => void;
    setAiSourceP1: (source: 'heuristic' | 'client' | 'ml' | null) => void;
    setAiSourceP2: (source: 'heuristic' | 'client' | 'ml') => void;
    setSoundEnabled: (enabled: boolean) => void;
    setDiagnosticsPanelOpen: (open: boolean) => void;
    setHowToPlayOpen: (open: boolean) => void;
    showError: (error: string) => void;
    hideError: () => void;
    reset: () => void;
  };
};

export const useUIStore = create<UIStore>(set => ({
  showModelOverlay: true,
  selectedMode: null,
  aiSourceP1: null,
  aiSourceP2: 'ml',
  soundEnabled: true,
  diagnosticsPanelOpen: false,
  howToPlayOpen: false,
  errorModal: {
    isOpen: false,
    error: '',
  },
  actions: {
    setShowModelOverlay: show => set({ showModelOverlay: show }),
    setSelectedMode: mode => set({ selectedMode: mode }),
    setAiSourceP1: source => set({ aiSourceP1: source }),
    setAiSourceP2: source => set({ aiSourceP2: source }),
    setSoundEnabled: enabled => set({ soundEnabled: enabled }),
    setDiagnosticsPanelOpen: open => set({ diagnosticsPanelOpen: open }),
    setHowToPlayOpen: open => set({ howToPlayOpen: open }),
    showError: error => set({ errorModal: { isOpen: true, error } }),
    hideError: () => set({ errorModal: { isOpen: false, error: '' } }),
    reset: () =>
      set({
        showModelOverlay: true,
        selectedMode: null,
        aiSourceP1: null,
        aiSourceP2: 'ml',
        soundEnabled: true,
        diagnosticsPanelOpen: false,
        howToPlayOpen: false,
        errorModal: { isOpen: false, error: '' },
      }),
  },
}));

export const useUIState = () =>
  useUIStore(state => ({
    showModelOverlay: state.showModelOverlay,
    selectedMode: state.selectedMode,
    aiSourceP1: state.aiSourceP1,
    aiSourceP2: state.aiSourceP2,
    soundEnabled: state.soundEnabled,
    diagnosticsPanelOpen: state.diagnosticsPanelOpen,
    howToPlayOpen: state.howToPlayOpen,
    errorModal: state.errorModal,
  }));
