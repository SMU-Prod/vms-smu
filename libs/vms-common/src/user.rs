//! Sistema de Usuários e Permissões
//!
//! Autenticação, autorização granular, grupos e auditoria.

use crate::types::CameraId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// ID de usuário
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ID de grupo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupId(pub Uuid);

impl GroupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for GroupId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID de role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleId(pub Uuid);

impl RoleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RoleId {
    fn default() -> Self {
        Self::new()
    }
}

/// Status do usuário
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Locked,
    PendingActivation,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// Provedor de autenticação
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthProvider {
    /// Autenticação local
    Local,
    /// Active Directory / LDAP
    ActiveDirectory { domain: String },
    /// SAML SSO
    SAML { provider_id: String },
    /// OAuth2 / OIDC
    OAuth2 { provider: String },
}

impl Default for AuthProvider {
    fn default() -> Self {
        Self::Local
    }
}

/// Configuração de MFA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFAConfig {
    /// MFA habilitado
    pub enabled: bool,

    /// Tipo de MFA
    pub mfa_type: MFAType,

    /// Secret para TOTP (criptografado)
    pub totp_secret: Option<String>,

    /// Telefone para SMS
    pub phone_number: Option<String>,

    /// Email para código
    pub email: Option<String>,

    /// Chaves de backup usadas
    pub backup_codes_used: HashSet<String>,
}

impl Default for MFAConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mfa_type: MFAType::TOTP,
            totp_secret: None,
            phone_number: None,
            email: None,
            backup_codes_used: HashSet::new(),
        }
    }
}

/// Tipo de MFA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MFAType {
    /// TOTP (Google Authenticator, etc)
    TOTP,
    /// SMS
    SMS,
    /// Email
    Email,
    /// WebAuthn (chave de segurança)
    WebAuthn,
}

impl Default for MFAType {
    fn default() -> Self {
        Self::TOTP
    }
}

/// Usuário do sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// ID do usuário
    pub id: UserId,

    /// Username (login)
    pub username: String,

    /// Email
    pub email: Option<String>,

    /// Nome completo
    pub full_name: String,

    /// Hash da senha (Argon2)
    pub password_hash: Option<String>,

    /// Status
    pub status: UserStatus,

    /// Provedor de autenticação
    pub auth_provider: AuthProvider,

    /// Configuração MFA
    pub mfa: MFAConfig,

    /// Grupos
    pub groups: Vec<GroupId>,

    /// Role diretas (além das de grupo)
    pub roles: Vec<RoleId>,

    /// Fuso horário
    pub timezone: String,

    /// Idioma
    pub language: String,

    /// Último login
    pub last_login: Option<DateTime<Utc>>,

    /// Último IP
    pub last_ip: Option<String>,

    /// Tentativas de login falhas
    pub failed_login_attempts: u32,

    /// Bloqueado até
    pub locked_until: Option<DateTime<Utc>>,

    /// Senha expira em
    pub password_expires_at: Option<DateTime<Utc>>,

    /// Deve trocar senha
    pub must_change_password: bool,

    /// Criado em
    pub created_at: DateTime<Utc>,

    /// Atualizado em
    pub updated_at: DateTime<Utc>,

    /// Criado por
    pub created_by: Option<UserId>,

    /// Avatar URL
    pub avatar_url: Option<String>,

    /// Metadados extras
    pub metadata: HashMap<String, String>,
}

impl User {
    pub fn new(username: &str, full_name: &str) -> Self {
        Self {
            id: UserId::new(),
            username: username.to_string(),
            email: None,
            full_name: full_name.to_string(),
            password_hash: None,
            status: UserStatus::PendingActivation,
            auth_provider: AuthProvider::Local,
            mfa: MFAConfig::default(),
            groups: Vec::new(),
            roles: Vec::new(),
            timezone: "America/Sao_Paulo".to_string(),
            language: "pt-BR".to_string(),
            last_login: None,
            last_ip: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_expires_at: None,
            must_change_password: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            avatar_url: None,
            metadata: HashMap::new(),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.status == UserStatus::Locked ||
            self.locked_until.map_or(false, |until| Utc::now() < until)
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active && !self.is_locked()
    }

    pub fn increment_failed_login(&mut self) {
        self.failed_login_attempts += 1;
        // Bloquear após 5 tentativas
        if self.failed_login_attempts >= 5 {
            self.locked_until = Some(Utc::now() + chrono::Duration::minutes(15));
        }
    }

    pub fn reset_failed_logins(&mut self) {
        self.failed_login_attempts = 0;
        self.locked_until = None;
    }
}

/// Grupo de usuários
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// ID do grupo
    pub id: GroupId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Roles do grupo
    pub roles: Vec<RoleId>,

    /// É grupo do Active Directory
    pub is_external: bool,

