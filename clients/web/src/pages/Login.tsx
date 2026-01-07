import { Component, createSignal, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Shield, Eye, EyeOff, Loader2 } from 'lucide-solid';
import { useAuth } from '../stores';

const Login: Component = () => {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [showPassword, setShowPassword] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');
  
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      const success = await login(username(), password());
      if (success) {
        navigate('/live');
      } else {
        setError('Usuário ou senha inválidos');
      }
    } catch {
      setError('Erro ao conectar ao servidor');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex items-center justify-center p-4">
      {/* Background Pattern */}
      <div class="absolute inset-0 opacity-20">
        <div class="absolute inset-0" style={{
          "background-image": `url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%233b82f6' fill-opacity='0.1'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E")`,
        }}></div>
      </div>

      {/* Login Card */}
      <div class="relative w-full max-w-md">
        {/* Glow Effect */}
        <div class="absolute -inset-1 bg-gradient-to-r from-vms-primary via-vms-accent to-vms-primary opacity-20 blur-xl rounded-2xl"></div>
        
        <div class="relative bg-slate-800/90 border border-slate-700 rounded-2xl p-8 backdrop-blur-xl shadow-2xl">
          {/* Logo */}
          <div class="flex flex-col items-center mb-8">
            <div class="w-16 h-16 bg-gradient-to-br from-vms-primary to-vms-accent rounded-2xl flex items-center justify-center mb-4 shadow-lg shadow-vms-primary/30">
              <Shield class="w-9 h-9 text-white" />
            </div>
            <h1 class="text-2xl font-bold bg-gradient-to-r from-white to-slate-300 bg-clip-text text-transparent">
              VMS Enterprise
            </h1>
            <p class="text-slate-400 text-sm mt-1">Video Management System</p>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit} class="space-y-5">
            <div>
              <label class="block text-sm font-medium text-slate-300 mb-2">
                Usuário
              </label>
              <input
                type="text"
                value={username()}
                onInput={(e) => setUsername(e.currentTarget.value)}
                class="input"
                placeholder="Digite seu usuário"
                required
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-slate-300 mb-2">
                Senha
              </label>
              <div class="relative">
                <input
                  type={showPassword() ? 'text' : 'password'}
                  value={password()}
                  onInput={(e) => setPassword(e.currentTarget.value)}
                  class="input pr-10"
                  placeholder="Digite sua senha"
                  required
                />
                <button
                  type="button"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-white transition-colors"
                  onClick={() => setShowPassword(!showPassword())}
                >
                  <Show when={showPassword()} fallback={<EyeOff class="w-4 h-4" />}>
                    <Eye class="w-4 h-4" />
                  </Show>
                </button>
              </div>
            </div>

            <Show when={error()}>
              <div class="p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm">
                {error()}
              </div>
            </Show>

            <button
              type="submit"
              disabled={loading()}
              class="w-full btn bg-gradient-to-r from-vms-primary to-vms-secondary hover:from-vms-secondary hover:to-vms-primary text-white font-semibold py-3 flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300"
            >
              <Show when={loading()} fallback="Entrar">
                <Loader2 class="w-5 h-5 animate-spin" />
                Entrando...
              </Show>
            </button>
          </form>

          {/* Demo Credentials */}
          <div class="mt-6 p-3 bg-slate-700/50 rounded-lg border border-slate-600">
            <p class="text-xs text-slate-400 text-center">
              <span class="text-slate-300 font-medium">Demo:</span> admin / admin
            </p>
          </div>

          {/* Footer */}
          <div class="mt-6 text-center text-xs text-slate-500">
            <p>© 2025 VMS Enterprise. Todos os direitos reservados.</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Login;