    /// DN do AD (se externo)
    pub external_dn: Option<String>,

    /// Criado em
    pub created_at: DateTime<Utc>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Self {
            id: GroupId::new(),
            name: name.to_string(),
            description: None,
            roles: Vec::new(),
            is_external: false,
            external_dn: None,
            created_at: Utc::now(),
        }
    }
}

/// Recurso que pode ter permissões
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    /// Sistema geral
    System,
    /// Câmera específica
    Camera(CameraId),
    /// Todas as câmeras
    AllCameras,
    /// Grupo de câmeras
    CameraGroup(String),
    /// PTZ de câmera
    PTZ(CameraId),
    /// Playback
    Playback,
    /// Exportação
    Export,
    /// Analytics
    Analytics,
    /// LPR
    LPR,
    /// Eventos
    Events,
    /// Mapas
    Maps,
    /// Usuários
    Users,
    /// Configurações
    Settings,
    /// Logs
    Logs,
    /// API
    API,
    /// Mobile
    Mobile,
}

/// Ação que pode ser executada
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    /// Visualizar
    View,
    /// Criar
    Create,
    /// Editar
    Edit,
    /// Excluir
    Delete,
    /// Executar
    Execute,
    /// Administrar
    Admin,
    /// Controlar (PTZ)
    Control,
    /// Exportar
    Export,
    /// Reconhecer (eventos)
    Acknowledge,
}

/// Permissão específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Recurso
    pub resource: Resource,

    /// Ações permitidas
    pub actions: HashSet<Action>,

    /// Condições (opcional)
    pub conditions: Option<PermissionConditions>,
}

impl Permission {
    pub fn new(resource: Resource, actions: Vec<Action>) -> Self {
        Self {
            resource,
            actions: actions.into_iter().collect(),
            conditions: None,
        }
    }

    pub fn full_access(resource: Resource) -> Self {
        Self {
            resource,
            actions: vec![
                Action::View,
                Action::Create,
                Action::Edit,
                Action::Delete,
                Action::Execute,
                Action::Admin,
                Action::Control,
                Action::Export,
                Action::Acknowledge,
            ].into_iter().collect(),
            conditions: None,
        }
    }

    pub fn view_only(resource: Resource) -> Self {
        Self::new(resource, vec![Action::View])
    }

    pub fn has_action(&self, action: Action) -> bool {
        self.actions.contains(&action)
    }
}

/// Condições para permissão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConditions {
    /// IPs permitidos
    pub allowed_ips: Option<Vec<String>>,

    /// Horários permitidos (formato HH:MM-HH:MM)
    pub allowed_times: Option<Vec<String>>,

    /// Dias da semana (0=domingo)
    pub allowed_days: Option<Vec<u8>>,

    /// Tempo máximo de sessão (segundos)
    pub max_session_duration: Option<u32>,
}

/// Role (conjunto de permissões)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// ID da role
    pub id: RoleId,

    /// Nome
    pub name: String,

    /// Descrição
    pub description: Option<String>,

    /// Permissões
    pub permissions: Vec<Permission>,

    /// É role de sistema (não pode ser editada)
    pub is_system: bool,

    /// Role pai (herança)
    pub parent_role: Option<RoleId>,

    /// Criado em
    pub created_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: &str) -> Self {
        Self {
            id: RoleId::new(),
            name: name.to_string(),
            description: None,
            permissions: Vec::new(),
            is_system: false,
            parent_role: None,
            created_at: Utc::now(),
        }
    }

    /// Role de administrador
    pub fn admin() -> Self {
        Self {
            id: RoleId::new(),
            name: "Administrator".to_string(),
            description: Some("Full system access".to_string()),
            permissions: vec![Permission::full_access(Resource::System)],
            is_system: true,
            parent_role: None,
            created_at: Utc::now(),
        }
    }

    /// Role de operador
    pub fn operator() -> Self {
        Self {
            id: RoleId::new(),
            name: "Operator".to_string(),
            description: Some("View cameras, playback, acknowledge events".to_string()),
            permissions: vec![
                Permission::new(Resource::AllCameras, vec![Action::View]),
                Permission::new(Resource::Playback, vec![Action::View, Action::Export]),
                Permission::new(Resource::Events, vec![Action::View, Action::Acknowledge]),
                Permission::new(Resource::Maps, vec![Action::View]),
            ],
            is_system: true,
            parent_role: None,
            created_at: Utc::now(),
        }
    }

    /// Role de visualizador
    pub fn viewer() -> Self {
        Self {
            id: RoleId::new(),
            name: "Viewer".to_string(),
            description: Some("View live cameras only".to_string()),
            permissions: vec![
                Permission::view_only(Resource::AllCameras),
            ],
            is_system: true,
            parent_role: None,
            created_at: Utc::now(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.push(permission);
    }

    pub fn has_permission(&self, resource: &Resource, action: Action) -> bool {
        self.permissions.iter().any(|p| {
            (p.resource == *resource || p.resource == Resource::System) && p.has_action(action)
        })
    }
}

/// Sessão de usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// ID da sessão
    pub id: Uuid,

    /// Usuário
    pub user_id: UserId,

    /// Token de acesso
    pub access_token: String,

    /// Token de refresh
    pub refresh_token: String,

    /// Expira em
    pub expires_at: DateTime<Utc>,

    /// Refresh expira em
    pub refresh_expires_at: DateTime<Utc>,

    /// IP do cliente
    pub client_ip: String,

    /// User-Agent
    pub user_agent: String,

    /// Tipo de cliente
    pub client_type: ClientType,

    /// Criada em
    pub created_at: DateTime<Utc>,

    /// Última atividade
    pub last_activity: DateTime<Utc>,

    /// MFA verificado
    pub mfa_verified: bool,
}

/// Tipo de cliente
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientType {
    Web,
    Desktop,
    Mobile,
    API,
}

/// Log de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// ID do log
    pub id: Uuid,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Usuário que executou a ação
    pub user_id: UserId,

    /// Username
    pub username: String,

    /// IP do cliente
    pub client_ip: String,

    /// Ação executada
    pub action: String,

    /// Recurso afetado
    pub resource: String,

    /// ID do recurso (se aplicável)
    pub resource_id: Option<String>,

    /// Resultado
    pub result: AuditResult,

    /// Detalhes (JSON)
    pub details: Option<String>,

    /// Sessão
    pub session_id: Option<Uuid>,
}

/// Resultado de ação auditada
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Denied,
}

impl AuditLog {
    pub fn new(
        user_id: UserId,
        username: &str,
        client_ip: &str,
        action: &str,
        resource: &str,
        result: AuditResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id,
            username: username.to_string(),
            client_ip: client_ip.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            resource_id: None,
            result,
            details: None,
            session_id: None,
        }
    }
}

/// Política de senha
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Tamanho mínimo
    pub min_length: u8,

    /// Requer maiúscula
    pub require_uppercase: bool,

    /// Requer minúscula
    pub require_lowercase: bool,

    /// Requer número
    pub require_number: bool,

    /// Requer caractere especial
    pub require_special: bool,

    /// Dias para expiração (0 = nunca)
    pub expiration_days: u32,

    /// Histórico de senhas (não repetir)
    pub password_history: u8,

    /// Máximo de tentativas antes de bloquear
    pub max_failed_attempts: u8,

    /// Tempo de bloqueio (minutos)
    pub lockout_duration_minutes: u32,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_number: true,
            require_special: false,
            expiration_days: 90,
            password_history: 5,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
        }
    }
}

impl PasswordPolicy {
    /// Política forte
    pub fn strong() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_number: true,
            require_special: true,
            expiration_days: 60,
            password_history: 10,
            max_failed_attempts: 3,
            lockout_duration_minutes: 30,
        }
    }

    /// Valida senha
    pub fn validate(&self, password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if password.len() < self.min_length as usize {
            errors.push(format!("Password must be at least {} characters", self.min_length));
        }

        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }

        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }

        if self.require_number && !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain at least one number".to_string());
        }

        if self.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("Password must contain at least one special character".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("admin", "System Administrator");
        assert_eq!(user.username, "admin");
        assert_eq!(user.status, UserStatus::PendingActivation);
        assert!(user.must_change_password);
    }

    #[test]
    fn test_user_lock() {
        let mut user = User::new("test", "Test User");
        user.status = UserStatus::Active;

        assert!(!user.is_locked());

        for _ in 0..5 {
            user.increment_failed_login();
        }

        assert!(user.is_locked());

        user.reset_failed_logins();
        assert!(!user.is_locked());
    }

    #[test]
    fn test_role_permissions() {
        let admin = Role::admin();
        assert!(admin.has_permission(&Resource::System, Action::Admin));
        assert!(admin.has_permission(&Resource::AllCameras, Action::View));

        let viewer = Role::viewer();
        assert!(viewer.has_permission(&Resource::AllCameras, Action::View));
        assert!(!viewer.has_permission(&Resource::AllCameras, Action::Edit));
    }

    #[test]
    fn test_password_policy() {
        let policy = PasswordPolicy::default();

        assert!(policy.validate("ValidPass123").is_ok());
        assert!(policy.validate("short").is_err());
        assert!(policy.validate("nouppercase123").is_err());
    }

    #[test]
    fn test_permission() {
        let perm = Permission::full_access(Resource::AllCameras);
        assert!(perm.has_action(Action::View));
        assert!(perm.has_action(Action::Delete));
        assert!(perm.has_action(Action::Control));
    }
}
